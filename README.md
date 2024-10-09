
# Viora - Solar Flare

This repository contains the Solana program for Viora, a crowdfunding platform for music artists to gather funds for their future work. The program is built using the Anchor framework and can be deployed to various networks like localnet, devnet, testnet, and mainnet.

## Prerequisites
- Ensure you have Rust installed: [Rust Installation](https://www.rust-lang.org/tools/install)
- Install Solana CLI: [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- Install Anchor: [Anchor Installation](https://book.anchor-lang.com/chapter_2/installation.html)
- Install Nodejs using nvm
- Install Yarn
---

## Commands

**Clean store cache**

```bash
anchor clean
```

**Sync keys**

```bash
anchor keys sync
```

**Build the program**

```bash
anchor build
```

**Running a local Solana node**

To spin up a local Solana blockchain, run:

```bash
solana-test-validator
```

You can add `-r` to reset the environment before starting:

```bash
solana-test-validator -r
```

**Deploy the program**

```bash
anchor deploy
```
** Get network config**
To see your current config:
```bash
solana config get
```
You should see output similar to the following:
```bash
Config File: /Users/test/.config/solana/cli/config.yml
RPC URL: https://api.mainnet-beta.solana.com
WebSocket URL: wss://api.mainnet-beta.solana.com/ (computed)
Keypair Path: /Users/test/.config/solana/id.json
Commitment: confirmed
```
**Set network**
To change network config:
```bash
solana config set -ul
```

Where:
- `-ul`: Localnet
- `-ud`: Devnet
- `-um`: Mainnet

---

## Network Options

You can switch the network by passing:

```bash
--provider.cluster <NETWORK>
```

Supported networks: `localnet`, `devnet`, `mainnet`.
