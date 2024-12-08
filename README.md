# Onchain Voting System (Rust & AnchorLang Framework)

Welcome to my Onchain Voting System project! 🗳️ This is a Web3-based solution for secure and decentralized voting, built on the Solana blockchain using Rust and the AnchorLang framework.

## Overview

This program allows users to participate in transparent, tamper-proof voting processes directly on the blockchain. The system is designed to ensure security, anonymity, and fairness, leveraging the power of Solana's speed and low transaction costs.

## Features

- Secure Voting: Each vote is recorded immutably on the blockchain.
- Anonymity: Voters' identities remain confidential while ensuring the integrity of their votes.
- Decentralized Logic: Smart contracts handle vote submissions, tallying, and result announcements.
- High Performance: Built on Solana, ensuring fast transactions with minimal fees.
- Custom Voting Options: Supports creating and managing multiple voting polls.

## Requirements

To run this project, you'll need:

- [Rust](https://www.rust-lang.org/tools/install) and [AnchorLang](https://www.anchor-lang.com/docs/installation) installed on your machine.
- A Solana wallet and the Solana CLI for deploying the program to a local or live Solana cluster.

## How to Run

1. Clone the repository:
   ```
   git clone https://github.com/AdedigbaOluwad1/onchain-voting-system.git 
   ```
2. Navigate to the project folder:
   ```
   cd onchain-voting-system
   ```
3. Install dependencies:
   ```
   anchor build
   ```
4. Deploy the program (localnet):
   ```
   anchor deploy
   ```
5. Run the client (for testing):
   ```
   anchor test
   ```
6. Interact with the program:
   - Use the Solana CLI or custom scripts to create and participate in voting polls.

## Code Explanation

- Program Initialization: Handles PDA creation and sets up the voting poll with metadata (e.g., poll name, description, options).
- Vote Submission: Allows users to cast their votes by signing a transaction.
- Vote Tallying: Smart contract logic ensures accurate and tamper-proof vote counting.
- Result Announcement: Votes are tallied automatically, and results can be queried by anyone.

# Contributing

Have ideas for improvement? Found a bug? Contributions are always welcome! Fork this repo, open issues, or submit pull requests to make this project even better.
