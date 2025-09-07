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

- Anchor stuff
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

#### other stuff to look into later on...

Setting up AI Tooling for Solana development<br/>
This section details optional AI tooling setup you can use to accelerate your Solana development.<br/>

- MCP server that you can connect to with cursor to improve Solana AI assisted development. https://mcp.solana.com/<br/>
- LLM.txt optimized documentation that you can use to train LLMs on Solana docs. https://solana.com/llms.txt<br/>

