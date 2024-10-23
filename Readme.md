# Rnark
Rnark is a rust library for creating groth16 snarks. My goal in this project is to make a gnark like rust library from scratch. Only the following crates are allowed
- `std::io` for input/out
- `std::error` for error handling
- `std::collections` for data structures
- `rand` for random numbers

# Modules
These are the following modules I would make from an overview
- `ff` : ff will be a finite field library. It would contain all the tools related to finite fields and ff extensions,arthematic and more.
-  `ecc`: ECC will be an elliptic curve library built with help of ff.
- `circuit`: Writing circuits in rust
- `pcs`: Pcs will be a polynomial commitment scheme library

These are the some I can think of though these are subject to change

## Goals/Milestone
- [ ] Develop the ff library