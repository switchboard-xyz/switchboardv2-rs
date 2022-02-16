use anchor_lang::prelude::*;

pub mod aggregator;
pub mod decimal;
pub mod error;
pub mod vrf;

pub use aggregator::AggregatorAccountData;
pub use vrf::VrfAccountData;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod switchboard_v2 {
    pub use super::*;

    pub fn vrf_request_randomness(
        ctx: Context<VrfRequestRandomness>,
        params: VrfRequestRandomnessParams,
    ) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(params: VrfRequestRandomnessParams)] // rpc parameters hint
pub struct VrfRequestRandomness<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub vrf: AccountInfo<'info>,
    #[account(mut)]
    pub oracle_queue: AccountInfo<'info>,
    pub queue_authority: AccountInfo<'info>,
    pub data_buffer: AccountInfo<'info>,
    #[account(mut)]
    pub permission: AccountInfo<'info>,
    #[account(mut)]
    pub escrow: AccountInfo<'info>,
    #[account(mut)]
    pub payer_wallet: AccountInfo<'info>,
    #[account(signer)]
    pub payer_authority: AccountInfo<'info>,
    pub recent_blockhashes: AccountInfo<'info>,
    pub program_state: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct VrfRequestRandomnessParams {
    pub permission_bump: u8,
    pub state_bump: u8,
}
