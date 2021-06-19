## rs-blockchain

Blockchain crate for the Rust programming language.

It's API is still very unstable and should not be taken for granted. 

## Can you show me how to use this crate?

Here's an example to get started
```rust
use std::env;

use rs_blockchain::rust_blockchain::blockchain::Blockchain;
use rs_blockchain::rust_blockchain::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut test = Blockchain::new();

    test.init();
    if args.len() >= 2 {
        let amount: u32 = remove_non_digits(&args[1]);

        for i in 1..=amount {
            test.add_transaction(format!("Transaction {}", i));
            test.mine();
        }
    } else {
        for i in 1..=25 {
            test.add_transaction(format!("Transaction {}", i));
            test.mine();
        }
    }
}
```

## Can I contribute to this project?
Of course, anyone can!