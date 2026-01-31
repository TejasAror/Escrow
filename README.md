# Escrow Smart Contract (Solana + Anchor)

This repository contains a simple Escrow smart contract implemented using the Solana Anchor framework (Rust).

The contract demonstrates how an escrow agreement between a buyer and a seller can be modeled using on chain state, focusing on correctness, clarity and Anchor best practices.



## Overview

The escrow program allows:
- A buyer to initialize an escrow agreement
- Funds to be logically marked as deposited
- The escrow to be released after validation

This implementation is intentionally state based and does not handle real SOL transfers, as it is designed for learning and assignment purposes.



## Features

- Initialize an escrow with buyer, seller and amount
- Mark escrow as funded
- Release escrow with validation
- Custom error handling
- Anchor safety checks for unchecked accounts



## Instructions

### initialize
Creates a new escrow account and stores:
- Buyer public key
- Seller public key
- Escrow amount
- Funded status (set to `false`)

### deposit
Marks the escrow as funded by updating the on-chain state.

### release
Releases the escrow by resetting the funded status.
Fails if the escrow is not funded.



## Error Handling

- NotFunded 
  Thrown when attempting to release an escrow that has not been funded.



## Build Instructions

To build the program locally:

```bash
   anchor build
