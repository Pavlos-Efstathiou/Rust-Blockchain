# Changelog
All noteworth changes to this project will be mentioned in this file.

## v0.2.0 Alma | 19/6/2021
### Additions
- A transaction struct which is used for transactions
- Implementations of ```std::fmt::display``` for ```Block``` and ```Transaction```
- Some minor changes

## v0.2.1 Alma | 20/6/2021
### Additions
- ```EmptyVecErr``` now implements Error and ```ErrResult```'s Error type is a ```Box<dyn std::error:Error>```
- Moved the code in ```mod.rs``` to ```lib.rs```
- Added a GitHub Workflow
- Minor changes

## v0.2.3 Alma | 1/7/2021
### Additions
- ```Block::init``` now instantiates a ```Blockchain``` struct and adds a Genesis Block to it
- Added the ```Blockchain::write_chain_to_file``` function which dumps every Block to a file
- Replaced the ```Blockchain::add_transaction``` function with the ```add_transaction!``` Macro
- A Blockchain's difficulty can be changed with the ```Blockchain::set_difficulty``` function.
- Removed the ```Transaction``` struct
- Transactions have been replaced with JSON in Strings
- Some minor changes

## v0.2.5 Alma | 2/7/2021
### Additions
- The ```regex``` crate isn't a dependency anymore
- Documented all public functions
- Added the ```Blockchain::list_transactions``` function
- Some minor changes

### Upcoming features
- Wallets
- Sending coins through a network
- Better Documentation