#![allow(non_snake_case)]
use super::error::SwitchboardError;
#[allow(unaligned_references)]
use crate::*;
use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::Instruction;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use std::cell::Ref;

#[zero_copy]
pub struct EcvrfProofZC {
    pub Gamma: EdwardsPointZC, // RistrettoPoint
    pub c: Scalar,
    pub s: Scalar,
}
impl Default for EcvrfProofZC {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// The `Scalar` struct holds an integer \\(s < 2\^{255} \\) which
/// represents an element of \\(\mathbb Z / \ell\\).
#[zero_copy]
pub struct Scalar {
    /// `bytes` is a little-endian byte encoding of an integer representing a scalar modulo the
    /// group order.
    ///
    /// # Invariant
    ///
    /// The integer representing this scalar must be bounded above by \\(2\^{255}\\), or
    /// equivalently the high bit of `bytes[31]` must be zero.
    ///
    /// This ensures that there is room for a carry bit when computing a NAF representation.
    //
    // XXX This is pub(crate) so we can write literal constants.  If const fns were stable, we could
    //     make the Scalar constructors const fns and use those instead.
    pub(crate) bytes: [u8; 32],
}
unsafe impl Pod for Scalar {}
unsafe impl Zeroable for Scalar {}

/// A `FieldElement51` represents an element of the field
/// \\( \mathbb Z / (2\^{255} - 19)\\).
///
/// In the 64-bit implementation, a `FieldElement` is represented in
/// radix \\(2\^{51}\\) as five `u64`s; the coefficients are allowed to
/// grow up to \\(2\^{54}\\) between reductions modulo \\(p\\).
///
/// # Note
///
/// The `curve25519_dalek::field` module provides a type alias
/// `curve25519_dalek::field::FieldElement` to either `FieldElement51`
/// or `FieldElement2625`.
///
/// The backend-specific type `FieldElement51` should not be used
/// outside of the `curve25519_dalek::field` module.
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct FieldElement51(pub(crate) [u64; 5]);
unsafe impl Pod for FieldElement51 {}
unsafe impl Zeroable for FieldElement51 {}

#[zero_copy]
pub struct FieldElementZC {
    pub(crate) bytes: [u64; 5],
}
impl Default for FieldElementZC {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
unsafe impl Pod for FieldElementZC {}
unsafe impl Zeroable for FieldElementZC {}
impl Into<FieldElementZC> for FieldElement51 {
    fn into(self) -> FieldElementZC {
        FieldElementZC { bytes: self.0 }
    }
}
impl Into<FieldElement51> for FieldElementZC {
    fn into(self) -> FieldElement51 {
        FieldElement51(self.bytes)
    }
}

/// A `CompletedPoint` is a point \\(((X:Z), (Y:T))\\) on the \\(\mathbb
/// P\^1 \times \mathbb P\^1 \\) model of the curve.
/// A point (x,y) in the affine model corresponds to \\( ((x:1),(y:1))
/// \\).
///
/// More details on the relationships between the different curve models
/// can be found in the module-level documentation.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CompletedPoint {
    pub X: FieldElement51,
    pub Y: FieldElement51,
    pub Z: FieldElement51,
    pub T: FieldElement51,
}
#[zero_copy]
pub struct CompletedPointZC {
    pub X: FieldElementZC,
    pub Y: FieldElementZC,
    pub Z: FieldElementZC,
    pub T: FieldElementZC,
}
impl Default for CompletedPointZC {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
unsafe impl Pod for CompletedPoint {}
unsafe impl Zeroable for CompletedPoint {}
impl Into<CompletedPointZC> for CompletedPoint {
    fn into(self) -> CompletedPointZC {
        CompletedPointZC {
            X: self.X.into(),
            Y: self.Y.into(),
            Z: self.Z.into(),
            T: self.T.into(),
        }
    }
}
impl Into<CompletedPoint> for CompletedPointZC {
    fn into(self) -> CompletedPoint {
        CompletedPoint {
            X: self.X.into(),
            Y: self.Y.into(),
            Z: self.Z.into(),
            T: self.T.into(),
        }
    }
}

/// An `EdwardsPoint` represents a point on the Edwards form of Curve25519.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdwardsPoint {
    pub(crate) X: FieldElement51,
    pub(crate) Y: FieldElement51,
    pub(crate) Z: FieldElement51,
    pub(crate) T: FieldElement51,
}
#[zero_copy]
pub struct EdwardsPointZC {
    pub(crate) X: FieldElementZC,
    pub(crate) Y: FieldElementZC,
    pub(crate) Z: FieldElementZC,
    pub(crate) T: FieldElementZC,
}
impl Default for EdwardsPointZC {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// A `ProjectivePoint` is a point \\((X:Y:Z)\\) on the \\(\mathbb
/// P\^2\\) model of the curve.
/// A point \\((x,y)\\) in the affine model corresponds to
/// \\((x:y:1)\\).
///
/// More details on the relationships between the different curve models
/// can be found in the module-level documentation.
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct ProjectivePoint {
    pub X: FieldElement51,
    pub Y: FieldElement51,
    pub Z: FieldElement51,
}
#[zero_copy]
pub struct ProjectivePointZC {
    pub(crate) X: FieldElementZC,
    pub(crate) Y: FieldElementZC,
    pub(crate) Z: FieldElementZC,
}
impl Default for ProjectivePointZC {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
unsafe impl Pod for ProjectivePoint {}
unsafe impl Zeroable for ProjectivePoint {}
impl Into<ProjectivePointZC> for ProjectivePoint {
    fn into(self) -> ProjectivePointZC {
        ProjectivePointZC {
            X: self.X.into(),
            Y: self.Y.into(),
            Z: self.Z.into(),
        }
    }
}
impl Into<ProjectivePoint> for ProjectivePointZC {
    fn into(self) -> ProjectivePoint {
        ProjectivePoint {
            X: self.X.into(),
            Y: self.Y.into(),
            Z: self.Z.into(),
        }
    }
}

#[zero_copy]
pub struct EcvrfIntermediate {
    pub r: FieldElementZC,
    pub N_s: FieldElementZC,
    pub D: FieldElementZC,
    pub t13: FieldElementZC,
    pub t15: FieldElementZC,
}
unsafe impl Pod for EcvrfIntermediate {}
unsafe impl Zeroable for EcvrfIntermediate {}

#[allow(non_snake_case)]
#[zero_copy]
pub struct VrfBuilder {
    pub producer: Pubkey,
    pub status: VrfStatus,
    pub repr_proof: [u8; 80],
    pub proof: EcvrfProofZC,
    pub Y_point: Pubkey,
    pub stage: u32,
    pub stage1_out: EcvrfIntermediate,
    pub R_1: EdwardsPointZC, // Ristretto
    pub R_2: EdwardsPointZC, // Ristretto
    pub stage3_out: EcvrfIntermediate,
    pub H_point: EdwardsPointZC, // Ristretto
    pub s_reduced: Scalar,
    pub Y_point_builder: [FieldElementZC; 3],
    pub Y_ristretto_point: EdwardsPointZC, // Ristretto
    pub mul_round: u8,
    pub hash_points_round: u8,
    pub mul_tmp1: CompletedPointZC,
    pub U_point1: EdwardsPointZC, // Ristretto
    pub U_point2: EdwardsPointZC, // Ristretto
    pub V_point1: EdwardsPointZC, // Ristretto
    pub V_point2: EdwardsPointZC, // Ristretto
    pub U_point: EdwardsPointZC,  // Ristretto
    pub V_point: EdwardsPointZC,  // Ristretto
    pub u1: FieldElementZC,
    pub u2: FieldElementZC,
    pub invertee: FieldElementZC,
    pub y: FieldElementZC,
    pub z: FieldElementZC,
    pub p1_bytes: [u8; 32],
    pub p2_bytes: [u8; 32],
    pub p3_bytes: [u8; 32],
    pub p4_bytes: [u8; 32],
    pub c_prime_hashbuf: [u8; 16],
    pub m1: FieldElementZC,
    pub m2: FieldElementZC,
    pub tx_remaining: u32,
    pub verified: bool,
    pub result: [u8; 32],
}
impl Default for VrfBuilder {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[zero_copy]
pub struct AccountMetaZC {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct AccountMetaBorsh {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[zero_copy]
pub struct CallbackZC {
    pub program_id: Pubkey,
    pub accounts: [AccountMetaZC; 32],
    pub accounts_len: u32,
    pub ix_data: [u8; 1024],
    pub ix_data_len: u32,
}
impl Default for CallbackZC {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Callback {
    pub program_id: Pubkey,
    pub accounts: Vec<AccountMetaBorsh>,
    pub ix_data: Vec<u8>,
}

#[zero_copy]
pub struct VrfRound {
    pub alpha: [u8; 256],
    pub alpha_len: u32,
    pub request_slot: u64,
    pub request_timestamp: i64,
    pub result: [u8; 32],
    pub num_verified: u32,
    pub _ebuf: [u8; 256], // Buffer for future info
}
impl Default for VrfRound {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum VrfStatus {
    StatusNone,
    StatusRequesting,
    StatusVerifying,
    StatusVerified,
    StatusCallbackSuccess,
    StatusVerifyFailure,
}

// #[derive(Copy, Clone)]
#[zero_copy]
pub struct VrfAccountData {
    pub status: VrfStatus,
    pub counter: u128,
    pub authority: Pubkey,
    pub oracle_queue: Pubkey,
    pub escrow: Pubkey,
    pub callback: CallbackZC,
    pub batch_size: u32,
    pub builders: [VrfBuilder; 8],
    pub builders_len: u32,
    pub test_mode: bool,
    // pub last_verified_round: VrfRound,
    pub current_round: VrfRound,
    //
    pub _ebuf: [u8; 1024], // Buffer for future info
}
impl Default for VrfAccountData {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl VrfAccountData {
    pub fn new<'info>(
        switchboard_vrf: &'info AccountInfo,
    ) -> Result<Ref<'info, VrfAccountData>, ProgramError> {
        let data = switchboard_vrf.try_borrow_data()?;

        let mut disc_bytes = [0u8; 8];
        disc_bytes.copy_from_slice(&data[..8]);
        if disc_bytes != VrfAccountData::discriminator() {
            msg!("{:?}", disc_bytes);
            return Err(SwitchboardError::AccountDiscriminatorMismatch.into());
        }

        Ok(Ref::map(data, |data| bytemuck::from_bytes(&data[8..])))
    }

    pub fn get_current_randomness_round_id(&self) -> u128 {
        self.counter
    }

    pub fn get_result(&self) -> Result<[u8; 32], ProgramError> {
        if self.current_round.result == [0u8; 32] {
            return Err(SwitchboardError::VrfEmptyError.into());
        }
        Ok(self.current_round.result)
    }

    // Need to log and update to actual value
    fn discriminator() -> [u8; 8] {
        return [101, 35, 62, 239, 103, 151, 6, 18];
    }
}
unsafe impl Pod for VrfAccountData {}
unsafe impl Zeroable for VrfAccountData {}

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
    #[account(mut, 
        seeds = [
            b"PermissionAccountData",
            queue_authority.key().as_ref(),
            oracle_queue.key().as_ref(),
            vrf.key().as_ref()
        ],
        bump = params.permission_bump
    )]
    pub permission: AccountInfo<'info>,
    #[account(mut, constraint = escrow.owner == program_state.key())]
    pub escrow: Account<'info, TokenAccount>,
    #[account(mut, constraint = payer_wallet.owner == payer_authority.key())]
    pub payer_wallet: Account<'info, TokenAccount>,
    #[account(signer)]
    pub payer_authority: AccountInfo<'info>,
    pub recent_blockhashes: AccountInfo<'info>,
    #[account(seeds = [b"STATE"], bump = params.state_bump)]
    pub program_state: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct VrfRequestRandomnessParams {
    pub permission_bump: u8,
    pub state_bump: u8,
}

impl<'info> VrfRequestRandomness<'info> {
    fn discriminator() -> [u8; 8] {
        return [230, 121, 14, 164, 28, 222, 117, 118];
    }

    pub fn get_instruction(
        &self,
        program_id: Pubkey,
        params: VrfRequestRandomnessParams,
    ) -> Result<Instruction, ProgramError> {
        let accounts = self.to_account_metas(None);

        let mut data: Vec<u8> = VrfRequestRandomness::discriminator().try_to_vec()?;
        let mut param_vec: Vec<u8> = params.try_to_vec()?;
        data.append(&mut param_vec);

        let instruction = Instruction::new_with_bytes(program_id, &data, accounts);
        Ok(instruction)
    }

    pub fn invoke(
        &self,
        program: AccountInfo<'info>,
        state_bump: u8,
        permission_bump: u8,
    ) -> ProgramResult {
        let cpi_params = VrfRequestRandomnessParams {
            permission_bump: permission_bump,
            state_bump: state_bump,
        };
        let instruction = self.get_instruction(program.key.clone(), cpi_params)?;
        let account_infos = self.to_account_infos();

        invoke(&instruction, &account_infos[..])
    }

    pub fn invoke_signed(
        &self,
        program: AccountInfo<'info>,
        state_bump: u8,
        permission_bump: u8,
        signer_seeds: &[&[&[u8]]],
    ) -> ProgramResult {
        let cpi_params = VrfRequestRandomnessParams {
            permission_bump: permission_bump,
            state_bump: state_bump,
        };
        let instruction = self.get_instruction(program.key.clone(), cpi_params)?;
        let account_infos = self.to_account_infos();

        invoke_signed(&instruction, &account_infos[..], signer_seeds)
    }

    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        return vec![
            self.authority.clone(),
            self.vrf.clone(),
            self.oracle_queue.clone(),
            self.queue_authority.clone(),
            self.data_buffer.clone(),
            self.permission.clone(),
            self.escrow.to_account_info().clone(),
            self.payer_wallet.to_account_info().clone(),
            self.payer_authority.clone(),
            self.recent_blockhashes.clone(),
            self.program_state.clone(),
            self.token_program.clone(),
        ];
    }

    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        return vec![
            AccountMeta {
                pubkey: self.authority.key.clone(),
                is_signer: true, // overwrite, authority has to sign
                is_writable: self.authority.is_writable,
            },
            AccountMeta {
                pubkey: self.vrf.key.clone(),
                is_signer: self.vrf.is_signer,
                is_writable: self.vrf.is_writable,
            },
            AccountMeta {
                pubkey: self.oracle_queue.key.clone(),
                is_signer: self.oracle_queue.is_signer,
                is_writable: self.oracle_queue.is_writable,
            },
            AccountMeta {
                pubkey: self.queue_authority.key.clone(),
                is_signer: self.queue_authority.is_signer,
                is_writable: self.queue_authority.is_writable,
            },
            AccountMeta {
                pubkey: self.data_buffer.key.clone(),
                is_signer: self.data_buffer.is_signer,
                is_writable: self.data_buffer.is_writable,
            },
            AccountMeta {
                pubkey: self.permission.key.clone(),
                is_signer: self.permission.is_signer,
                is_writable: self.permission.is_writable,
            },
            AccountMeta {
                pubkey: self.escrow.to_account_info().key.clone(),
                is_signer: self.escrow.to_account_info().is_signer,
                is_writable: self.escrow.to_account_info().is_writable,
            },
            AccountMeta {
                pubkey: self.payer_wallet.to_account_info().key.clone(),
                is_signer: self.payer_wallet.to_account_info().is_signer,
                is_writable: self.payer_wallet.to_account_info().is_writable,
            },
            AccountMeta {
                pubkey: self.payer_authority.key.clone(),
                is_signer: self.payer_authority.is_signer,
                is_writable: self.payer_authority.is_writable,
            },
            AccountMeta {
                pubkey: self.recent_blockhashes.key.clone(),
                is_signer: self.recent_blockhashes.is_signer,
                is_writable: self.recent_blockhashes.is_writable,
            },
            AccountMeta {
                pubkey: self.program_state.key.clone(),
                is_signer: self.program_state.is_signer,
                is_writable: self.program_state.is_writable,
            },
            AccountMeta {
                pubkey: self.token_program.key.clone(),
                is_signer: self.token_program.is_signer,
                is_writable: self.token_program.is_writable,
            },
        ];
    }
}
