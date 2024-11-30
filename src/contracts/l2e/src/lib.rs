// Find all our documentation at https://docs.near.org

mod events;
mod external;

use std::f32::consts::E;

pub use events::*;
pub use external::*;

use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::store::IterableSet;
use near_sdk::{
    env, log, near, require,
    store::{IterableMap, Vector},
    AccountId, Gas, NearToken, PanicOnDefault, Promise, PromiseError, PromiseOrValue,
};
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::storage_management::{
    StorageBalance, StorageBalanceBounds, StorageManagement,
};

pub const CONSTRACT_NAME: &str = "L2eTop";
pub const CONSTRACT_VERSION: &str = "1.0.0";

// Define the contract structure
#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct L2eTop {
    greeting: String,

    // spenderid -> <(ownerid, maintoken balance, token balance)> total balance can be mutli stage claim.
    balances: IterableMap<AccountId, Vector<(AccountId, NearToken, NearToken)>>,
    // ownerid -> <(spenderid, nft tokenid, claimed true/false)>
    nfts: IterableMap<AccountId, Vector<(AccountId, TokenId, bool)>>,
    erc20_address: Vector<AccountId>,
    erc721_address: Vector<AccountId>,
    // nft token id num
    token_id_num: U128,
    admin_address: IterableSet<AccountId>,
    auth_token_owner: IterableSet<AccountId>,
}

// Implement the contract structure
#[near]
impl L2eTop {
    #[init]
    #[private] // only callable by the contract's account
    pub fn init(erc20: AccountId, erc721: AccountId) -> Self {
        let mut default_bal_map =
            IterableMap::<AccountId, Vector<(AccountId, NearToken, NearToken)>>::new(b"b");
        default_bal_map.insert(env::signer_account_id(), Vector::new(b"v"));
        
        let mut default_nft_map =
            IterableMap::<AccountId, Vector<(AccountId, TokenId, bool)>>::new(b"n");
        default_nft_map.insert(env::signer_account_id(), Vector::new(b"i"));

        let mut erc20_address = Vector::new(b"2");
        erc20_address.push(erc20);

        let mut erc721_address = Vector::new(b"7");
        erc721_address.push(erc721);

        let token_id_num = U128::from(1000);
        let mut admin_address = IterableSet::new(b"a");
        admin_address.insert(env::signer_account_id());

        let mut auth_token_owner = IterableSet::new(b"t");
        auth_token_owner.insert(env::signer_account_id());

        Self {
            greeting: "Hello".to_string(),

            balances: default_bal_map,
            nfts: default_nft_map,
            erc20_address: erc20_address,
            erc721_address: erc721_address,

            token_id_num: token_id_num,
            admin_address: admin_address,
            auth_token_owner: auth_token_owner,
        }
    }

    pub fn get_erc20_address(&self) -> Vec<String> {
        let ft_address: Vec<String> = self
            .erc20_address
            .iter()
            .map(|x| x.to_string())
            .collect();

        let ft_address_log = EventLog {
            standard: CONSTRACT_NAME.to_string(),
            version: CONSTRACT_VERSION.to_string(),
            event: EventLogVariant::FTAddress(ft_address.clone()),
        };

        env::log_str(&ft_address_log.to_string());

        ft_address
    }

    pub fn get_erc721_address(&self) -> Vec<String> {
        let nft_address: Vec<String> = self
            .erc721_address
            .iter()
            .map(|x| x.to_string())
            .collect();

        let nft_address_log = EventLog {
            standard: CONSTRACT_NAME.to_string(),
            version: CONSTRACT_VERSION.to_string(),
            event: EventLogVariant::NFTAddress(nft_address.clone()),
        };

        env::log_str(&nft_address_log.to_string());

        nft_address
    }

    pub fn get_admin_address(&self) -> Vec<String> {
        let admin_address: Vec<String> = self
            .admin_address
            .iter()
            .map(|x| x.to_string())
            .collect();

        let admin_address_log = EventLog {
            standard: CONSTRACT_NAME.to_string(),
            version: CONSTRACT_VERSION.to_string(),
            event: EventLogVariant::AdminAddress(admin_address.clone()),
        };

        env::log_str(&admin_address_log.to_string());

        admin_address
    }

    pub fn get_auth_token_owner(&self) -> Vec<String> {
        let auth_token_owner: Vec<String> = self
            .auth_token_owner
            .iter()
            .map(|x| x.to_string())
            .collect();

        let auth_token_owner_log = EventLog {
            standard: CONSTRACT_NAME.to_string(),
            version: CONSTRACT_VERSION.to_string(),
            event: EventLogVariant::AuthOwnerAddress(auth_token_owner.clone()),
        };

        env::log_str(&auth_token_owner_log.to_string());

        auth_token_owner
    }

    pub fn get_all_spender_claim_for_owner(&self) -> Option<Vec<(String, String, bool)>> {
        let owner = env::signer_account_id();
        let spender_nftid_claim = self.nfts.get(&owner);
        if let Some(spender_nftid_claim) = spender_nftid_claim {
            let result_vecs: Vec<(std::string::String, std::string::String, bool)> =
                spender_nftid_claim
                    .iter()
                    .map(|a_s_b| (a_s_b.0.to_string(), a_s_b.1.clone(), a_s_b.2))
                    .collect();

            let all_spender_claim_for_owner_log = EventLog {
                standard: CONSTRACT_NAME.to_string(),
                version: CONSTRACT_VERSION.to_string(),
                event: EventLogVariant::AllSpenderClaimedForOwner(result_vecs.clone()),
            };

            env::log_str(&all_spender_claim_for_owner_log.to_string());
            return Some(result_vecs);
        }

        None
    }

    pub fn get_all_owner_rewards_for_spender(&self) -> Option<Vec<(String, String, String)>> {
        let spender = env::signer_account_id();
        let owner_bal_map = self.balances.get(&spender);
        if let Some(owner_bal_map) = owner_bal_map {
            let result_vecs: Vec<(
                std::string::String,
                std::string::String,
                std::string::String,
            )> = owner_bal_map
                .iter()
                .map(|a_n_b| {
                    (
                        a_n_b.0.to_string(),
                        a_n_b.1.to_string(),
                        a_n_b.2.to_string(),
                    )
                })
                .collect();

            let all_owner_rewards_for_spender_log = EventLog {
                standard: CONSTRACT_NAME.to_string(),
                version: CONSTRACT_VERSION.to_string(),
                event: EventLogVariant::AllOwnerRewardsForSpender(result_vecs.clone()),
            };

            env::log_str(&all_owner_rewards_for_spender_log.to_string());

            return Some(result_vecs);
        }

        None
    }

    pub fn get_allowances_for_spender(&self, owner: AccountId) -> Option<(u128, u128)> {
        let spender = env::predecessor_account_id();
        let mut main_token = 0;
        let mut token_amount = 0;

        if self.balances.contains_key(&spender) {
            let balances = self.balances.get(&spender).unwrap();

            let value = balances.iter().find(|x| x.0 == owner);
            if let Some(value) = value {
                main_token = value.1.as_near();
                token_amount = value.2.as_near();
            }

            let allowances_for_spender_log = EventLog {
                standard: CONSTRACT_NAME.to_string(),
                version: CONSTRACT_VERSION.to_string(),
                event: EventLogVariant::AllowancesForSpender,
            };

            env::log_str(&allowances_for_spender_log.to_string());

            return Some((main_token, token_amount));
        }

        None
    }

    /// Transfer main token, ft token, nft token for spender to l2e-top contract.
    /// ft is l2e contract associated token.ft_amount is the amount of ft token to transfer. by frontend control, default value is 0.
    #[payable]
    pub fn approve_for_spender(
        &mut self,
        spender: AccountId,
        main_token_amount: NearToken,
        ft_amount: NearToken,
        token_metadata: Option<TokenMetadata>,
        erc20_address: Option<AccountId>,  // erc20 address's owner must be l2e-top contract.
        erc721_address: Option<AccountId>,  // erc721 address's owner must be l2e-top contract.
    ) -> bool {
        log!("approve_for_spender: {:#?}", spender);
        let l2e_account = env::current_account_id();
        let owner = env::signer_account_id();
        let mut current_amount = NearToken::from_near(0);

        require!(
            main_token_amount > NearToken::from_near(0),
            "main_token_amount should be greater than 0"
        );

        // main_token_amount should be transfer value, env::attached_deposit() is acutal value.
        // frontend control vara_value >= env::attached_deposit()
        let attached_amount = env::attached_deposit();
        require!(attached_amount > NearToken::from_near(0), "attached_amount should be greater than 0");
        if attached_amount >= main_token_amount {
            current_amount = main_token_amount;
        } else {
            log!("attached_amount cannot be less than main_token_amount.");
            return false;
        }

        // Approve main token and ft token for spender
        // check if spender has balance
        if self.balances.contains_key(&spender) {
            let owner_value = self
                .balances
                .get_mut(&spender)
                .expect("No balance found for spender");
            if owner_value.iter().find(|x| x.0 == owner).is_some() {
                log!("Spender has already approved balance.");
                return false;
            }

            owner_value.push((owner.clone(), current_amount, ft_amount));
        } else {
            let mut owner_value = Vector::new(b"b");
            owner_value.push((owner.clone(), current_amount, ft_amount));
            self.balances.insert(spender.clone(), owner_value);

            let mut current_erc20 = self.erc20_address.get(0).expect("No erc20 address found");
            log!("current_erc20: {:#?}", current_erc20);
            current_erc20 = erc20_address.as_ref().unwrap_or(current_erc20);

            // current_erc20 = if let Some(ref erc20) = erc20_address {
            //     erc20
            // } else {
            //     current_erc20
            // };

            // cross contract call to erc20
            // check spender and l2e account has balance
            if !Self::is_account_registered_for_ft(spender.clone(), current_erc20.clone()) {
                return false;
            }

            // if !Self::is_account_registered_for_ft(l2e_account.clone(), current_erc20.clone()) {
            //     return false;
            // }
            // let _ft_contract_balance_promise = ext_ft_contract::ext(current_erc20.clone())
            //    .storage_balance_of(spender.clone())
            //    .then(Self::ext(env::current_account_id()).ft_storage_balance_of_callback(current_erc20.clone(), spender.clone()));
               
            // let _call_ft_contract_promise = ext_ft_contract::ext(current_erc20.clone())
            //     .with_static_gas(Gas::from_tgas(100))
            //     .with_attached_deposit(NearToken::from_yoctonear(1))
            //     .ft_transfer(env::current_account_id(), U128::from(ft_amount.as_near()), None);

            log!("Spender has no balance, create new balance. {:?}", self
            .balances
            .get(&spender)
            .expect("No balance found for spender").iter().find(|x| x.0 == owner));
        }

        // Mint and Approve NFT for spender
        let token_id: u128 = (self.token_id_num).into();
        self.token_id_num = U128::from(token_id + 1);

        let mut current_erc721 = self.erc721_address.get(0).expect("No erc721 address found");
        log!("current_erc721: {:#?}", current_erc721);
        current_erc721 = if let Some(ref erc721) = erc721_address {
            erc721
        } else {
            current_erc721
        };

        // cross contract call to erc721
        let _pro = if let Some(tm) = token_metadata {
            log!("token_metadata is Some use custom metadata");
            let promise = ext_nft_contract::ext(current_erc721.clone())
            // .with_static_gas(Gas::from_tgas(300))
            .with_attached_deposit(NearToken::from_millinear(20))
            .nft_mint((token_id + 1).to_string(), l2e_account.clone(), tm);
            log!("nft_mint promise returned");
            let _mint_nft_promise = promise.then(
                // Create a promise to callback query_greeting_callback
                Self::ext(l2e_account.clone())
                    .nft_mint_callback(),
            );
            log!("nft_mint_callback promise returned");
            _mint_nft_promise
        } else {
            log!("token_metadata is None use default metadata");
            let promise = ext_nft_contract::ext(current_erc721.clone())
            .with_attached_deposit(NearToken::from_millinear(20))
            .nft_mint((token_id + 1).to_string(), l2e_account.clone(), TokenMetadata {
                title: Some("L2E.TOP Chain Near Network".to_string()),
                description: Some("Near Network and L2E.TOP Joint Certification Reward.".to_string()),
                copies: Some(1),
                media: None,
                media_hash: None,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
            });

            let _mint_nft_promise = promise.then(
                // Create a promise to callback query_greeting_callback
                Self::ext(l2e_account.clone())
                    .nft_mint_callback(),
            );
            _mint_nft_promise
        };

        // // approve nft for spender
        // let nft_approve_promise = ext_nft_contract::ext(current_erc721.clone())
        //     // .with_attached_deposit(NearToken::from_yoctonear(1))
        //     .with_attached_deposit(NearToken::from_millinear(200))
        //     .nft_approve((token_id + 1).to_string(), spender.clone(), None,);

        // let _nft_approve_callback_promise = nft_approve_promise.then(
        //     Self::ext(l2e_account)
        //         .nft_approve_callback(),
        // );

        // store nft tokenid and spender address
        let nfts: &mut Vector<(AccountId, String, bool)> =
            self.nfts.get_mut(&owner).expect("No nft owner found for owner");
        nfts.push((spender, (token_id + 1).to_string(), false));
        log!("Store nft tokenid and spender address.");
        true
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn nft_mint_callback(
        &self,
        #[callback_result] call_result: Result<Token, PromiseError>,
    ) -> Option<Token> {
        log!("beigin nft_mint_callback");
        if call_result.is_err() {
            log!("There was an error contacting NFT contract nft_mint: {:#?}", call_result.map_err(|e| format!("error details: {:#?}", e)));
            return None;
        }

        // Return the token data
        log!("Minted NFT with token_id::----------------");
        let token: Token = call_result.unwrap();
        log!("Minted NFT with token_id: {:?}", token);
        return Some(token);
    }

    #[private]
    pub fn is_account_registered_for_ft(account_id: AccountId, erc20_address: AccountId) -> bool {
        let _ft_contract_balance_promise = ext_ft_contract::ext(erc20_address.clone())
            .with_attached_deposit(NearToken::from_millinear(20))
            .storage_balance_of(account_id.clone())
            .then(Self::ext(env::current_account_id()).ft_storage_balance_of_callback(erc20_address, account_id));

        true
    }

    #[private]
    pub fn ft_storage_balance_of_callback(
        &self,
        erc20_address: AccountId,
        spender: AccountId,
        #[callback_result] call_result: Result<Option<StorageBalance>, PromiseError>,
    ) -> Option<StorageBalance> {
        // Check if the promise succeeded
        if call_result.is_err() {
            log!("There was an error contacting FT contract ft_storage_balance_callback");
            return None;
        }

        // register spender if not registered
        let result: Option<StorageBalance> = call_result.unwrap();
        if result.is_none() {
            log!("ft_storage_balance_of_callback result is None");
            let promise = ext_ft_contract::ext(erc20_address)
                .with_attached_deposit(NearToken::from_millinear(20))
                .storage_deposit(Some(spender.clone()), Some(true));
            let _ = promise.then(Self::ext(env::current_account_id()).storage_deposit_callback());
        } else {
            log!("ft_storage_balance_of_callback spender is registered");
        }
        log!("ft_storage_balance_callback over");
        return result;
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn storage_deposit_callback(
        &self,
        #[callback_result] call_result: Result<StorageBalance, PromiseError>,
    ) -> Option<bool> {
        // Check if the promise succeeded
        if call_result.is_err() {
            log!("There was an error contacting FT contract storage_deposit_callback");
            return None;
        }
        let result = call_result.unwrap();
        log!("storage_deposit_callback over: total--{:?}, available--{:?}", result.total, result.available);
        return Some(true);
    }

    #[private]
    pub fn nft_approve_callback(
        &self,
        #[callback_result] call_result: Result<Vec<u8>, PromiseError>,
    ){
        if call_result.is_err() {
            log!("There was an error contacting FT contract nft_approve_callback: {:#?}", call_result.map_err(|e| format!("error details: {:#?}", e)));
        }
    }

    /// First mint and approve nft for spender, Then call this method to claim nft.
    #[payable]
    pub fn transfer_nft_from(&mut self, owner: AccountId, erc721_address: Option<AccountId>) -> bool {
        let spender = env::signer_account_id();
        let token_id = self
            .nfts
            .get(&owner)
            .expect("No nft found for owner")
            .iter()
            .find(|x| x.0 == spender && !x.2)
            .expect("No unclaimed nft found for spender")
            .1
            .clone();
        log!("transfer_nft_from token_id: {:#?}", token_id);
        let mut current_erc721 = self.erc721_address.get(0).expect("No nft address found");
        current_erc721 = if let Some(ref erc721) = erc721_address {
            erc721
        } else {
            current_erc721
        };

        let promise = ext_nft_contract::ext(current_erc721.clone())
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .nft_transfer(spender.clone(), token_id.clone(), None, None);

        let _transfer_nft_promise = promise.then(
            // Create a promise to callback query_greeting_callback
            Self::ext(env::current_account_id()).nft_transfer_callback(),
        );

        // Set already claimed nft to true
        let nfts: &mut Vector<(AccountId, String, bool)> =
            self.nfts.get_mut(&owner).expect("No nft found for owner");
        let index = nfts
            .iter()
            .position(|x| x.0 == spender && x.1 == token_id)
            .unwrap();
        nfts[index as u32].2 = true;

        let transfer_nft_from_log = EventLog {
            standard: CONSTRACT_NAME.to_string(),
            version: CONSTRACT_VERSION.to_string(),
            event: EventLogVariant::TransferNftFrom(vec![NftTransferLog {
                authorized_id: Some(owner.clone().to_string()),
                old_owner_id: env::current_account_id().to_string(),
                new_owner_id: spender.to_string(),
                token_ids: vec![token_id.clone()],
                memo: Some("L2E Team".to_string()),
            }]),
        };

        env::log_str(&transfer_nft_from_log.to_string());

        true
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn nft_transfer_callback(&self, #[callback_result] call_result: Result<(), PromiseError>) {
        // Check if the promise succeeded
        if call_result.is_err() {
            log!("There was an error contacting NFT contract nft_transfer: {:#?}", call_result.map_err(|e| format!("error details: {:#?}", e)));
        }
    }

    #[payable]
    pub fn transfer_balances_from(
        &mut self,
        owner: AccountId,
        erc20_address: Option<AccountId>,
    ) -> bool {
        log!("transfer_balances_from: {:#?}", owner);
        let spender = env::signer_account_id();

        // check nft authoriaztion
        let nft_id_flag = self
            .nfts
            .get(&owner)
            .expect("No Approved nft found for owner")
            .iter()
            .find(|x| x.0 == spender)
            .expect("No claimed nft found for spender")
            .2;
        log!("transfer_balances_from nft_id_flag: {} ", nft_id_flag);

        if !nft_id_flag {
            return false;
        }
        log!("transfer_balances_from main_token_amount begin");
        // transfer main token and ft token from owner to spender
        for (k, v) in self.balances.iter(){
            log!("transfer_balances_from k: {:#?}", k);
            log!("transfer_balances_from v: {:#?}", v);
            for arr_v_tuple in v.iter(){
                log!("transfer_balances_from arr_v_tuple0: {:#?}", arr_v_tuple.0);
                log!("transfer_balances_from arr_v_tuple1: {:#?}", arr_v_tuple.1);
                log!("transfer_balances_from arr_v_tuple2: {:#?}", arr_v_tuple.2);
            }
        }
        log!("transfer_balances_from spender: {:#?}", spender);
        for v in self.balances.get(&spender).expect("read vector fail").iter(){
            log!("transfer_balances_from v: {:#?}", v);
        }
        let current_main_token_amount = self
            .balances
            .get(&spender)
            .expect("No balance approve found for spender")
            .iter()
            .find(|x| x.0 == owner)
            .expect("No main_token balance found for spender")
            .1;
        log!("transfer_balances_from main_token_amount over");

        // transfer current contract main token to spender
        let _promise = Promise::new(spender.clone()).transfer(current_main_token_amount);

        // if main_token_amount <= current_main_token {
        //     let new_main_token_amount = current_main_token
        //         .checked_sub(main_token_amount)
        //         .expect("main_token_amount Subtraction overflow");

        //     self.balances
        //         .get_mut(&spender)
        //         .expect("No balance found for owner")
        //         .iter_mut()
        //         .find(|x| x.0 == owner)
        //         .expect("No main_token balance found for spender")
        //         .1 = new_main_token_amount;
        //     log!("transfer_balances_from transfer maintoken");

        //     // transfer current contract main token to spender
        //     let _promise = Promise::new(spender.clone()).transfer(main_token_amount);
        // }
        log!("transfer_balances_from transfer maintoken over");

        // transfer ft token from owner to
        let mut current_erc20 = self.erc20_address.get(0).expect("No nft address found");
        current_erc20 = if let Some(ref erc20) = erc20_address {
            erc20
        } else {
            current_erc20
        };

        let current_ft_token_amount = self
            .balances
            .get(&spender)
            .expect("No balance approve found for owner")
            .iter()
            .find(|x| x.0 == owner)
            .expect("No ft_amount balance found for spender")
            .2;

        // transfer current contract ft token to spender, cross contract call to erc20
        // cross contract call to erc20
        let promise = ext_ft_contract::ext(current_erc20.clone())
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .ft_transfer(spender.clone(), U128::from(current_ft_token_amount.as_near()), None);
        
        let _mint_nft_promise = promise.then(
            // Create a promise to callback query_greeting_callback
            Self::ext(env::current_account_id())
                .ft_transfer_callback(),
        );

        // if ft_amount <= current_ft_token {
        //     current_ft_token = current_ft_token
        //         .checked_sub(ft_amount)
        //         .expect("ft_amount Subtraction overflow");

        //     self.balances
        //         .get_mut(&spender)
        //         .expect("No balance found for owner")[0]
        //         .2 = current_ft_token;
        //     log!("transfer_balances_from transfer token");

        //     // transfer current contract ft token to spender, cross contract call to erc20
        //     // cross contract call to erc20
        //     let promise = ext_ft_contract::ext(current_erc20.clone())
        //         .ft_transfer(spender.clone(), U128::from(ft_amount.as_near()), None);

        //     let _mint_nft_promise = promise.then(
        //         // Create a promise to callback query_greeting_callback
        //         Self::ext(env::current_account_id())
        //             .ft_transfer_callback(),
        //     );
        // }

        let transfer_balances_from_log = EventLog {
            standard: CONSTRACT_NAME.to_string(),
            version: CONSTRACT_VERSION.to_string(),
            event: EventLogVariant::TransferBalacnesFrom(vec![BalacnesTransferLog {
                authorized_id: Some(owner.clone().to_string()),
                old_owner_id: env::current_account_id().to_string(),
                new_owner_id: spender.to_string(),
                main_token_amount: vec![current_main_token_amount.as_near().to_string()],
                ft_token_amount: vec![current_ft_token_amount.as_near().to_string()],
                memo: Some("L2E Team".to_string()),
            }]),
        };

        env::log_str(&transfer_balances_from_log.to_string());
        log!("transfer_balances_from over");

        true
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn ft_transfer_callback(&self, #[callback_result] call_result: Result<(), PromiseError>) {
        // Check if the promise succeeded
        if call_result.is_err() {
            log!("There was an error contacting FT contract ft_transfer: {:#?}", call_result.map_err(|e| format!("error details: {:#?}", e)));
        }
    }

    pub fn add_admin_address(&mut self, new_admin_address: AccountId) -> bool {
        let current_caller = env::predecessor_account_id();
        if self.admin_address.contains(&current_caller) && !self.admin_address.contains(&new_admin_address) {
            self.admin_address.insert(new_admin_address.clone());
            log!("New admin address added: {}", new_admin_address.to_string());
            return true;
        }

        false
    }

    pub fn add_auth_token_owner(&mut self, owner_address: AccountId) -> bool {
        let current_caller = env::predecessor_account_id();
        let mut admin_flag = false;
        if self.admin_address.contains(&current_caller) {
            admin_flag = true;
        }

        if admin_flag {
            // add auth_token_owner
            let mut auth_flag = false;
            if self.auth_token_owner.contains(&owner_address) {
                auth_flag = true;
            }

            if !auth_flag {
                self.auth_token_owner.insert(owner_address.clone());
            }

            if !self.balances.contains_key(&owner_address) {
                self.balances.insert(owner_address.clone(), Vector::new(b"b"));
            }

            if !self.nfts.contains_key(&owner_address) {
                self.nfts.insert(owner_address.clone(), Vector::new(b"b"));
            }

            log!("New auth_token_owner added: {}", owner_address.to_string());

            return true;
        }

        false
    }

    pub fn add_contract_address(
        &mut self,
        erc20_address: AccountId,
        erc721_address: AccountId,
    ) -> bool {
        let current_caller = env::predecessor_account_id();

        if self.admin_address.contains(&current_caller)
            || self.auth_token_owner.contains(&current_caller)
        {
            if self.erc20_address.iter().position(|x| x == &erc20_address).is_none() {
                self.erc20_address.push(erc20_address.clone());
            }

            if self.erc721_address.iter().position(|x| x == &erc721_address).is_none() {
                self.erc721_address.push(erc721_address.clone());
            }

            log!("New contract address added: {}, {}", erc20_address.to_string(), erc721_address.to_string());

            return true;
        }

        false
    }

    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn get_greeting(&self) -> String {
        self.greeting.clone()
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, greeting: String) {
        log!("Saving greeting: {greeting}");
        self.greeting = greeting;
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_contract() {
        let erc20: AccountId = "erc20.near".parse().unwrap();
        let erc721: AccountId = "erc721.near".parse().unwrap();

        let contract = L2eTop::init(erc20, erc721);
        // L2eTop::init(env::current_account_id(), "erc721.testnet".to_string());

        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello");

        assert!(contract.balances.len() == 1);
        assert!(contract.balances.get(&env::signer_account_id()).unwrap().is_empty());

        assert!(contract.nfts.len() == 1);
        assert!(contract.nfts.get(&env::signer_account_id()).unwrap().is_empty());


        assert!(contract.erc20_address.len() == 1);
        assert_eq!(
            contract.erc20_address.get(0),
            "erc20.near".parse::<AccountId>().as_ref().ok()
        );

        assert!(contract.erc721_address.len() == 1);
        assert_eq!(
            contract.erc721_address.get(0),
            "erc721.near".parse::<AccountId>().as_ref().ok()
        );

        assert_eq!(contract.token_id_num, U128::from(0));

        assert!(contract.admin_address.len() == 1);
        assert!(contract.admin_address.contains(&(env::signer_account_id())));

        assert!(contract.auth_token_owner.len() == 1);
        assert!(contract.auth_token_owner.contains(&(env::signer_account_id())));

    }

    #[test]
    fn test_all_get_and_add_functions() {
        let erc20: AccountId = "erc20.near".parse().unwrap();
        let erc721: AccountId = "erc721.near".parse().unwrap();
        let new_admin: AccountId = "new_admin.near".parse().unwrap();
        let new_auth: AccountId = "new_auth.near".parse().unwrap();

        let mut contract = L2eTop::init(erc20, erc721);

        assert!(contract.add_admin_address(new_admin));
        assert!(contract.add_auth_token_owner(new_auth.clone()));
        
        assert_eq!(contract.get_erc20_address(), vec!["erc20.near".to_owned()]);
        assert_eq!(contract.get_erc721_address(), vec!["erc721.near".to_owned()]);
        assert_eq!(contract.get_admin_address(), vec![ env::signer_account_id().to_string(), "new_admin.near".to_owned()]);
        assert_eq!(contract.get_auth_token_owner(), vec![ env::signer_account_id().to_string(), "new_auth.near".to_owned()]);

        assert_eq!(contract.get_all_spender_claim_for_owner(), Some(vec![]));
        assert_eq!(contract.get_all_owner_rewards_for_spender(), Some(vec![]));
        assert_eq!(contract.get_allowances_for_spender("owner.near".parse().unwrap()), Some((0,0)));

        assert_eq!(contract.balances.len(), 2);
        assert!(contract.balances.contains_key(&env::signer_account_id()));
        assert!(contract.balances.contains_key(&new_auth));

        assert_eq!(contract.nfts.len(), 2);
        assert!(contract.balances.contains_key(&env::signer_account_id()));
        assert!(contract.balances.contains_key(&new_auth));
    }

}
