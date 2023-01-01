use anchor_lang::prelude::*; // X.I - standard anchor import
use std::mem::size_of; // X.I - standard size of import

declare_id!("3A6B3ELfxPn7M1mbof29kr9KiejFzuY488T9aTYZi6Cs"); // declare program ID

#[program] // X.P - declare program name
mod hyperion {
    use super::*;
    pub fn tag_wallet(ctx: Context<TagWallet>, wallet: Pubkey, label: String) -> Result<()> {
        let tag = &mut ctx.accounts.tag;
        let clock: Clock = Clock::get().unwrap();

        // Handle Error - Label Error
        if label.chars().count() > 16 {
            return Err(ErrorCode::LabelTooLong.into());
        }

        // Handle Error - Votes Error
        if tag.votes > 0 {
            return Err(ErrorCode::NonZeroVotes.into());
        }

        tag.wallet = wallet;
        tag.label = label;
        tag.votes = 0;
        tag.timestamp = clock.unix_timestamp;

        msg!("Tagged wallet: {}!", tag.wallet); // Will show tagged wallet in logs.
        msg!("Tag label: {}!", tag.label); // Will show tagged wallet in logs.
        msg!("Timestamp: {}!", tag.timestamp); // Will show tagged wallet in logs.
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TagWallet<'info> {
    #[account(init, 
              seeds = [b"tag".as_ref(), wallet.key().as_ref(), signer.key().as_ref()],
              bump,
              payer = signer,
              space = size_of::<Tag>()
             )]
    pub tag: Account<'info, Tag>,
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: account wallet is safe because it simple stores the tagged wallet string.
    #[account(mut)]
    pub wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

// X.S -  define the struct of each 'Tag' account.
#[account]
#[derive(Default)]
pub struct Tag {
    pub wallet: Pubkey,
    pub label: String,
    pub votes: i64,
    pub timestamp: i64,
}

// X.E Errors
#[error_code]
pub enum ErrorCode {
    #[msg("The provided wallet tag is too long - it should be a maximum of 16 chars")]
    LabelTooLong,
    #[msg("Non zero votes - attempted to tag wallet with pre-tagged votes.")]
    NonZeroVotes,
}
