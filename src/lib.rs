use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod tienda_cosmeticos {
    use super::*;

    //////////////////////////// Crear Tienda ////////////////////////////

    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {

        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let cosmeticos: Vec<Cosmetico> = Vec::new();

        context.accounts.tienda.set_inner(TiendaCosmeticos {
            owner: owner_id,
            nombre,
            cosmeticos,
        });

        Ok(())
    }

    //////////////////////////// Agregar Cosmetico ////////////////////////////

    pub fn agregar_cosmetico(
        context: Context<NuevoCosmetico>,
        nombre: String,
        marca: String,
        precio: u32
    ) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let cosmetico = Cosmetico {
            nombre,
            marca,
            precio,
            disponible: true,
        };

        context.accounts.tienda.cosmeticos.push(cosmetico);

        Ok(())
    }

    //////////////////////////// Eliminar Cosmetico ////////////////////////////

    pub fn eliminar_cosmetico(context: Context<NuevoCosmetico>, nombre: String) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let cosmeticos = &mut context.accounts.tienda.cosmeticos;

        for i in 0..cosmeticos.len() {

            if cosmeticos[i].nombre == nombre {

                cosmeticos.remove(i);
                msg!("Cosmetico {} eliminado!", nombre);

                return Ok(());

            }

        }

        Err(Errores::CosmeticoNoExiste.into())
    }

    //////////////////////////// Ver Cosmeticos ////////////////////////////

    pub fn ver_cosmeticos(context: Context<NuevoCosmetico>) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "Lista de cosmeticos: {:#?}",
            context.accounts.tienda.cosmeticos
        );

        Ok(())
    }

    //////////////////////////// Alternar Disponibilidad ////////////////////////////

    pub fn alternar_estado(context: Context<NuevoCosmetico>, nombre: String) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let cosmeticos = &mut context.accounts.tienda.cosmeticos;

        for i in 0..cosmeticos.len() {

            let estado = cosmeticos[i].disponible;

            if cosmeticos[i].nombre == nombre {

                let nuevo_estado = !estado;

                cosmeticos[i].disponible = nuevo_estado;

                msg!(
                    "El cosmetico {} ahora tiene disponibilidad: {}",
                    nombre,
                    nuevo_estado
                );

                return Ok(());

            }

        }

        Err(Errores::CosmeticoNoExiste.into())
    }

}

//////////////////////////// Errores ////////////////////////////

#[error_code]
pub enum Errores {

    #[msg("Error, no eres el propietario de la tienda")]
    NoEresElOwner,

    #[msg("Error, el cosmetico no existe")]
    CosmeticoNoExiste,

}

//////////////////////////// Cuenta Tienda ////////////////////////////

#[account]
#[derive(InitSpace)]

pub struct TiendaCosmeticos {

    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    cosmeticos: Vec<Cosmetico>,

}

//////////////////////////// Struct Cosmetico ////////////////////////////

#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    Clone,
    InitSpace,
    PartialEq,
    Debug
)]

pub struct Cosmetico {

    #[max_len(60)]
    nombre: String,

    #[max_len(40)]
    marca: String,

    precio: u32,

    disponible: bool,

}

//////////////////////////// Contexto Crear Tienda ////////////////////////////

#[derive(Accounts)]

pub struct NuevaTienda<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = TiendaCosmeticos::INIT_SPACE + 8,
        seeds = [b"tienda_cosmeticos", owner.key().as_ref()],
        bump
    )]

    pub tienda: Account<'info, TiendaCosmeticos>,

    pub system_program: Program<'info, System>,

}

//////////////////////////// Contexto Cosmeticos ////////////////////////////

#[derive(Accounts)]

pub struct NuevoCosmetico<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub tienda: Account<'info, TiendaCosmeticos>,

}
