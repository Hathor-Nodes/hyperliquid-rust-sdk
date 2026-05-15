use serde::Deserialize;

use alloy::primitives::Address;

use crate::{
    info::{AssetPosition, Level, MarginSummary},
    DailyUserVlm, Delta, FeeSchedule, Leverage, OrderInfo, Referrer, ReferrerState,
    UserTokenBalance,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserStateResponse {
    pub asset_positions: Vec<AssetPosition>,
    pub cross_margin_summary: MarginSummary,
    pub margin_summary: MarginSummary,
    pub withdrawable: String,
}

#[derive(Deserialize, Debug)]
pub struct UserTokenBalanceResponse {
    pub balances: Vec<UserTokenBalance>,
}

/// Per-coin spot balance, returned by `InfoClient::spot_user_state`.
///
/// Mirrors the Python SDK's `dict` shape (`{"coin", "hold", "total"}`) so
/// downstream code computing `perps + spot USDC` for equity (CLAUDE.md
/// §Position sizing, incident 2026-04-22) can read the same fields it would
/// in Python. All numeric values are wire-encoded as strings by HL.
///
/// The HL API also returns an `entryNtl` field; serde ignores unknown fields
/// by default, so it is intentionally omitted here for parity with Python.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SpotBalance {
    pub coin: String,
    pub hold: String,
    pub total: String,
}

/// Response shape for `InfoClient::spot_user_state` — the spot clearinghouse
/// equivalent of `UserStateResponse`. Equity computations sum
/// `marginSummary.accountValue` (perps) plus `balances[coin="USDC"].total`
/// (this struct, spot).
#[derive(Deserialize, Debug, Clone)]
pub struct SpotUserStateResponse {
    pub balances: Vec<SpotBalance>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Canned fixture matches the real public-API shape returned by
    /// `POST /info` with `{"type": "spotClearinghouseState", "user": ...}`.
    /// The `entryNtl` key is present on the wire; deserialisation drops it.
    #[test]
    fn spot_user_state_response_deserialises_canned_fixture() {
        let payload = r#"{
            "balances": [
                {"coin": "USDC", "hold": "0.0", "total": "1234.56", "entryNtl": "0.0"},
                {"coin": "PURR", "hold": "0.0", "total": "42.0", "entryNtl": "0.0"}
            ]
        }"#;

        let parsed: SpotUserStateResponse =
            serde_json::from_str(payload).expect("canned fixture must deserialise");
        assert_eq!(parsed.balances.len(), 2);
        assert_eq!(parsed.balances[0].coin, "USDC");
        assert_eq!(parsed.balances[0].total, "1234.56");
        assert_eq!(parsed.balances[0].hold, "0.0");
        assert_eq!(parsed.balances[1].coin, "PURR");
        assert_eq!(parsed.balances[1].total, "42.0");
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserFeesResponse {
    pub active_referral_discount: String,
    pub daily_user_vlm: Vec<DailyUserVlm>,
    pub fee_schedule: FeeSchedule,
    pub user_add_rate: String,
    pub user_cross_rate: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrdersResponse {
    pub coin: String,
    pub limit_px: String,
    pub oid: u64,
    pub side: String,
    pub sz: String,
    pub timestamp: u64,
    pub cloid: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserFillsResponse {
    pub closed_pnl: String,
    pub coin: String,
    pub crossed: bool,
    pub dir: String,
    pub hash: String,
    pub oid: u64,
    pub px: String,
    pub side: String,
    pub start_position: String,
    pub sz: String,
    pub time: u64,
    pub fee: String,
    pub tid: u64,
    pub fee_token: String,
    pub twap_id: Option<u64>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FundingHistoryResponse {
    pub coin: String,
    pub funding_rate: String,
    pub premium: String,
    pub time: u64,
}

#[derive(Deserialize, Debug)]
pub struct UserFundingResponse {
    pub time: u64,
    pub hash: String,
    pub delta: Delta,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct L2SnapshotResponse {
    pub coin: String,
    pub levels: Vec<Vec<Level>>,
    pub time: u64,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradesResponse {
    pub coin: String,
    pub side: String,
    pub px: String,
    pub sz: String,
    pub time: u64,
    pub hash: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct CandlesSnapshotResponse {
    #[serde(rename = "t")]
    pub time_open: u64,
    #[serde(rename = "T")]
    pub time_close: u64,
    #[serde(rename = "s")]
    pub coin: String,
    #[serde(rename = "i")]
    pub candle_interval: String,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "c")]
    pub close: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "v")]
    pub vlm: String,
    #[serde(rename = "n")]
    pub num_trades: u64,
}

#[derive(Deserialize, Debug)]
pub struct OrderStatusResponse {
    pub status: String,
    /// `None` if the order is not found
    #[serde(default)]
    pub order: Option<OrderInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReferralResponse {
    pub referred_by: Option<Referrer>,
    pub cum_vlm: String,
    pub unclaimed_rewards: String,
    pub claimed_rewards: String,
    pub referrer_state: ReferrerState,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActiveAssetDataResponse {
    pub user: Address,
    pub coin: String,
    pub leverage: Leverage,
    pub max_trade_szs: Vec<String>,
    pub available_to_trade: Vec<String>,
    pub mark_px: String,
}
