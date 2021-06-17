use std::env;
mod blockchain_lib;
use blockchain_lib::blockchain::Blockchain;
fn main() {
    let args: Vec<String> = env::args().collect();
    let amount: u32;

    let mut test = Blockchain::new();
    test.init();

    if args.len() >= 2 {
        let u32_vec: Vec<u32> = args[1].chars().filter_map(|i| i.to_digit(10)).collect();
        amount = u32_vec
            .iter()
            .map(|x| x.to_string())
            .collect::<String>()
            .trim()
            .parse()
            .unwrap();

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
