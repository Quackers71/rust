#![allow(clippy::result_large_err)] // ???

use anchor_lang::prelude::*; // imports all the good stuff!

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H"); // program id = an address

#[program] // program macro
    // Rust module
pub mod voting {
    use super::*; // uses the imported stuff!
    // init poll function
    pub fn initialize_poll(ctx: Context<InitializePoll>,
        poll_id: u64,
        description: String,
        poll_start: u64,
        poll_end: u64,) -> Result<()> {
        
        let poll = &mut ctx.accounts.poll;
        poll.poll_id = poll_id;
        poll.description = description;
        poll.poll_start = poll_start;
        poll.poll_end = poll_end;
        poll.candidate_amount = 0;
        Ok(())
    }

    pub fn initialize_candidate(ctx: Context<InitializeCandidate>,
                                candidate_name: String,
                                poll_id: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct InitializeCandidate<'info> {
    #[account(mut)] 
    pub signer: Signer<'info>,

    #[account(
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        init,
        payer = signer,
        space = 8 + Poll::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()],
        bump,
    )]

    pub candidate: Account<'info, Candidate>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}

#[derive(Accounts)] // derive Accounts macro 
#[instruction(poll_id: u64)] // instruction macro used for the PDA (seeds) below
    // Init Poll struct using lifetimes i.e. <'info>
pub struct InitializePoll<'info> {
    #[account(mut)] // make it mutable (so it can be changed)
    pub signer: Signer<'info>,
    #[account(
        init, // To create the account if required
        payer = signer, // payer is the signer
        space = 8 + Poll::INIT_SPACE, // 8 bytes (reserved) +  Poll::[#dervice(InitSpace)] below, sets how much space is required
        seeds = [poll_id.to_le_bytes().as_ref()], // seeds = program derived address (PDA), poll_id is required to tie it to the account
        bump, // Used to calculate the seeds...
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>, // System Program = Token Program // this is required for the account macro
}

#[account] // uses the account macro
#[derive(InitSpace)] // derive macro which gives all instances of Poll the total space of all the items within the struct
    // The Poll struct is used to group the related data together
pub struct Poll {
    pub poll_id: u64,
    #[max_len(280)] // max_len macro to set the length of the String
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}