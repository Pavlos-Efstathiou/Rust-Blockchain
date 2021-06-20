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

### Upcoming features
- Wallets
- Sending coins through a network
- Better Documentation