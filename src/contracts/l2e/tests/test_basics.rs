// rustc 1.80版本下的测试，与nft ft rust 1.70版不匹配

use near_sdk::{json_types::U128, log, AccountId};
use near_workspaces::{types::NearToken, Account, Contract};
use serde_json::json;
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let _ = near_workspaces::compile_project("./").await?;

    // test_basics_on(&contract_wasm).await?;
    Ok(())
}

async fn test_basics_on(contract_wasm: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // let sandbox = near_workspaces::sandbox().await?;

    // // load contracts
    // let ft_contract_wasm = std::fs::read("./tests/fungible_token.wasm")?;
    // let nft_contract_wasm = std::fs::read("./tests/non_fungible_token.wasm")?;

    // // deploy contracts
    // let ft_contract = sandbox.dev_deploy(&ft_contract_wasm).await?;
    // let nft_contract = sandbox.dev_deploy(&nft_contract_wasm).await?;
    // let contract = sandbox.dev_deploy(contract_wasm).await?;

    //   // Initialize contract
    // contract
    // .call('new_default_meta')
    // .args_json(nft_account, 'new_default_meta', { owner_id: l2e_account.accountId });

    // contract
    // .call('new_default_meta')
    // .args_json(ft_account, 'new_default_meta', { owner_id: l2e_account.accountId, total_supply: '100000000000000000000000000000' });

    // contract.
    // call('init')
    // .args_json(l2e_account, 'init', { erc20: ft_account.accountId, erc721: nft_account.accountId });

    // let contract_account = contract.id();
    // let user_account = sandbox.dev_create_account().await?;

    // // init contracts
    // let ft_init_outcome = ft_contract
    //     .call("new_default_meta")
    //     .args_json(serde_json::json!({
    //         "owner_id": contract_account,
    //         "total_supply": U128::from(100000000000)
    //     }))
    //     .transact()
    //     .await?;
    // // log!("FT Init Outcome: {:#?}", ft_init_outcome);
    // assert!(ft_init_outcome.is_success());

    // let nft_init_outcome = nft_contract
    //     .call("new_default_meta")
    //     .args_json(serde_json::json!({
    //         "owner_id": contract_account
    //     }))
    //     .transact()
    //     .await?;

    // // log!("NFT Init Outcome: {:#?}", nft_init_outcome);
    // assert!(nft_init_outcome.is_success());

    // let init_outcome = contract
    //     .call("init")
    //     .args_json(json!({"erc20": ft_contract.id(),"erc721": nft_contract.id()}))
    //     .transact()
    //     .await?;
    // // log!("L2E Init Outcome: {:#?}", init_outcome);
    // assert!(init_outcome.is_success());

    // test_greeting_on(&user_account, &contract).await?;
    // test_get_erc20_address(&contract, &ft_contract).await?;
    // test_get_erc721_address(&contract, &nft_contract).await?;
    // test_get_admin_address(&contract, contract_account).await?;
    // test_get_auth_owner_address(&contract, contract_account).await?;
    // test_get_all_spender_claim_for_owner(&contract).await?;
    // test_get_all_owner_rewards_for_spender(&contract).await?;
    // test_get_allowances_for_spender(&contract, &user_account).await?;
    // test_approve_for_spender(
    //     &contract,
    //     &user_account,
    //     NearToken::from_near(1),
    //     NearToken::from_near(100),
    //     ft_contract.id().clone(),
    //     nft_contract.id().clone(),
    // )
    // .await?;
    // test_transfer_nft_from(&contract, &user_account, &contract.as_account(), &nft_contract.as_account()).await?;
    // test_transfer_balances_from(&contract, &user_account, &contract.as_account(), &ft_contract.as_account()).await?;

    Ok(())
}

async fn test_greeting_on(
    user: &Account,
    contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_message_outcome = contract.view("get_greeting").args_json(json!({})).await?;
    assert_eq!(user_message_outcome.json::<String>()?, "Hello");

    let outcome = user
        .call(contract.id(), "set_greeting")
        .args_json(json!({"greeting": "Hello World!"}))
        .transact()
        .await?;

    assert!(outcome.is_success());
    let user_message_outcome = contract.view("get_greeting").args_json(json!({})).await?;
    assert_eq!(user_message_outcome.json::<String>()?, "Hello World!");

    log!("Test Greeting On: OK");
    Ok(())
}

async fn test_get_erc20_address(
    contract: &Contract,
    ft_contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_message_outcome = contract
        .view("get_erc20_address")
        .args_json(json!({}))
        .await?;
    assert_eq!(
        user_message_outcome.json::<Vec<String>>()?,
        vec![ft_contract.id().to_string()]
    );

    log!("Test Get Erc20 Address: OK");
    Ok(())
}

async fn test_get_erc721_address(
    contract: &Contract,
    nft_contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_message_outcome = contract
        .view("get_erc721_address")
        .args_json(json!({}))
        .await?;
    assert_eq!(
        user_message_outcome.json::<Vec<String>>()?,
        vec![nft_contract.id().to_string()]
    );

    log!("Test Get Erc721 Address: OK");
    Ok(())
}

async fn test_get_admin_address(
    contract: &Contract,
    admin_account: &AccountId,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_message_outcome = contract
        .view("get_admin_address")
        .args_json(json!({}))
        .await?;
    assert_eq!(
        user_message_outcome.json::<Vec<String>>()?,
        vec![admin_account.to_string()]
    );

    log!("Test Get Admin Address: OK");
    Ok(())
}

async fn test_get_auth_owner_address(
    contract: &Contract,
    auth_owner_account: &AccountId,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_message_outcome = contract
        .view("get_auth_token_owner")
        .args_json(json!({}))
        .await?;

    let right_result: Vec<String> = vec![auth_owner_account.to_string()];
    assert_eq!(user_message_outcome.json::<Vec<String>>()?, right_result);

    log!("Test Get Auth Owner Address: OK");
    Ok(())
}

async fn test_get_all_spender_claim_for_owner(
    contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    // 需要获取当前用户以及修改存储的方法，都需call调用，仅view查看除外
    let user_message_outcome = contract
        .as_account()
        .call(contract.id(), "get_all_spender_claim_for_owner")
        .args_json(json!({}))
        .transact()
        .await?;
    assert!(user_message_outcome.is_success());

    assert_eq!(
        user_message_outcome.json::<Option<Vec<()>>>()?,
        Some(vec![])
    );

    log!("Test Get All Spender Claim For Owner: OK");
    Ok(())
}

async fn test_get_all_owner_rewards_for_spender(
    contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_message_outcome = contract
        .as_account()
        .call(contract.id(), "get_all_owner_rewards_for_spender")
        .args_json(json!({}))
        .transact()
        .await?;
    assert!(user_message_outcome.is_success());

    assert_eq!(
        user_message_outcome.json::<Option<Vec<()>>>()?,
        Some(vec![])
    );

    log!("Test Get All Owner Rewards For Spender: OK");
    Ok(())
}

async fn test_get_allowances_for_spender(
    contract: &Contract,
    user_account: &Account,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_message_outcome = contract
        .as_account()
        .call(contract.id(), "get_allowances_for_spender")
        .args_json(json!({"owner": user_account.id()}))
        .transact()
        .await?;
    assert!(user_message_outcome.is_success());

    assert_eq!(
        user_message_outcome.json::<Option<(u128, u128)>>()?,
        Some((0, 0))
    );

    log!("Test Get Allowances For Spender: OK");
    Ok(())
}

// 默认contract admin_account 作为 auth owner account
async fn test_approve_for_spender(
    contract: &Contract,
    spender: &Account,
    main_token_amount: NearToken,
    ft_amount: NearToken,
    erc20: AccountId,
    erc721: AccountId,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_message_outcome = contract
        .as_account()
        .call(contract.id(), "approve_for_spender")
        .deposit(NearToken::from_near(2))
        .args_json(json!({
            "spender": spender.id(), 
            "main_token_amount": main_token_amount, 
            "ft_amount": ft_amount, 
            "token_metadata": TokenMetadata {
                title: Some("L2E.TOP Chain Near Network".to_string()),
                description: Some("Near Network and L2E.TOP Joint Certification Reward.".to_string()),
                copies: Some(1),
                media: Some("".to_string()),
                media_hash: None,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
            },
            "erc20": erc20,
            "erc721": erc721,
        }))
        .max_gas()
        .transact()
        .await?;

    assert!(user_message_outcome.is_success());

    log!("Test Approve For Spender: OK");
    Ok(())
}

async fn test_transfer_nft_from(
    contract: &Contract,
    spender: &Account,
    owner: &Account,
    erc721: &Account,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_message_outcome = spender
        .call(contract.id(), "transfer_nft_from")
        .args_json(json!({
            "owner": owner.id(), 
            "erc721": erc721.id()
        }))
        .max_gas()
        .transact()
        .await?;

    // log!("Test Transfer Nft From Spender: {:#?}", user_message_outcome);
    assert!(user_message_outcome.is_success());

    log!("Test Transfer Nft From: OK");
    Ok(())
}

async fn test_transfer_balances_from(
    contract: &Contract,
    spender: &Account,
    owner: &Account,
    erc20: &Account,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_message_outcome = spender
        .call(contract.id(), "transfer_balances_from")
        .args_json(json!({
            "owner": owner.id(), 
            "erc20": erc20.id(),
        }))
        .max_gas()
        .transact()
        .await?;

    // log!("Test Transfer Balances From Spender: {:#?}", user_message_outcome);
    assert!(user_message_outcome.is_success());

    log!("Test Transfer Balances From: OK");
    Ok(())
}
