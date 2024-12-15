use std::fs::File;
use std::io::BufReader;
use std::{env, io::Write};
use tracing::{error, info};

use transaction::{transaction_reader, Transaction};

mod account;
mod client;
mod error;
mod transaction;
mod transaction_record;
mod trial_balance;

#[cfg(feature = "logging")]
fn init_logging() {
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    let target = tracing_subscriber::filter::Targets::new().with_default(LevelFilter::DEBUG);

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_line_number(true)
        .with_level(true);

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(target)
        .init();
}

fn main() {
    #[cfg(feature = "logging")]
    init_logging();

    info!("Starting the program");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <transactions_file>", args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1]).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut trial_balance = trial_balance::TrialBalance::new();

    let transaction_reader = transaction_reader(reader);
    for tx_row in transaction_reader {
        match tx_row {
            Ok(tx_row) => {
                let tx: Result<Transaction, _> = tx_row.try_into();
                match tx {
                    Ok(tx) => {
                        info!("Handling transaction {:?}", tx);
                        let err = trial_balance.handle_transaction(tx);
                        if let Err(err) = err {
                            error!("Could not handle transaction {:?}", err);
                        }
                    }
                    Err(err) => error!("Could not parse transaction {:?}", err),
                }
            }
            Err(err) => error!("Could not parse transaction {:?}", err),
        }
    }
    let stdout = std::io::stdout();
    let mut locked_stdout = stdout.lock();
    if let Err(err) = trial_balance.to_csv(&mut locked_stdout) {
        error!("Could not write to stdout {:?}", err);
    }
    locked_stdout.flush().unwrap();
}
