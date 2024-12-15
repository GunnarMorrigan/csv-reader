# Introduction
My implementation of the example transaction engine

## Completeness
This solution covers the following transactions:
- Deposit
- Withdrawal
- Dispute
- Resolve
- Chargeback

Correctness is partially enforced by the type system. The transactions are grouped into `Transfer` and `Mutation` enums. This allows for dedicated functions that do not have repeated checks, and it enables easier branching in, for example, match statements. This approach limits the number of bugs one can make. However, it does not prevent all bugs. To guarantee a working solution, several test cases have been written.

## Safety and Robustness
No unsafe code is used. For error handling the typical `Result` enum is used with a custom error enum `TransactionError`. When an error is created a log entry is made. Logging can be enabled via a feature flag to easily debug the application.

## Efficiency

### Parsing
The entire file is not read in its entirety before processing; instead, parsing and processing happen row by row. For efficient reading, a buffered input stream is used.

### Data structure
There is not much information regarding the requirements of the system. The biggest data structure choice is the use of [`HashMap`]s that back the account storage and ledger storage. For very large amounts of transactions, they are the de facto standard with O(1) + C lookup times. However, the additional constant is rather large compared to array indexation. This overhead is acceptable as it makes development and handling of large data sets easier.

## Maintainability
To maintain maintainability, the following tactics have been applied:
- (Auto) Format using Rust's native formatter
- Wrap new types like Client and Transaction IDs in a new type so that they cannot be confused with other u16, u32 types
- Documentation
- Use of Cargo clippy to provide more linting
- A good number of test cases
