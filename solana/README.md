# Solana

- https://solana.com
- https://solana.com/docs
- https://solana.com/docs/intro/installation

From the installation documentation make sure you install :

```
nvm
npm
node
cargo
rustc
solana-cli
avm
anchor
yarn
```

- Useful commands for Solana
```
$ sudo $(command -v solana-sys-tuner) --user $(whoami) > sys-tuner.log 2>&1

$ solana-test-validator
$ solana config set --url http://127.0.0.1:8899
$ solana config set -ul
$ solana config get

$ solana-keygen new
$ solana address
$ solana account
$ solana balance <address>
$ solana airdrop <amt> <address>
```

## Anchor stuff
```
$ anchor init my-1st-project
$ cd my-1st-project
$ anchor build
$ anchor deploy
```

- Anchor Test
```
$ anchor test --skip-local-validator
```

- Output
```
  my-1st-project
Your transaction signature Z4B32v46DeZpYUqVyzq7M2E5dr7ehW9RjT6E98bHfPwN6Mxe3JKZQTQSQDPH4gYXPtmVw77jFpSNJaKuoDnjSUE
    ✔ Is initialized! (378ms)


  1 passing (379ms)

✨  Done in 1.78s.
```

- To reclaim the SOL allocated to a program account, you can close your Solana program.<br/>
To close a program, use the solana program close <PROGRAM_ID> command. For example:
```
solana program close 3ynNB373Q3VAzKp7m4x238po36hjAGFXFJB4ybN2iTyg --bypass-warning
```

## Solana - Getting Started

- https://solana.com/docs/intro/quick-start
- Solana Playground > https://beta.solpg.io
- Solana Explorer (Beta) > https://explorer.solana.com/address/FGeUws71meAJHbRyFiN2GC9E6N8Jfs3sh5HNjBnYjbBZ?cluster=devnet
- https://faucet.solana.com

### You'll learn

- Solana Accounts: Learn how the Solana network stores data.
- Sending Transactions: Learn to interact with the Solana network by sending transactions.
- Building and Deploying Programs: Create your first Solana program and deploy it to the network.
- Program Derived Addresses (PDAs): Learn how to use PDAs to create deterministic addresses for accounts.
- Cross-Program Invocations (CPIs): Learn how to call other programs from within your program, enabling complex<br/> 
  interactions and  composability between different programs on Solana.

## Solana Developer Bootcamp 2024 - Projects 1-9

- Solana Developer Bootcamp 2024 - Learn Blockchain and Full Stack Web3 Development - Projects 1-9
  - https://youtu.be/amAq-WHAFs8?si=rXVFGr9ORutOapdY via Solana YT Channel

(0:00:00) | Welcome to the Bootcamp<br/>
(0:03:06) | Blockchain Basics<br/>
(0:10:22) | Project 1 | Building a Favorites Program<br/>
(0:37:42) | Project 2 | Creating a Voting Application<br/>
(1:50:32) | Project 3 | Integrating Blinks and Actions<br/>
(2:31:45) | Project 4 | Building a CRUD Application<br/>
(3:34:51) | Project 5 | Creating a Token<br/>
(3:49:12) | Project 6 | Creating a NFT<br/>
(4:25:22) | Project 7 | Building a Swap Program<br/>
(5:48:15) | Project 8 | Creating a Token Vesting App<br/>
(8:31:17) | Project 9 | Building a Token Lottery<br/>

<br/>

## other stuff to look into later on...

Setting up AI Tooling for Solana development<br/>
This section details optional AI tooling setup you can use to accelerate your Solana development.<br/>

- MCP server that you can connect to with cursor to improve Solana AI assisted development. https://mcp.solana.com/<br/>
- LLM.txt optimized documentation that you can use to train LLMs on Solana docs. https://solana.com/llms.txt<br/>

