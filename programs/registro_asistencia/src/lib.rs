use anchor_lang::prelude::*;

declare_id!("D8Tzg8afoUnq2WiXW2qDEGSC2ENH8QYH122yMeyrbazY");

#[program]
pub mod registro_asistencia {
    use super::*;

    pub fn registrar(ctx: Context<Registrar>, nombre: String) -> Result<()> {
        let registro = &mut ctx.accounts.registro;
        registro.nombre = nombre;
        registro.usuario = *ctx.accounts.usuario.key;
        Ok(())
    }
}

#[account]
pub struct Registro {
    pub nombre: String,
    pub usuario: Pubkey,
}

#[derive(Accounts)]
pub struct Registrar<'info> {
    #[account(init, payer = usuario, space = 8 + 40 + 32)]
    pub registro: Account<'info, Registro>,

    #[account(mut)]
    pub usuario: Signer<'info>,

    pub system_program: Program<'info, System>,
}
