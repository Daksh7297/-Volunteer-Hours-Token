<div align="center">

# 🎖️ Volunteer Hours Token (VHT)

### A Soroban Smart Contract on Stellar Blockchain

*Rewarding Volunteers, One Token at a Time!* 🌟

![Stellar](https://img.shields.io/badge/Stellar-Soroban-7C3AED?style=for-the-badge&logo=stellar&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-Smart%20Contract-E74C3C?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-2ECC71?style=for-the-badge)
![Network](https://img.shields.io/badge/Network-Testnet-F39C12?style=for-the-badge)
![Status](https://img.shields.io/badge/Status-Deployed-00D4AA?style=for-the-badge)

<br>

[View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CC4LLDBSDFT7LNN2HPKHCJHEGWQGMGCQRC7PYPTK2S46ZPSWROLJB6JW
) · [StellarIDE](https://stellaride.vercel.app/ide) · [Freighter Wallet](https://freighter.app/)

</div>

---

## 📋 Table of Contents

- [Project Description](#-project-description)
- [Problem Statement](#-problem-statement)
- [What It Does](#-what-it-does)
- [Features](#-features)
- [Architecture](#-architecture)
- [Smart Contract Functions](#-smart-contract-functions)
- [Token Economics](#-token-economics)
- [Getting Started](#-getting-started)
- [Usage and Commands](#-usage-and-commands)
- [Tech Stack](#-tech-stack)
- [Project Structure](#-project-structure)
- [Roadmap](#-roadmap)
- [Contributing](#-contributing)
- [License](#-license)

---

## 📖 Project Description

**Volunteer Hours Token (VHT)** is a decentralized token system built on the **Stellar Blockchain** using **Soroban Smart Contracts (Rust)**. It solves the critical problem of tracking, verifying, and rewarding volunteer work in a transparent and tamper-proof manner.

When volunteers contribute their time to organizations — such as **NGOs, colleges, community groups, or corporate CSR programs** — they earn **VHT tokens**, a blockchain-based digital token that serves as **verifiable proof of their contribution**. These tokens can then be **redeemed for real-world rewards** like gift cards, certificates, merchandise, and more.

### 🎯 Mission

> *"To create a transparent, tamper-proof, and rewarding ecosystem that motivates volunteers and gives organizations a reliable way to track and recognize contributions."*

### 🌍 Why Stellar Blockchain?

Feature | Benefit
--------|--------
⚡ **3-5 Second Finality** | Transactions confirm almost instantly
💰 **Near-Zero Fees** | Negligible transaction costs
🌱 **Eco-Friendly** | Minimal energy consumption with no mining
🔒 **Battle-Tested Security** | Proven network operating since 2014
🌐 **Global Accessibility** | Works with just a smartphone and internet
🛠️ **Soroban Smart Contracts** | Powerful Rust-based contract platform

---

## ❌ Problem Statement

Volunteer work tracking faces significant challenges worldwide:

\# | Problem | Impact
---|---------|-------
1 | 📋 **No Transparent Tracking** | Hours recorded in spreadsheets or paper — easy to manipulate
2 | 🎭 **Fake Hours Claims** | No verification mechanism — anyone can claim false hours
3 | 😔 **No Reward System** | Volunteers get a "thank you" but no tangible recognition
4 | 📉 **Motivation Loss** | 64% of volunteers say lack of recognition reduces motivation
5 | 🚫 **No Portable Proof** | Hours at one organization cannot be verified by another
6 | 📊 **Administrative Overhead** | Manual tracking wastes organizational resources
7 | 🔒 **No Cross-Org Recognition** | Volunteer work is not recognized across organizations

### 📊 Real-World Impact

- 🌍 **880 million+** people volunteer worldwide (UN data)
- 💰 **\$187 billion** worth of volunteer time contributed annually (US alone)
- 📉 **64%** of volunteers cite lack of recognition as demotivating
- 🚫 **NO** standardized global system exists to track and reward volunteering

---

## ⚙️ What It Does

VHT creates a **complete lifecycle** for volunteer management on the blockchain:

> **VOLUNTEER REGISTERS** ➜ **LOGS HOURS** ➜ **ADMIN APPROVES** ➜ **TOKENS MINTED**
>
> **REWARD CLAIMED** ⬅ **BROWSES REWARDS** ⬅ **CHECKS BALANCE**

### Detailed Flow

Step | Actor | Action | Result
-----|-------|--------|-------
1 | 🔴 Admin | Deploys and initializes contract | Token system is live
2 | 🔴 Admin | Adds rewards to marketplace | Rewards available for redemption
3 | 🟢 Volunteer | Registers with Stellar wallet | Gets on-chain identity
4 | 🟢 Volunteer | Logs work hours with description | Entry recorded (pending approval)
5 | 🔴 Admin | Reviews and approves hours | Triggers auto token minting
6 | 🤖 Contract | Mints VHT tokens to volunteer | Tokens appear in volunteer wallet
7 | 🟢 Volunteer | Browses reward marketplace | Sees available rewards and costs
8 | 🟢 Volunteer | Redeems tokens for reward | Tokens burned and reward claimed

---

## ✨ Features

### 🔧 Core Features

Feature | Description | Status
--------|-------------|-------
🏗️ **Contract Initialization** | One-time setup with admin, token name, symbol, and rate | ✅ Done
👤 **Volunteer Registration** | Self-registration with wallet address and name | ✅ Done
⏰ **Hours Logging** | Log 1-24 hours per entry with work description | ✅ Done
✅ **Admin Approval** | Admin verifies and approves logged hours | ✅ Done
🪙 **Auto Token Minting** | Tokens automatically minted on approval at configurable rate | ✅ Done
🎁 **Reward Marketplace** | Admin adds redeemable rewards with token costs | ✅ Done
🎫 **Reward Redemption** | Volunteers spend tokens to claim real-world rewards | ✅ Done
📤 **Token Transfer** | Peer-to-peer token transfer between volunteers | ✅ Done
💰 **Balance Checking** | Real-time on-chain token balance inquiry | ✅ Done
📊 **Supply Tracking** | Total token supply monitoring | ✅ Done
🔥 **Token Burning** | Tokens burned on redemption creating deflationary mechanism | ✅ Done
📜 **Activity Logging** | All hours, approvals, and redemptions permanently recorded | ✅ Done

### 🔒 Security Features

Feature | Description
--------|------------
🔐 **Admin-Only Functions** | approve_hours and add_reward restricted to admin only
🔑 **Wallet Authentication** | All write operations require require_auth verification
🛡️ **Double-Approval Prevention** | Hours cannot be approved twice with duplicate check
⚠️ **Balance Validation** | Transfer and redeem fails if insufficient tokens
🚫 **Duplicate Registration** | Volunteers cannot register twice with same wallet
✅ **Input Validation** | Hours validated in 1-24 range and amounts must be positive
🔄 **Re-initialization Guard** | Contract cannot be initialized more than once
📦 **Stock Management** | Rewards auto-deactivate when stock reaches zero

### 🌐 Transparency Features

Feature | Description
--------|------------
📜 **On-Chain Records** | Every action permanently recorded on Stellar blockchain
🔍 **Public Verification** | Anyone can verify volunteer contributions via block explorer
📊 **Auditable Trail** | Complete history of hours, approvals, minting, and redemptions
🌍 **Decentralized** | No single point of failure as data lives on distributed network

---

## 🏗️ Architecture

> **User (Browser)** ➜ **Freighter Wallet (Auth and Signing)** ➜ **Stellar Blockchain (Soroban)** ➜ **VHT Smart Contract (Rust/WASM)**

**Smart Contract Modules:**

- 🪙 **Token Engine** — mint, burn, transfer
- 👤 **Volunteer Manager** — register, log hours, approve
- 🎁 **Reward Marketplace** — add rewards, redeem
- 💾 **On-Chain Storage** — balances, volunteers, hours, rewards, redemptions

### 👥 User Roles

Role | Icon | Permissions
-----|------|------------
**Admin** | 🔴 | Initialize contract, approve hours, add rewards, manage system
**Volunteer** | 🟢 | Register, log hours, check balance, redeem rewards, transfer tokens
**Public** | 🔵 | View statistics and verify contributions via explorer (read-only)

---

## 📞 Smart Contract Functions

### 🔴 Admin Functions

Function | Parameters | Description | Returns
---------|-----------|-------------|--------
`initialize()` | admin, token_name, token_symbol, tokens_per_hour | One-time contract setup | —
`approve_hours()` | log_id | Approve volunteer hours and auto-mint tokens | —
`add_reward()` | name, cost, available | Add new reward to marketplace | reward_id

### 🟢 Volunteer Functions

Function | Parameters | Description | Returns
---------|-----------|-------------|--------
`register_volunteer()` | volunteer_address, name | Self-register as volunteer | —
`log_hours()` | volunteer, hours, description | Log work hours with 1-24 range pending approval | log_id
`redeem_reward()` | volunteer, reward_id | Spend tokens to claim reward | redemption_id
`transfer()` | from, to, amount | Transfer tokens to another volunteer | —

### 🔵 View Functions (Read-Only)

Function | Parameters | Returns
---------|-----------|--------
`balance()` | account | Token balance
`total_supply()` | — | Total tokens in circulation
`get_volunteer()` | address | Volunteer info including hours, tokens, and status
`get_reward()` | reward_id | Reward details including name, cost, and stock
`get_hours_log()` | log_id | Hours log entry details
`get_volunteer_count()` | — | Total registered volunteers
`get_reward_count()` | — | Total rewards in marketplace
`name()` | — | Token name
`symbol()` | — | Token symbol
`get_rate()` | — | Tokens earned per hour
`get_redemption()` | redemption_id | Redemption record details
`get_redemption_count()` | — | Total redemptions made

---

## 💰 Token Economics

Property | Value
---------|------
**Name** | Volunteer Hours Token
**Symbol** | VHT
**Decimals** | 0 (whole tokens only)
**Initial Supply** | 0 (no pre-mint)
**Max Supply** | Unlimited (demand-based)
**Minting** | Automatic on admin approval
**Burning** | On reward redemption
**Transfer** | Peer-to-peer enabled
**Rate** | 1 Hour = 1 VHT (configurable by admin)

### Token Flow

- **MINT (Supply Increases):** Volunteer works ➜ Admin approves ➜ Tokens created
- **BURN (Supply Decreases):** Volunteer redeems reward ➜ Tokens destroyed

This creates a **healthy token economy** where supply increases with volunteer contributions and decreases when rewards are claimed. Tokens maintain real value because they are backed by actual verified volunteer hours.

### 🎁 Sample Reward Tiers

Tier | Reward | Cost (VHT) | Hours Required
-----|--------|-----------|---------------
🥉 Bronze | Coffee Voucher | 5 VHT | 5 hours
🥈 Silver | Movie Ticket | 10 VHT | 10 hours
🥇 Gold | T-Shirt or Merchandise | 25 VHT | 25 hours
💎 Platinum | Online Course Certificate | 50 VHT | 50 hours
👑 Diamond | Gift Card | 100 VHT | 100 hours
🏆 Legend | Volunteer of the Month Award | 200 VHT | 200 hours

---

## 🚀 Getting Started

### Prerequisites

Tool | Purpose | Link
-----|---------|-----
Freighter Wallet | Stellar wallet as Chrome Extension | [Download](https://freighter.app/)
StellarIDE | Browser-based IDE for Soroban | [Open](https://stellaride.vercel.app/ide)
Stellar Testnet | Free test network | Automatic via Freighter

### Quick Deploy

1. Open [StellarIDE](https://stellaride.vercel.app/ide)
2. Paste contract code in `contracts/contract/src/lib.rs`
3. Click **Build** and wait for Build Complete
4. Connect **Freighter Wallet** set to Testnet
5. Click **Upload WASM** and approve in Freighter
6. Click **Instantiate Contract** and approve in Freighter
7. Save the **Contract ID** that is displayed

### Wallet Setup

1. Install Freighter from Chrome Web Store
2. Create new wallet and **save recovery phrase securely**
3. Switch to **TESTNET** network in Freighter settings
4. Get free test XLM from Stellar Friendbot

---

## 📖 Usage and Commands

> **Note:** Replace `YOUR_CONTRACT_ID` with your deployed contract address and `YOUR_ADMIN_WALLET` or `VOLUNTEER_WALLET` with actual Stellar public keys in all commands below.

### 1. Initialize Contract (One-Time)

    stellar contract invoke \
      --id YOUR_CONTRACT_ID \
      --source-account YOUR_ADMIN_WALLET \
      --network testnet \
      -- \
      initialize \
      --admin YOUR_ADMIN_WALLET \
      --token_name VHT \
      --token_symbol VHT \
      --tokens_per_hour 1

### 2. Add Rewards (Admin)

    stellar contract invoke \
      --id YOUR_CONTRACT_ID \
      --source-account YOUR_ADMIN_WALLET \
      --network testnet \
      -- \
      add_reward --name Coffee --cost 5 --available 100

### 3. Register Volunteer

    stellar contract invoke \
      --id YOUR_CONTRACT_ID \
      --source-account VOLUNTEER_WALLET \
      --network testnet \
      -- \
      register_volunteer \
      --volunteer_address VOLUNTEER_WALLET \
      --name Raju

### 4. Log Hours

    stellar contract invoke \
      --id YOUR_CONTRACT_ID \
      --source-account VOLUNTEER_WALLET \
      --network testnet \
      -- \
      log_hours \
      --volunteer VOLUNTEER_WALLET \
      --hours 8 \
      --description Cleanup

### 5. Approve Hours (Admin Only)

    stellar contract invoke \
      --id YOUR_CONTRACT_ID \
      --source-account YOUR_ADMIN_WALLET \
      --network testnet \
      -- \
      approve_hours --log_id 0

### 6. Check Balance

    stellar contract invoke \
      --id YOUR_CONTRACT_ID \
      --network testnet \
      -- \
      balance --account VOLUNTEER_WALLET

### 7. Redeem Reward

    stellar contract invoke \
      --id YOUR_CONTRACT_ID \
      --source-account VOLUNTEER_WALLET \
      --network testnet \
      -- \
      redeem_reward \
      --volunteer VOLUNTEER_WALLET \
      --reward_id 0

### 8. Transfer Tokens

    stellar contract invoke \
      --id YOUR_CONTRACT_ID \
      --source-account FROM_WALLET \
      --network testnet \
      -- \
      transfer \
      --from FROM_WALLET \
      --to TO_WALLET \
      --amount 3

---

## 🛠️ Tech Stack

Layer | Technology | Purpose
------|-----------|--------
Blockchain | Stellar Network | Decentralized ledger
Smart Contract | Soroban (Rust compiled to WASM) | Business logic on-chain
Wallet | Freighter | User authentication and transaction signing
IDE | StellarIDE | Browser-based development environment
Testing | Soroban SDK | Contract unit testing
Network | Stellar Testnet | Development and testing environment
Explorer | Stellar Expert | Transaction verification and monitoring

---

## 📁 Project Structure

    VolunteerHoursToken/
    ├── contract/
    │   ├── src/
    │   │   └── lib.rs              # Main smart contract code (Rust)
    │   ├── Cargo.toml              # Rust dependencies and configuration
    │   └── ...
    ├── .gitignore                   # Git ignore rules
    └── README.md                    # Project documentation (this file)

### Key File: contract/src/lib.rs

Section | Description
--------|------------
Data Structures | Volunteer, HoursLog, Reward, and Redemption structs
Storage Keys | DataKey enum for on-chain storage mapping
initialize() | Contract setup with admin and token configuration
register_volunteer() | Volunteer self-registration with validation
log_hours() | Hour logging with 1-24 hour validation
approve_hours() | Admin approval with automatic token minting
add_reward() | Reward marketplace management
redeem_reward() | Token redemption with burning mechanism
transfer() | Peer-to-peer token transfer with balance checks
View Functions | Read-only queries for balances, info, and counts

---

## 🗺️ Roadmap

### Phase 1: Smart Contract ✅ Complete

- [x] Contract initialization system
- [x] Volunteer registration
- [x] Hours logging with validation
- [x] Admin approval workflow
- [x] Automatic token minting
- [x] Reward marketplace
- [x] Token redemption and burning
- [x] Peer-to-peer transfer
- [x] Testnet deployment

### Phase 2: Frontend Development 🔄 In Progress

- [ ] Admin dashboard UI
- [ ] Volunteer dashboard UI
- [ ] Reward marketplace UI
- [ ] Freighter wallet integration
- [ ] Transaction history viewer

### Phase 3: Advanced Features 📋 Planned

- [ ] Multi-organization support
- [ ] Volunteer leaderboard
- [ ] Achievement badges as NFTs
- [ ] Automated verification with GPS or QR code
- [ ] Email and push notifications
- [ ] Analytics dashboard

### Phase 4: Production Launch 🚀 Future

- [ ] Security audit
- [ ] Mainnet deployment
- [ ] Partner onboarding for NGOs and colleges
- [ ] Mobile app for iOS and Android
- [ ] Public launch

---

## 🤝 Contributing

Contributions are welcome! Here is how you can help:

1. **Fork** the repository
2. **Create** a feature branch — `git checkout -b feature/amazing-feature`
3. **Commit** your changes — `git commit -m 'Add amazing feature'`
4. **Push** to the branch — `git push origin feature/amazing-feature`
5. **Open** a Pull Request

### Ideas for Contribution

- 🐛 Bug fixes and improvements
- ✨ New smart contract features
- 🎨 Frontend development
- 📝 Documentation updates
- 🧪 Test cases

---

## 📄 License

This project is licensed under the **MIT License**.

    MIT License

    Copyright (c) 2025 dakshjecrcfoundation

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.

---

## 👨‍💻 Author

**dakshjecrcfoundation** — [GitHub Profile](https://github.com/dakshjecrcfoundation)

---

## 🙏 Acknowledgements

- [Stellar Development Foundation](https://stellar.org/) — Blockchain platform
- [Soroban](https://soroban.stellar.org/) — Smart contract framework
- [Freighter](https://freighter.app/) — Stellar wallet
- [StellarIDE](https://stellaride.vercel.app/) — Online development environment
- [Stellar Expert](https://stellar.expert/) — Block explorer

---

<div align="center">

### ⭐ Star this repo if you found it useful!

Made with ❤️ for **Volunteers Worldwide** 🌍

*Building a better world, one token at a time* 🪙

</div>


Wallet Address: GBVDVITLNJMA7JGVVSBNG3CYHT666SDAJWYOMPX6YWHJIMCFMXKY5SQS


Contract Address: CC4LLDBSDFT7LNN2HPKHCJHEGWQGMGCQRC7PYPTK2S46ZPSWROLJB6JW


https://stellar.expert/explorer/testnet/contract/CC4LLDBSDFT7LNN2HPKHCJHEGWQGMGCQRC7PYPTK2S46ZPSWROLJB6JW


<img width="1366" height="684" alt="image" src="https://github.com/user-attachments/assets/0938adde-7b32-4663-a4c6-cb9983573a43" />



