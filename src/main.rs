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
