# Changelog
All noteworth changes to this project will be mentioned in this file.

## v0.2.0 Alma | 19/6/2021
### Changes
- A transaction struct which is used for transactions
- Implementations of ```std::fmt::display``` for ```Block``` and ```Transaction```
- Some minor changes

## v0.2.1 Alma | 20/6/2021
### Changes
- ```EmptyVecErr``` now implements Error and ```ErrResult```'s Error type is a ```Box<dyn std::error:Error>```
- Moved the code in ```mod.rs``` to ```lib.rs```
- Added a GitHub Workflow
- Minor changes

## v0.2.3 Alma | 1/7/2021
### Changes
- ```Block::init``` now instantiates a ```Blockchain``` struct and adds a Genesis Block to it
- Added the ```Blockchain::write_chain_to_file``` function which dumps every Block to a file
- Replaced the ```Blockchain::add_transaction``` function with the ```add_transaction!``` Macro
- A Blockchain's difficulty can be changed with the ```Blockchain::set_difficulty``` function.
- Removed the ```Transaction``` struct
- Transactions have been replaced with JSON in Strings
- Some minor changes

## v0.2.5 Alma | 2/7/2021
### Changes
- The ```regex``` crate isn't a dependency anymore
- Documented all public functions
- Added the ```Blockchain::list_transactions``` function
- Some minor changes

## v0.2.6 Alma | 5/7/2021
### Changes
- ```chrono``` isn't a dependency anymore
- Replaced ```chrono``` with ```std::time::{SystemTime, UNIX_EPOCH}```
- ```VERSION_INFO``` is static now
- JSON is pretty printed now
- Minor changes

## v0.2.7 Alma | 7/7/2021
### Changes
- Added ```ArgValue```; a type alias of ```Option<u32>``` which implements the ```arg_unwrap``` method
- Made ```rs_blockchain::remove_non_digits``` return an ```ArgValue<u32>```

### Upcoming features
- Better Documentation
- Multithreading
- Unit Tests