use anchor_lang::prelude::*;

declare_id!("3oAeHidCBZboHj82aP1AhbZbCc8TNRjvkQaDZQJyJAt3");

#[program]
pub mod user_profile {
    use super::*;

    pub fn initialize_profile(ctx: Context<InitializeProfile>, username: String, bio: String) -> Result<()> {
        require!(username.len() <= Profile::MAX_USERNAME, ProfileError::UsernameTooLong);
        require!(bio.len() <= Profile::MAX_BIO, ProfileError::BioTooLong);

        let profile = &mut ctx.accounts.profile;

        profile.authority = ctx.accounts.user.key();
        profile.username = username;
        profile.bio = bio;
        profile.bump = ctx.bumps.profile;
        
        Ok(())
    }

    pub fn update_profile(ctx: Context<UpdateProfile>, username: String, bio: String) -> Result<()> {

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(
        init,
        payer = user,
        seeds = [b"profile",user.key().as_ref()],
        bump,
        space = Profile::MAX_SIZE,
    )]
    pub profile: Account<'info, Profile>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(
        mut,
        seeds = [b"profile", user.key().as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,
    pub user: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Profile {
    pub authority: Pubkey,
    pub bump: u8,
    #[max_len(32)]
    pub username: String,
    #[max_len(256)]
    pub bio: String,
}

impl Profile{
    pub const MAX_USERNAME: usize = 32;
    pub const MAX_BIO: usize = 256;
    pub const MAX_SIZE: usize = 8 + 32 + 1 + 4 + Self::MAX_USERNAME + 4 + Self::MAX_BIO;
}

#[error_code]
pub enum ProfileError {
    UsernameTooLong,
    BioTooLong,
    Unauthorized,
}


