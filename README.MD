# rs-blockchain

### WIP Blockchain package for the Rust programming language.

### It's API is still very unstable and should not be taken for granted. The documentation is not that good.

# Can you show me how to use this crate?

### Here's an example
```rust
use std::env;
use rust_blockchain::blockchain::Blockchain;

fn main() {
    // Collects all command line arguments into a Vector
    let args: Vec<String> = env::args().collect();
    // New Blockchain struct
    let mut chain = Blockchain::new();
    // Initialize the chain
    chain.init();

    if args.len() >= 2 {
        // Convert argument one to an unsigned 32 bit integer
        let amount: u32 = args[1].trim().parse().unwrap();

        for i in 1..=amount {
            // Add a transaction
            test.add_transaction(format!("Transaction {}", i));
            // Mine blocks
            test.mine();
        }
    } else {
        for i in 1..=25 {
            // Add a transaction
            test.add_transaction(format!("Transaction {}", i));
            // Mine blocks
            test.mine();
        }
    }
}

```

# Can I contribute to this project?

### Of course, anyone can!