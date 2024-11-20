use near_sdk::{ext_contract, AccountId, Promise};
use near_sdk::json_types::U128;


pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;

type TokenId = String;

// Validator interface, for cross-contract calls
#[ext_contract(ext_nft_contract)]
trait ERC721Contract {
    fn nft_mint(&self, token_series_id: String, receiver_id: AccountId) -> Promise;
    fn nft_transfer(&self, receiver_id: AccountId, token_id: TokenId) -> Promise;
    fn nft_token(&self, token_id: TokenId) -> Promise;

}

#[ext_contract(ext_ft_contract)]
trait ERC20Contract {
    fn ft_transfer(&self, receiver_id: AccountId, amount: U128) -> Promise;
}
