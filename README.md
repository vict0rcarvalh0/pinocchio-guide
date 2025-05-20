# Pinocchio Guide
## Introduction

Pinocchio is a zero-dependency library to create Solana programs in Rust. It takes advantage of the way SVM loaders serialize the program input parameters into a byte array that is then passed to the program's entrypoint to define zero-copy types to read the input. Since the communication between a program and SVM loader — either at the first time the program is called or when one program invokes the instructions of another program — is done via a byte array, a program can define its own types. This completely eliminates the dependency on the solana-program crate, which in turn mitigates dependency issues by having a crate specifically designed to create on-chain programs.

As a result, Pinocchio can be used as a replacement for solana-program to write on-chain programs, which are optimized in terms of both compute units consumption and binary size.

The Pinocchio Guide serves as a comprehensive resource for understanding and implementing core account management functionalities on the Solana blockchain. It emphasizes foundational principles, code-level explanations, and best practices to help developers fully leverage Solana's unique transaction optimization system.

Feel free to contribute to the repository and optimize the examples and documentation in any way!

## Documentation

- [Guide](GUIDE.md): This file provides a comprehensive explanation of Pinocchio's core functionalities, focusing on system-level operations and instruction calls. It includes examples and best practices for implementing account management functionalities on the Solana blockchain.
  
- [Tutorial](TUTORIAL.md): This file demonstrates how to transform an Anchor-based Vault into a Pinocchio-based Vault. It provides step-by-step instructions for building and optimizing a Vault program, comparing the high-level abstractions of Anchor with the low-level optimizations of Pinocchio.

## Examples

The `examples` folder contains program examples that implement Pinocchio functions. These examples are still a work in progress and may contain errors or incomplete implementations. Contributions to improve these examples are welcome!