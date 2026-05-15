use serde::{Deserialize, Serialize};

use crate::ws::sub_structs::*;

#[derive(Deserialize, Clone, Debug)]
pub struct Trades {
    pub data: Vec<Trade>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct L2Book {
    pub data: L2BookData,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AllMids {
    pub data: AllMidsData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub data: UserData,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserFills {
    pub data: UserFillsData,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Candle {
    pub data: CandleData,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OrderUpdates {
    pub data: Vec<OrderUpdate>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserFundings {
    pub data: UserFundingsData,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserNonFundingLedgerUpdates {
    pub data: UserNonFundingLedgerUpdatesData,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Notification {
    pub data: NotificationData,
}

#[derive(Deserialize, Clone, Debug)]
pub struct WebData2 {
    pub data: WebData2Data,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ActiveAssetCtx {
    pub data: ActiveAssetCtxData,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ActiveSpotAssetCtx {
    pub data: ActiveSpotAssetCtxData,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ActiveAssetData {
    pub data: ActiveAssetDataData,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Bbo {
    pub data: BboData,
}

#[cfg(test)]
mod tests {
    //! Round-trip tests for the `User` and `UserFundings` WSS message
    //! wrappers. The Hathor `InfoBridge::msg_to_value` translates these
    //! into `serde_json::Value` envelopes for downstream callbacks; the
    //! trailing-stop / partial-TP logic depends on the inner `fills` and
    //! `fundings` payloads reaching the callback (Hathor #215).
    use super::*;
    use crate::ws::sub_structs::{TradeInfo, UserData, UserFunding, UserFundingsData};
    use alloy::primitives::Address;

    fn sample_address() -> Address {
        "0x0000000000000000000000000000000000000001"
            .parse()
            .expect("static valid address")
    }

    fn sample_trade_info() -> TradeInfo {
        TradeInfo {
            coin: "BTC".to_string(),
            side: "B".to_string(),
            px: "50000.0".to_string(),
            sz: "0.01".to_string(),
            time: 1_700_000_000_000,
            hash: "0xabc".to_string(),
            start_position: "0.0".to_string(),
            dir: "Open Long".to_string(),
            closed_pnl: "0.0".to_string(),
            oid: 42,
            cloid: None,
            crossed: true,
            fee: "0.5".to_string(),
            fee_token: "USDC".to_string(),
            tid: 7,
        }
    }

    #[test]
    fn user_data_round_trips_with_fills() {
        let user = User {
            data: UserData::Fills(vec![sample_trade_info()]),
        };
        let value = serde_json::to_value(&user).expect("serialize User");

        // Wrapper serialises as `{"data": {"fills": [...]}}` because
        // `UserData` is an internally-tagged enum with `Fills` variant
        // becoming `{"fills": [...]}` under serde's default enum repr.
        let fills = value
            .pointer("/data/fills")
            .and_then(|v| v.as_array())
            .expect("fills array present at /data/fills");
        assert_eq!(fills.len(), 1, "one fill expected");
        assert_eq!(fills[0]["coin"], "BTC");
        assert_eq!(fills[0]["px"], "50000.0");
        assert_eq!(fills[0]["side"], "B");
        assert_eq!(fills[0]["oid"], 42);
        assert_eq!(fills[0]["tid"], 7);
    }

    #[test]
    fn user_fundings_round_trips() {
        let fundings = UserFundings {
            data: UserFundingsData {
                is_snapshot: Some(true),
                user: sample_address(),
                fundings: vec![UserFunding {
                    time: 1_700_000_000_000,
                    coin: "BTC".to_string(),
                    usdc: "1.5".to_string(),
                    szi: "0.01".to_string(),
                    funding_rate: "0.0001".to_string(),
                }],
            },
        };
        let value = serde_json::to_value(&fundings).expect("serialize UserFundings");

        let inner = value
            .pointer("/data/fundings")
            .and_then(|v| v.as_array())
            .expect("fundings array present at /data/fundings");
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0]["coin"], "BTC");
        assert_eq!(inner[0]["usdc"], "1.5");
        assert_eq!(inner[0]["fundingRate"], "0.0001");
    }
}
