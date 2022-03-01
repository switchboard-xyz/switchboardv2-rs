#[allow(unaligned_references)]
use super::decimal::SwitchboardDecimal;
use super::error::SwitchboardError;
use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};
use solana_program::pubkey::Pubkey;
use std::cell::Ref;
use std::cell::RefCell;
use bytemuck::{try_cast_slice, try_from_bytes};
use superslice::*;

#[zero_copy]
#[derive(Default)]
#[repr(packed)]
pub struct AggregatorHistoryRow {
    pub timestamp: i64,
    pub value: SwitchboardDecimal,
}
unsafe impl Pod for AggregatorHistoryRow {}
unsafe impl Zeroable for AggregatorHistoryRow {}

pub struct AggregatorHistoryBuffer<'a> {
    pub insertion_idx: usize,
    pub rows: Ref<'a, [AggregatorHistoryRow]>,
}
impl<'a> AggregatorHistoryBuffer<'a> {
    pub fn new(
        history_buffer: &'a AccountInfo,
    ) -> Result<AggregatorHistoryBuffer<'a>, ProgramError> {
        let data = history_buffer.try_borrow_data()?;

        let mut disc_bytes = [0u8; 8];
        disc_bytes.copy_from_slice(&data[..8]);
        if disc_bytes != *b"BUFFERxx" {
            return Err(SwitchboardError::AccountDiscriminatorMismatch.into());
        }
        let insertion_idx: u32 = try_from_bytes::<u32>(&data[8..12]).unwrap().clone();
        let rows = Ref::map(data, |data| try_cast_slice(&data[12..]).unwrap());
        return Ok(Self {
            insertion_idx: insertion_idx as usize,
            rows: rows
        })
    }

    pub fn lower_bound(&self, timestamp: i64) -> Option<AggregatorHistoryRow> {
        if self.rows[self.insertion_idx].timestamp == 0 {
            return None;
        }
        let lower = &self.rows[..self.insertion_idx + 1];
        let lahr = lower.lower_bound_by(|x| x.timestamp.cmp(&timestamp));
        if lahr != 0 {
            return Some(lower[lahr - 1]);
        }
        if self.insertion_idx + 1 < self.rows.len() &&
            self.rows[self.insertion_idx + 1].timestamp != 0 {
            let upper = &self.rows[self.insertion_idx + 1..];
            let uahr = upper.lower_bound_by(|x| x.timestamp.cmp(&timestamp));
            if uahr != 0 {
                return Some(upper[uahr - 1]);
            }
        }
        None
    }
}
