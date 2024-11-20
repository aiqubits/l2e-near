use std::fmt;

use near_sdk::serde::{Deserialize, Serialize};

/// Enum that represents the data type of the EventLog.
/// The enum can either be an NftMint or an NftTransfer.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
    FTAddress(Vec<String>),
    NFTAddress(Vec<String>),
    AdminAddress(Vec<String>),
    AuthOwnerAddress(Vec<String>),
    AllSpenderClaimedForOwner(Vec<(String, String, bool)>),
    AllOwnerRewardsForSpender(Vec<(String, String, String)>),

    // 主网币allowances返回事件
    AllowancesForSpender,

    // spender转移nft给自己
    TransferNftFrom(Vec<NftTransferLog>),
    // spender转移balances给自己
    TransferBalacnesFrom(Vec<BalacnesTransferLog>),

}

/// Interface to capture data about an event
///
/// Arguments:
/// * `standard`: name of standard e.g. nep171
/// * `version`: e.g. 1.0.0
/// * `event`: associate event data
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    // 合约得名字
    pub standard: String,
    // 合约得版本
    pub version: String,

    // `flatten` to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>}.
    #[serde(flatten)]
    pub event: EventLogVariant,
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_JSON:{}",
            &serde_json::to_string(self).map_err(|_| fmt::Error)?
        ))
    }
}

/// An event log to capture token transfer
///
/// Arguments
/// * `authorized_id`: approved account to transfer
/// * `old_owner_id`: "owner.near"
/// * `new_owner_id`: "receiver.near"
/// * `token_ids`: ["1", "12345abc"]
/// * `memo`: optional message
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftTransferLog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_id: Option<String>,

    pub old_owner_id: String,
    pub new_owner_id: String,
    pub token_ids: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// An event log to capture token transfer
///
/// Arguments
/// * `authorized_id`: approved account to transfer
/// * `old_owner_id`: "owner.near"
/// * `new_owner_id`: "receiver.near"
/// * `token_ids`: ["1", "12345abc"]
/// * `memo`: optional message
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct BalacnesTransferLog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_id: Option<String>,

    pub old_owner_id: String,
    pub new_owner_id: String,
    pub main_token_amount: Vec<String>,
    pub ft_token_amount: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nep_format_ft_address() {
        let expected = r#"EVENT_JSON:{"standard":"l2e.top","version":"1.0.0","event":"f_t_address","data":["token.near","token2.near"]}"#;
        let log = EventLog {
            standard: "l2e.top".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::FTAddress(vec!["token.near".parse().unwrap(), "token2.near".parse().unwrap()]),
        };
        assert_eq!(expected, log.to_string());
    }

    #[test]
    fn nep_format_nft_address() {
        let expected = r#"EVENT_JSON:{"standard":"l2e.top","version":"1.0.0","event":"n_f_t_address","data":["nft.near","nft2.near"]}"#;
        let log = EventLog {
            standard: "l2e.top".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::NFTAddress(vec!["nft.near".parse().unwrap(), "nft2.near".parse().unwrap()]),
        };
        assert_eq!(expected, log.to_string());
    }

    #[test]
    fn nep_format_admin_address() {
        let expected = r#"EVENT_JSON:{"standard":"l2e.top","version":"1.0.0","event":"admin_address","data":["admin.near"]}"#;
        let log = EventLog {
            standard: "l2e.top".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::AdminAddress(vec!["admin.near".parse().unwrap()]),
        };
        assert_eq!(expected, log.to_string());
    }

    #[test]
    fn nep_format_auth_owner_address() {
        let expected = r#"EVENT_JSON:{"standard":"l2e.top","version":"1.0.0","event":"auth_owner_address","data":["auth_owner.near"]}"#;
        let log = EventLog {
            standard: "l2e.top".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::AuthOwnerAddress(vec!["auth_owner.near".parse().unwrap()]),
        };
        assert_eq!(expected, log.to_string());
    }

    #[test]
    fn nep_format_all_spender_claimed_for_owner() {
        let expected = r#"EVENT_JSON:{"standard":"l2e.top","version":"1.0.0","event":"all_spender_claimed_for_owner","data":[["spender1.near","token1.near",true],["spender2.near","token2.near",false]]}"#;
        let log = EventLog {
            standard: "l2e.top".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::AllSpenderClaimedForOwner(vec![
                ("spender1.near".parse().unwrap(), "token1.near".to_string(), true),
                ("spender2.near".parse().unwrap(), "token2.near".to_string(), false),
            ]),
        };
        assert_eq!(expected, log.to_string());
    }

    #[test]
    fn nep_format_all_owner_rewards_for_spender() {
        let expected = r#"EVENT_JSON:{"standard":"l2e.top","version":"1.0.0","event":"all_owner_rewards_for_spender","data":[["owner1.near","1","100"]]}"#;
        let log = EventLog {
            standard: "l2e.top".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::AllOwnerRewardsForSpender(vec![
                ("owner1.near".parse().unwrap(), "1".to_string(), "100".to_string()),
            ]),
        };
        assert_eq!(expected, log.to_string());
    }

    #[test]
    fn nep_format_allowances_for_spender() {
        let expected = r#"EVENT_JSON:{"standard":"l2e.top","version":"1.0.0","event":"allowances_for_spender"}"#;
        let log = EventLog {
            standard: "l2e.top".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::AllowancesForSpender,
        };
        assert_eq!(expected, log.to_string());
    }

    #[test]
    fn nep_format_transfer_nft() {
        let expected = r#"EVENT_JSON:{"standard":"l2e.top","version":"1.0.0","event":"transfer_nft_from","data":[{"authorized_id":"market.near","old_owner_id":"user1.near","new_owner_id":"user2.near","token_ids":["token"],"memo":"L2E Team!"}]}"#;
        let log = EventLog {
            standard: "l2e.top".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::TransferNftFrom(vec![NftTransferLog {
                authorized_id: Some("market.near".to_string()),
                old_owner_id: "user1.near".to_string(),
                new_owner_id: "user2.near".to_string(),
                token_ids: vec!["token".to_string()],
                memo: Some("L2E Team!".to_owned()),
            }]),
        };
        assert_eq!(expected, log.to_string());
    }

    #[test]
    fn nep_format_transfer_balances() {
        let expected = r#"EVENT_JSON:{"standard":"l2e.top","version":"1.0.0","event":"transfer_balacnes_from","data":[{"authorized_id":"market.near","old_owner_id":"user1.near","new_owner_id":"user2.near","main_token_amount":["near"],"ft_token_amount":["token"],"memo":"L2E Team!"}]}"#;
        let log = EventLog {
            standard: "l2e.top".to_string(),
            version: "1.0.0".to_string(),
            event: EventLogVariant::TransferBalacnesFrom(vec![BalacnesTransferLog {
                authorized_id: Some("market.near".to_string()),
                old_owner_id: "user1.near".to_string(),
                new_owner_id: "user2.near".to_string(),
                main_token_amount: vec!["near".to_string()],
                ft_token_amount: vec!["token".to_string()],
                memo: Some("L2E Team!".to_owned()),
            }]),
        };
        assert_eq!(expected, log.to_string());
    }
} 
