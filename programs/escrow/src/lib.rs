use anchor_lang::prelude::*;

declare_id!("814UtsDpgGyQXAVhEuBbyUX4YR6k8hFZLzSPdW2Sv3Wa");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        amount: u64,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.buyer = ctx.accounts.buyer.key();
        escrow.seller = ctx.accounts.seller.key();
        escrow.amount = amount;
        escrow.funded = false;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.funded = true;
        Ok(())
    }

    pub fn release(ctx: Context<Release>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        require!(escrow.funded, EscrowError::NotFunded);

        escrow.funded = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = buyer, space = 8 + 32 + 32 + 8 + 1)]
    pub escrow: Account<'info, EscrowData>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    pub seller: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub escrow: Account<'info, EscrowData>,
    pub buyer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Release<'info> {
    #[account(mut)]
    pub escrow: Account<'info, EscrowData>,
    pub seller: Signer<'info>,
}

#[account]
pub struct EscrowData {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub amount: u64,
    pub funded: bool,
}

#[error_code]
pub enum EscrowError {
    #[msg("Escrow not funded")]
    NotFunded,
}
