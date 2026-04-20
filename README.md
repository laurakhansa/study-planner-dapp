# Stellar Study Planner DApp

**Stellar Study Planner DApp** - Blockchain-Based Decentralized Study & Task Management System

## Project Description

Stellar Study Planner DApp is a decentralized smart contract solution built on the Stellar blockchain using Soroban SDK. It provides a secure, immutable platform for students and lifelong learners to manage their study schedules, subjects, and topics directly on the blockchain. 

Unlike centralized applications, this DApp is **Wallet-Integrated**. It maps all study plans directly to the user's Stellar wallet address (`Address`). Data is managed securely; users must authenticate via their own wallets to create or delete their study plans, ensuring complete privacy and data ownership.

## Key Features

### 1. Wallet-Bound Data (Security)
- All study plans are tied directly to the creator's wallet Address.
- Utilizes `require_auth()` to ensure only the true owner can modify or delete their plans.

### 2. Simple Study Plan Creation
- Create study sessions by providing the `user` address, `subject`, and `topic`.
- Automated ID generation for unique task identification.

### 3. Isolated Data Retrieval
- Fetch your personalized study collection simply by querying your wallet address.
- Real-time synchronization with the Stellar blockchain state.

### 4. Secure Deletion
- Remove specific completed study plans using their unique IDs.
- Permanent removal from the contract storage, verified on-chain.

## Contract Details

- **Testnet Smart Contract ID:** `[CCUXDGCTKGY4PZMXP7ZALJYGU6WRSWH6ISHWM6LLC2AN4K4UPNGTMCJB]`
- <img width="2880" height="1800" alt="image" src="https://github.com/user-attachments/assets/3496708c-8385-4f16-a00e-d3c8635e32aa" />


## Future Scope

### Short-Term Enhancements
1. **Deadlines & Timestamps:** Add timestamp features to track when a study session is due.
2. **Status Tracking:** Implement "Pending", "In Progress", and "Completed" statuses.

### Medium-Term Development
3. **Study Group Collaboration:** Multi-signature requirements to allow study groups to share learning materials.
4. **Reward System:** Issue small token rewards for completing study plans on time to motivate learners.

## Getting Started

Deploy the smart contract to Stellar's Soroban Testnet and interact with it using the three main functions. Don't forget to pass your wallet address as the `user` parameter!

- `create_plan(user: Address, subject: String, topic: String)` - Create a new study material.
- `get_plans(user: Address)` - Retrieve all stored study plans for a specific wallet.
- `delete_plan(user: Address, id: u64)` - Remove a specific study plan by its ID.

---
**Stellar Study Planner DApp** - Own your learning journey on the Blockchain.
