use near_sdk::{ext_contract, AccountId, Promise};
use near_sdk::json_types::U128;
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::TokenId;
pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;

// Validator interface, for cross-contract calls
#[ext_contract(ext_nft_contract)]
trait ERC721Contract {
    fn nft_mint(&mut self, token_id: TokenId, token_owner_id: AccountId, token_metadata: TokenMetadata,) -> Promise;
}

#[ext_contract(ext_ft_contract)]
trait ERC20Contract {
    fn storage_balance_of(&self, account_id: AccountId) -> Promise;
    fn storage_deposit(&mut self, account_id: Option<AccountId>, registration_only: Option<bool>) -> Promise;
}
