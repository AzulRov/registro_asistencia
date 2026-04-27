use anchor_lang::prelude::*;

declare_id!("D8Tzg8afoUnq2WiXw2dQEGSC2ENH8QYH122yMeyrbazY");

#[program]
pub mod registro_asistencia {
    use super::*;

    pub fn crear_evento(ctx: Context<CrearEvento>, nombre_evento: String) -> Result<()> {
        let evento = &mut ctx.accounts.evento;
        evento.nombre_evento = nombre_evento;
        evento.organizador = *ctx.accounts.organizador.key;
        evento.total_registros = 0;
        Ok(())
    }

    pub fn registrar_participante(ctx: Context<RegistrarParticipante>, nombre: String) -> Result<()> {
        let registro = &mut ctx.accounts.registro;
        let evento = &mut ctx.accounts.evento;

        registro.nombre = nombre;
        registro.usuario = *ctx.accounts.usuario.key;
        registro.asistio = false;

        evento.total_registros += 1;
        Ok(())
    }

    pub fn marcar_asistencia(ctx: Context<MarcarAsistencia>) -> Result<()> {
        let registro = &mut ctx.accounts.registro;
        registro.asistio = true;
        Ok(())
    }
}

#[account]
pub struct Evento {
    pub nombre_evento: String,
    pub organizador: Pubkey,
    pub total_registros: u64,
}

#[account]
pub struct Registro {
    pub nombre: String,
    pub usuario: Pubkey,
    pub asistio: bool,
}

#[derive(Accounts)]
pub struct CrearEvento<'info> {
    #[account(init, payer = organizador, space = 8 + 50 + 32 + 8)]
    pub evento: Account<'info, Evento>,

    #[account(mut)]
    pub organizador: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegistrarParticipante<'info> {
    #[account(mut)]
    pub evento: Account<'info, Evento>,

    #[account(init, payer = usuario, space = 8 + 50 + 32 + 1)]
    pub registro: Account<'info, Registro>,

    #[account(mut)]
    pub usuario: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MarcarAsistencia<'info> {
    #[account(mut)]
    pub registro: Account<'info, Registro>,
}
