# Rnark
Rnark is a gnark like library in rust. Only the following crates are allowed
- `std` for input/out , error handling,collections etc
- `num_bigint` for bigint operations
- `rand` for random numbers
- `serde` for serialization

# Modules
These are the following modules I would make from an overview
- `ff` : ff will be a finite field library. It would contain all the tools related to finite fields and ff extensions,arthematic and more.
-  `ecc`: ECC will be an elliptic curve library built with help of ff.
- `circuit`: Writing circuits in rust
- `pcs`: Pcs will be a polynomial commitment scheme library


