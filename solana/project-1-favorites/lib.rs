use anchor_lang::prelude::*;

declare_id!("EKJ3pF5v8XYxri1Bf8iJdgMSXcbjki3JPsJdvGLBdLFv"); // program id = an address

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8; // a discriminator is written to every account on the Blockchain - 8 bytes

#[program] // macro
           // regular Rust module
pub mod favorites {
    use super::*; // uses the imported stuff

    // a function
    pub fn set_favorites(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greetings from {}", context.program_id); // an anchor message macro
        let user_public_key = context.accounts.user.key(); // variable created to show the User's PubKey

        msg!(
            "User {}'s, favorite number is {}, favorite color is {} and their hobbies are {:?}",
            user_public_key,
            number,
            color,
            hobbies
        );

        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

#[account] // account macro
#[derive(InitSpace)] // derive macro which gives all instances of Favorites the total space of all the items within the struct
                     // structs are used to group related data together.
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)] // nested (items, length of string)
    pub hobbies: Vec<String>, // bit like an array
}

#[derive(Accounts)] // So Anchor knows this is using the Accounts Struct
                    // set of favorites accounts
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // 'info = lifetime

    #[account(
        init_if_needed, // make the account if it doesn't exist
        payer = user, // who pays = user (signer)
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE, // how much space is required
        seeds = [b"favorites", user.key().as_ref()], // program derived address (PDA) - key::value store
        // The above is a safeguard in Anchor whereby only the User can sign a transaction
        bump // used to calculate the seeds?!
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>, // System Program = Token Program and again a lifetime is used
}
