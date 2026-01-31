


# Escrow 

```md
# Escrow Program (Solana + Anchor)

This repository contains a simple Escrow smart contract implemented using Solana Anchor (Rust).

The program models a basic escrow flow between a buyer and a seller using on-chain state.



## Features

- Initialize escrow with buyer, seller, and amount
- Mark escrow as funded
- Release escrow after validation
- Safe and minimal state transitions



## Instructions

### `initialize`
Creates a new escrow account and stores:
- Buyer address
- Seller address
- Escrow amount
- Funded status (false)

### `deposit`
Marks the escrow as funded.

### `release`
Releases the escrow by resetting the funded flag.
Fails if escrow is not funded.

---

## Build Instructions

```bash
   anchor build
