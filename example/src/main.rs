use std::env;

use rs_blockchain::add_transaction;
use rs_blockchain::rust_blockchain::blockchain::Blockchain;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut new_blockchain = Blockchain::init();
    let mut collected_args: String = String::new();

    for arg in args[..].iter() {
        collected_args.push_str(arg);
    }

    let amount = rs_blockchain::remove_non_digits(&collected_args);

    new_blockchain.set_difficulty(4);
    println!("{}", rs_blockchain::VERSION_INFO);

    if amount != 0 {
        for i in 1..=amount {
            add_transaction!(
                new_blockchain, "Jane Doe" => "John Doe", i as f32,
                new_blockchain, "John Doe" => "Jane Doe", 1 << i,
            );
            new_blockchain.mine();
        }
    } else {
        for i in 1..=25 {
            add_transaction!(
                new_blockchain, "Jane Doe" => "John Doe", i as f32,
                new_blockchain, "John Doe" => "Jane Doe", 1 << i,
            );
            new_blockchain.mine();
        }
    }

    new_blockchain.list_transactions();
    new_blockchain.write_chain_to_file()?;
    Ok(())
}
