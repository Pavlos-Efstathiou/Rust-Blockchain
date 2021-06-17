use std::env;

mod rust_blockchain;
use rust_blockchain::blockchain::Blockchain;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut test = Blockchain::new();
    test.init();

    if args.len() >= 2 {
        let amount: u32 = args[1].trim().parse().unwrap();

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
