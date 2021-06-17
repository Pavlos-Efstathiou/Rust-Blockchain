use std::fmt;

#[derive(Debug, Clone)]
pub struct EmptyVecErr;

impl fmt::Display for EmptyVecErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No transactions have happened!")
    }
}

// fn double_first(vec: Vec<&str>) -> Result<u32> {
//     vec.first()
//         // Change the error to our new type.
//         .ok_or(EmptyVecErr)
//         .and_then(|s| {
//             // Place code here
//             .map_err(|_| EmptyVecErr)
//         })
// }

// fn print(result: Result<u32>) {
//     match result {
//         Ok(n) => println!("Mined a block with index {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }
