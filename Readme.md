# Rnark
Rnark is a gnark like library in rust. Only the following crates are allowed
- `std` for input/out , error handling,collections etc
- `rand` for random numbers
- `serde` for serialization

# Modules
These are the following modules I would make from an overview
- `bigint`:  It will contains all tools neccessary for big integer operations
- `ff` : ff will be a finite field library. It would contain all the tools related to finite fields and ff extensions,arthematic and more.
-  `ecc`: ECC will be an elliptic curve library built with help of ff.
- `circuit`: Writing circuits in rust
- `pcs`: Pcs will be a polynomial commitment scheme library

These are the some I can think of though these are subject to change

# RoadMap
## BigInt
- [X] Simple addition of bigint's , to  and into traits for common numeric types
- [X] implement Karstuba's algorithm for bigint multiplication (both balanced and unbalanced)
- [ ] SvobodaDivision and Recursive divide and conquer for bigint division
- [ ] Exact division algorith based on Karp-MarkStein trick
- [ ] implement square root, gcd algorithms
- [ ] base conversions
- [ ] Modular arthmatics and fast fourier transform
