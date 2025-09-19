## Solana Developer Bootcamp 2024 - Project 2

- Project 1 | Building a Voting Program

- Setup commands
```
mkdir project-2-voting
cd project-2-voting
npx create-solana-dapp
cd voting-dapp
```
- Start the NodeJS dapp
```
npm run dev
```
<br/><img src="./images/project-2-voting-dapp-nodejs.png" width="700"><br/>

- Votingdapp Homepage UI via http://localhost:3000<br/>
<br/><img src="./images/project-2-voting-dapp-frontend-home-ui.png" width="700"/><br/>

- Solana Test Validator (validated via http://127.0.0.1:8899)
```
solana-test-validator
```
<br/><img src="./images/project-2-voting-solana-test-validator.png" width="700"/><br/>

- Install the Phantom wallet & setup a new account
- Change the configuration to Solana > Developer Settings > Solana Localnet

- Votingdapp Account UI via http://localhost:3000/account/5Q2XJX27tw312nCixSmf8AfPv6ijASqxafodHfWhPQrH<br/>
<br/><img src="./images/project-2-voting-dapp-frontend-account-ui.png" width="700"/><br/>

- Voting Dapp Structure
<br/><img src="./images/project-2-voting-dapp-structure.png" width="700"/><br/>

- Make changes and Anchor Build
```
cd solana/project-2-voting/voting-dapp/anchor/
anchor build
```

<br/>Output from Anchor Build
```
warning: `voting` (lib) generated 11 warnings (5 duplicates)
    Finished `release` profile [optimized] target(s) in 1.22s
   Compiling voting v0.1.0 (/Users/robq/repos/rust/solana/project-2-voting/voting-dapp/anchor/programs/voting)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.43s
     Running unittests src/lib.rs (/Users/robq/repos/rust/solana/project-2-voting/voting-dapp/anchor/target/debug/deps/voting-6d84153f12de5567)
```