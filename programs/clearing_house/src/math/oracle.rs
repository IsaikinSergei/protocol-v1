use crate::error::{ClearingHouseResult, ErrorCode};
use crate::math::amm;
use crate::state::market::AMM;
use crate::state::state::OracleGuardRails;
use anchor_lang::prelude::AccountInfo;
use solana_program::clock::Slot;

pub fn block_liquidation(
    amm: &AMM,
    account_infos: &[AccountInfo],
    clock_slot: Slot,
    guard_rails: &OracleGuardRails,
) -> ClearingHouseResult<bool> {
    if !guard_rails.use_for_liquidations {
        return Ok(true);
    }

    let oracle_account_info = account_infos
        .iter()
        .find(|account_info| account_info.key.eq(&amm.oracle))
        .ok_or(ErrorCode::OracleNotFound)?;

    return block_operation(amm, oracle_account_info, clock_slot, guard_rails);
}

pub fn block_funding_rate_update(
    amm: &AMM,
    oracle_account_info: &AccountInfo,
    clock_slot: Slot,
    guard_rails: &OracleGuardRails,
) -> ClearingHouseResult<bool> {
    return block_operation(amm, oracle_account_info, clock_slot, guard_rails);
}

fn block_operation(
    amm: &AMM,
    oracle_account_info: &AccountInfo,
    clock_slot: Slot,
    guard_rails: &OracleGuardRails,
) -> ClearingHouseResult<bool> {
    let oracle_is_valid =
        amm::is_oracle_valid(amm, oracle_account_info, clock_slot, &guard_rails.validity)?;
    let oracle_mark_spread_pct =
        amm::calculate_oracle_mark_spread_pct(&amm, &oracle_account_info, 0, clock_slot)?;
    let is_oracle_mark_too_divergent =
        amm::is_oracle_mark_too_divergent(oracle_mark_spread_pct, &guard_rails.price_divergence)?;

    return Ok(!oracle_is_valid || is_oracle_mark_too_divergent);
}