## Rust Blockchain
[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![MIT licensed][license-image]
![Rust Version][rustc-image]

Blockchain crate for the Rust programming language.

It's API is still very unstable and should not be taken for granted. 

## Can you show me how to use this crate?

Here's an example to get started
```rust
use std::env;

use rs_blockchain::rust_blockchain::blockchain::Blockchain;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut test = Blockchain::new();

    println!("{}", rs_blockchain::VERSION_INFO);
    test.init();

    if args.len() >= 2 {
        let mut collected_args: String = String::new();

        for arg in args[..].iter() {
            collected_args.push_str(arg);
        }

        let amount: u32 = rs_blockchain::remove_non_digits(&collected_args);

        for i in 1..=amount {
            test.add_transaction("Jane Doe", "John Doe", i as f32);
            test.mine();
        }
    } else {
        for i in 1..=25 {
            test.add_transaction("Jane Doe", "John Doe", i as f32);
            test.mine();
        }
    }
}

```

## Can I contribute to this project?
Of course, anyone can!


[crate-image]: https://img.shields.io/crates/v/rs_blockchain.svg
[crate-link]: https://crates.io/crates/rs_blockchain.svg
[docs-image]: https://docs.rs/rs_blockchain/badge.svg
[docs-link]: https://docs.rs/rs_blockchain/
[license-image]: https://img.shields.io/badge/license-MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.4+-blue.svg