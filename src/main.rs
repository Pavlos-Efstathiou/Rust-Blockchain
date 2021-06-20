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
