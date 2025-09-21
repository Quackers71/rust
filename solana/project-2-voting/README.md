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
<br/>

Output from Anchor Build
```
warning: `voting` (lib) generated 11 warnings (5 duplicates)
    Finished `release` profile [optimized] target(s) in 1.22s
   Compiling voting v0.1.0 (/Users/robq/repos/rust/solana/project-2-voting/voting-dapp/anchor/programs/voting)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.43s
     Running unittests src/lib.rs (/Users/robq/repos/rust/solana/project-2-voting/voting-dapp/anchor/target/debug/deps/voting-6d84153f12de5567)
```
<br/>

- Testing your Smart Contract
- https://github.com/kevinheavey/anchor-bankrun/tree/main - you can install using :
```
yarn add anchor-bankrun

#or

npm install anchor-bankrun --force < --force required
```

- Run the test :
```
anchor test --skip-local-validator --skip-deploy

# Need fix this stuff...
```

- Latest anchor build and up to 1:31:55
```
   Finished `release` profile [optimized] target(s) in 1.32s
   Compiling voting v0.1.0 (/Users/robq/repos/rust/solana/project-2-voting/voting-dapp/anchor/programs/voting)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.43s
     Running unittests src/lib.rs (/Users/robq/repos/rust/solana/project-2-voting/voting-dapp/anchor/target/debug/deps/voting-6d84153f12de5567)
```

- Re-ran anchor test - Output (Still need to fix this stuff!)
```
anchor % anchor test --skip-local-validator --skip-deploy

---
Found a 'test' script in the Anchor.toml. Running it as a test suite!

Running test suite: "/Users/robq/repos/rust/solana/project-2-voting/voting-dapp/anchor/Anchor.toml"

 FAIL  tests/basic.test.ts
  ● Test suite failed to run

    Cannot find module 'solana-bankrun-darwin-arm64' from 'node_modules/solana-bankrun/dist/internal.js'

    Require stack:
      node_modules/solana-bankrun/dist/internal.js
      node_modules/solana-bankrun/dist/index.js
      anchor/tests/basic.test.ts

      3 | import { PublicKey } from '@solana/web3.js'
      4 | import { Voting } from '../target/types/voting';
    > 5 | import { startAnchor } from 'solana-bankrun';
        | ^
      6 | import { describe, it } from 'node:test';
      7 | import { BankrunProvider } from "anchor-bankrun";
      8 |

      at Resolver._throwModNotFoundError (node_modules/jest-resolve/build/index.js:863:11)
      at Object.<anonymous> (node_modules/solana-bankrun/solana-bankrun/internal.js:141:29)
      at Object.<anonymous> (node_modules/solana-bankrun/solana-bankrun/index.ts:1:1)
      at Object.<anonymous> (anchor/tests/basic.test.ts:5:1)

Test Suites: 1 failed, 1 total
Tests:       0 total
Snapshots:   0 total
Time:        0.284 s
Ran all test suites.
```
