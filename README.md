# Reec

**reec** is a lightweight Ethereum Execution Client written in Rust.  
It aims to provide a modular and readable implementation of the Ethereum execution layer, while also serving as a practical base for experimentation and interoperability.

---

## ✨ Goals

- **Lightweight & modular**: Clean, minimal execution client architecture written in idiomatic Rust.
- **Spec-aligned**: Implements the Ethereum Execution Layer as defined in the [execution specs](https://github.com/ethereum/execution-specs?tab=readme-ov-file).
- **Interoperable**: Able to sync and communicate with other Ethereum clients.

---

## 📦 Architecture Overview

```text
reec/
├── consensus/     # Consensus-related interfaces (e.g., Engine API integration)
├── rpc/           # JSON-RPC server and method handlers (eth, net, web3)
├── core/          # Core Ethereum execution logic: EVM, block processing, tx pool
├── net/           # DevP2P networking, peer discovery, RLPx, protocol handling
├── storage/       # Database layer: chain data, state trie, receipts, etc.
```

##📍 Roadmap

### Milestone 1: RPC Node
Add support to follow a post-Merge localnet as a read-only RPC Node. This first milestone will only support a canonical chain (every incoming block has to be the child of the current head).

RPC endpoints
 - `engine_newPayloadV3` (excl. block building)
 - `eth_blobBaseFee`
 - `eth_blockNumber`
 - `eth_call` (at head block)
 - `eth_chainId`
 - `eth_createAccessList`  (at head block)
 - `eth_estimateGas`
 - `eth_feeHistory`
 - `eth_getBalance` (at head block)
 - `eth_getBlockByHash`
 - `eth_getBlockByNumber`
 - `eth_getBlockReceipts`
 - `eth_getBlockTransactionCountByNumber`
 - `eth_getCode` (at head block)
 - `eth_getStorageAt` (at head block)
 - `eth_getTransactionByBlockHashAndIndex`
 - `eth_getTransactionByBlockNumberAndIndex`
 - `eth_getTransactionByHash` 

### Milestone 2: P2P Network
Implement DevP2P protocol, including RLPx `p2p` and `eth` capabilities. This will allow us to receive blocks and transactions from other nodes and is a prerequisite for the next milestones.
 
 RPC endpoints
 - `admin_nodeInfo`

### Milestone 3: Reorgs
 Implement support for block reorganizations (reorgs) and historical state queries. This milestone involves persisting the state trie to enable efficient access to historical states and implementing a tree structure for the blockchain to manage multiple chain branches. We'll develop algorithms for chain reorganizations and enhance the query system to support state lookups at any historical block.
 
 RPC endpoints
 - `engine_exchangeCapabilities`
 - `engine_forkchoiceUpdatedV3`
 - `eth_call` (at any block)
 - `eth_createAccessList`  (at any block)
 - `eth_getBalance` (at any block)
 - `eth_getCode` (at any block)
 - `eth_getProof`
 - `eth_getStorageAt` (at any block)
 
 ### Milestone 4: Syncing
 Add snap sync protocol, which lets us get a recent copy of the blockchain state instead of going through all blocks from genesis. Since we don't support older versions of the spec by design, this is a prerequisite to being able to sync the node with public networks, including mainnet.
 
 RPC endpoints
 - `engine_forkchoiceUpdatedV3`
 - `engine_newPayloadV3`
 - `eth_syncing`

### Milestone 5: Block building
 Keep transactions received from other nodes in memory, and add the ability to build new payloads, so that the consensus client can propose new blocks.
 
 RPC endpoints
 - `engine_getPayloadV3`
 - `engine_newPayloadV3` (with block building)

## 🤝 Contributing
We're building reec in the open and welcome contributors!

If you'd like to contribute:

Clone the repo and set up your environment

Look through [open issues](https://github.com/ScalingLabs/reec/issues)

Create a PR with clear commit history and tests

Be kind and respectful — we’re here to learn and build together.

## 👤 Maintainers
[@owanikin](https://github.com/owanikin)
