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

- Ran to Fix the anchor test missing module :
```
npm install --save-dev solana-bankrun jest
```

- Re-ran anchor test - Output (Still need to fix this stuff!)
```
anchor % anchor test --skip-local-validator --skip-deploy

---
Found a 'test' script in the Anchor.toml. Running it as a test suite!

Running test suite: "/Users/robq/repos/rust/solana/project-2-voting/voting-dapp/anchor/Anchor.toml"


 RUNS  tests/basic.test.ts
[2025-09-21T10:54:35.676951000Z INFO  solana_program_test] "voting" SBF program from target/deploy/voting.so, modified 2 hours, 26 minutes, 48 seconds, 750 ms, 783 µs and 462 ns ago
 FAIL  tests/basic.test.ts9000Z INFO  solana_program_test] "voting" SBF program from target/deploy/voting.so, modified 2 hours, 26 minutes, 48 seconds
  ● Test suite failed to run

    Your test suite must contain at least one test.

      at onResult (node_modules/@jest/core/build/index.js:1056:18)
      at node_modules/@jest/core/build/index.js:1126:165
      at node_modules/emittery/index.js:363:13
          at Array.map (<anonymous>)
      at Emittery.emit (node_modules/emittery/index.js:361:23)

Test Suites: 1 failed, 1 total
Tests:       0 total
Snapshots:   0 total
Time:        0.551 s
Ran all test suites.
[2025-09-21T10:54:35.693157000Z INFO  solana_program_test] Overriding account at JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H
[2025-09-21T10:54:35.713289000Z DEBUG solana_runtime::message_processor::stable_log] Program JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H invoke [1]
[2025-09-21T10:54:35.713375000Z DEBUG solana_runtime::message_processor::stable_log] Program log: Instruction: InitializePoll
[2025-09-21T10:54:35.713464000Z DEBUG solana_runtime::message_processor::stable_log] Program 11111111111111111111111111111111 invoke [2]
[2025-09-21T10:54:35.713480000Z DEBUG solana_runtime::message_processor::stable_log] Program 11111111111111111111111111111111 success
[2025-09-21T10:54:35.713510000Z DEBUG solana_runtime::message_processor::stable_log] Program JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H consumed 7612 of 200000 compute units
[2025-09-21T10:54:35.713516000Z DEBUG solana_runtime::message_processor::stable_log] Program JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H success

  ●  Cannot log after tests are done. Did you forget to wait for something async in your test?
    Attempted to log "{
      pollId: <BN: 1>,
      description: 'What is your favorite type of Peanut Butter?',
      pollStart: <BN: 0>,
      pollEnd: <BN: 7be1d1b3>,
      candidateAmount: <BN: 0>
    }".

    

      at console.log (node_modules/@jest/console/build/index.js:311:10)
      at TestContext.<anonymous> (anchor/tests/basic.test.ts:54:17)
          at async Promise.all (index 0)

▶ Voting
  ✔ Initialize Poll (41.332084ms)
✔ Voting (41.838666ms)
ℹ tests 1
ℹ suites 1
ℹ pass 1
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 73.6055
```
