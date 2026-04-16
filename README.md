## VoxPop: The Eternal Wall of Aspiration

##📖 About The Application

VoxPop is a fully decentralized, anonymous bulletin board built on the Stellar Soroban smart contract platform. It acts as an "Eternal Wall of Aspiration" where anyone can post messages, criticisms, or confessions safely without fear of censorship or identity tracking.

In a world where digital voices are often silenced or manipulated by centralized platforms, VoxPop leverages blockchain technology to ensure that once a message is posted, it is permanently etched into the Stellar ledger and cannot be deleted by anyone—not even the administrators.

## ✨ Key Features

Anonymous Posting: No user identities or wallet addresses are stored alongside the message content. Your voice is heard, but your identity remains hidden.

Censorship-Resistant (Immutability): The smart contract intentionally lacks a delete or edit function. What goes on the ledger, stays on the ledger.

On-Chain Timestamp Verification: Every message is stamped with the exact, unalterable ledger time (env.ledger().timestamp()), preventing any timeline manipulation.

Gas-Free Experience Ready: Designed to work with Stellar's Fee Sponsorship (Sponsored Reserves). A relayer account can pay the transaction fees, allowing users to send messages completely free of charge.

Event-Based Architecture: Emits Soroban contract events (vox_pop) for every new message, allowing frontend applications to track and display the public feed efficiently without excessive storage costs.

Anti-Spam Limit: Messages are strictly limited to 280 characters to optimize ledger space and prevent storage abuse.

## 🚀 Stellar Testnet Information

The VoxPop smart contract is currently deployed on the Stellar Testnet. You can interact with it using the Soroban CLI or Stellar SDKs.

Network: Stellar Testnet

Smart Contract ID: CA_PLACEHOLDER_VOXPOP_CONTRACT_ID_REPLACE_ME_AFTER_DEPLOY (Note: Replace this with your actual deployed Contract ID)

RPC URL: https://soroban-testnet.stellar.org

Network Passphrase: Test SDF Network ; September 2015

## 🛠 Getting Started

Prerequisites

Make sure you have the Stellar CLI installed and configured for the Testnet.

Interacting via CLI

1. View all public messages (The Lighthouse)

stellar contract invoke \
  --id CA_PLACEHOLDER_VOXPOP_CONTRACT_ID_REPLACE_ME_AFTER_DEPLOY \
  --network testnet \
  -- \
  get_wall


2. Post a new anonymous message (The Whisper)

stellar contract invoke \
  --id CA_PLACEHOLDER_VOXPOP_CONTRACT_ID_REPLACE_ME_AFTER_DEPLOY \
  --network testnet \
  --source YOUR_TESTNET_ACCOUNT \
  -- \
  post_whisper \
  --text "Hello Web3! This is my first anonymous message."


## Smart Contract ID
ID: CCSQYQZCYOJLFLWFIGRV46JBELWX3RJS4AFFYNSCE7XY6VWDSIZGZIZN
<img width="1904" height="855" alt="{D76AD635-15B2-42D3-8F93-BC2DEE5BD43D}" src="https://github.com/user-attachments/assets/015a2c3b-8e31-4a40-a2b5-da0bf93363ab" />

