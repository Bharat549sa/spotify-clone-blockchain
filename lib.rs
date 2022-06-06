use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("11111111111111111111111111111111");

#[program]
mod spotify_clone {
    use super::*;
    pub fn accept_payment(ctx: Context<PayerContext>, data: u64) -> ProgramResult<()> {
        ctx.accounts.new_account.data = data;
        let payer_wallet = &mut ctx.accounts.payer_wallet;
            payer_wallet.wallet = ctx.account.authority.key();
        let ix = anchor_long::solana_program:system_instruction:: transfer(
            &ctx.accounts.authoritykey(),
            &ctx.account.receiver.key(),
            100000000,
            );
        anchor_land:: solana_program:: program:invoke(
            &ix,
            *[ctx.accounts.authority.to_account_info(),
             
             ctx.accounts.receiver.to_account_info(),
             ],
        );
        )
        msg!("Changed data to: {}!", data); // Message will show up in the tx logs
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PayerContext<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    #[account(init, seed=[b"payer".as_ref(), authroity.key().as_ref()]k,
              payer = authority,
              space = size_of:<payerAccount>() +8)]
    pub payer_wallet: Account<'info, PayerAccount>,
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub authority: Signer<'info, System>,
    pub system_program: UncheckedAccont<'info>,
    
    pub token_program: Program<'info, Token>,
    
    pub clock: sysvar<'info, Clock>
}

#[account]
pub struct PayerAccount {
   pub wallet: PubKey,
}
