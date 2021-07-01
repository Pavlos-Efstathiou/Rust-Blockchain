use std::env;

use rs_blockchain::add_transaction;
use rs_blockchain::rust_blockchain::blockchain::Blockchain;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut new_blockchain = Blockchain::init();

    println!("{}", rs_blockchain::VERSION_INFO);

    if args.len() >= 2 {
        let mut collected_args: String = String::new();

        for arg in args[..].iter() {
            collected_args.push_str(arg);
        }

        let amount = rs_blockchain::remove_non_digits(&collected_args);

        match amount {
            Ok(o) => {
                for i in 1..=o {
                    add_transaction!(new_blockchain, "Jane Doe" => "John Doe", i as f32);
                    new_blockchain.mine();
                }
            }
            Err(_) => eprintln!("First argument does not contain a number"),
        }
    } else {
        for i in 1..=25 {
            add_transaction!(new_blockchain, "Jane Doe" => "John Doe", i as f32);
            new_blockchain.mine();
        }
    };

    new_blockchain.write_chain_to_file()?;
    Ok(())
}
