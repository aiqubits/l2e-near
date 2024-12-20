import path from 'path';
import anyTest from 'ava';
import { Worker, NEAR } from 'near-workspaces';
import { setDefaultResultOrder } from 'dns';import { assert } from 'console';
 setDefaultResultOrder('ipv4first'); // temp fix for node >v17

/**
 *  @typedef {import('near-workspaces').NearAccount} NearAccount
 *  @type {import('ava').TestFn<{worker: Worker, accounts: Record<string, NearAccount>}>}
 */
const test = anyTest;

test.beforeEach(async t => {
  // Create sandbox
  const worker = t.context.worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;
  const l2e_account = await root.createSubAccount('l2e-account');
  const ft_account = await root.createSubAccount('ft-account');
  const nft_account = await root.createSubAccount('nft-account');
  const user_account = await root.createSubAccount('user-account');
  const owner_account = await root.createSubAccount('owner-account');

  // Get wasm file path from package.json test script in folder above
  await l2e_account.deploy(
    path.join('./target/wasm32-unknown-unknown/release/l2e_top_near.wasm')
  );
  await ft_account.deploy(
    path.join('./tests/fungible_token.wasm')
  );
  await nft_account.deploy('./tests/non_fungible_token.wasm');

  // Initialize contract
  await l2e_account.callRaw(nft_account, 'new_default_meta', { owner_id: l2e_account.accountId });
  await l2e_account.callRaw(ft_account, 'new_default_meta', { owner_id: l2e_account.accountId, total_supply: '100000000000000000000000000000' });
  await l2e_account.callRaw(l2e_account, 'init', { erc20: ft_account.accountId, erc721: nft_account.accountId });

  // Save state for test runs, it is unique for each test
  t.context.accounts = { root, l2e_account, ft_account, nft_account, user_account, owner_account };
});

test.afterEach.always(async (t) => {
  await t.context.worker.tearDown().catch((error) => {
    console.log('Failed to stop the Sandbox:', error);
  });
});

// Only view caller
test('returns the default greeting', async (t) => {
  const { l2e_account } = t.context.accounts;
  const greeting = await l2e_account.view('get_greeting', {});
  t.is(greeting, 'Hello');
});

test('changes the greeting', async (t) => {
  const { root, l2e_account } = t.context.accounts;
  await root.callRaw(l2e_account, 'set_greeting', { greeting: 'Howdy' });
  const greeting = await l2e_account.view('get_greeting', {});
  t.is(greeting, 'Howdy');
});

test('test_get_erc20_address', async (t) => {
  const { l2e_account, ft_account } = t.context.accounts;
  const erc20_address = await l2e_account.view('get_erc20_address', {});
  t.deepEqual(erc20_address, [ft_account.accountId]);
});

test('test_get_erc721_address', async (t) => {
  const { l2e_account, nft_account } = t.context.accounts;
  const erc721_address = await l2e_account.view('get_erc721_address', {});
  t.deepEqual(erc721_address, [nft_account.accountId]);
});

test('test_get_admin_address', async (t) => {
  const { l2e_account } = t.context.accounts;
  const admin_address = await l2e_account.view('get_admin_address', {});
  t.deepEqual(admin_address, [l2e_account.accountId]);
});

test('test_get_auth_owner_address', async (t) => {
  const { l2e_account } = t.context.accounts;
  const auth_owner_address = await l2e_account.view('get_auth_token_owner', {});
  t.deepEqual(auth_owner_address, [l2e_account.accountId]);
});

test('test_add_auth_token_owner', async (t) => {
  const { l2e_account, owner_account } = t.context.accounts;
  const add_auth_token_owner_result = await l2e_account.call(l2e_account, 'add_auth_token_owner', { owner_address: owner_account });
  t.deepEqual(add_auth_token_owner_result, true);
});

// Already signed account caller
test('test_get_all_spender_claim_for_owner', async (t) => {
  const { l2e_account } = t.context.accounts;
  const all_spender_claim_for_owner_address = await l2e_account.call(l2e_account, 'get_all_spender_claim_for_owner', {});
  t.deepEqual(all_spender_claim_for_owner_address, [["l2e-account.test.near","0",false]]);
});

test('test_get_all_owner_rewards_for_spender', async (t) => {
  const { l2e_account } = t.context.accounts;
  const all_owner_rewards_for_spender_address = await l2e_account.call(l2e_account, 'get_all_owner_rewards_for_spender', {});
  t.deepEqual(all_owner_rewards_for_spender_address, [["l2e-account.test.near",0,0]]);
});

test('test_get_allowances_for_spender', async (t) => {
  const { l2e_account } = t.context.accounts;
  const allowances_for_spender_address = await l2e_account.call(l2e_account, 'get_allowances_for_spender', { owner: 'l2e-account.test.near' });
  t.deepEqual(allowances_for_spender_address, [0,0]);
});

setTimeout(() => {
  console.log('test_approve_transfer_nft_balances_for_spender begin 10s');
}, 10000);

// Change storage state caller
test('test_approve_transfer_nft_balances_for_spender', async (t) => {
  console.log('test_approve_for_spender begin start');
  setTimeout(() => {
    console.log('test_approve_for_spender sleep 5s');
  }, 5000);

  const { l2e_account, ft_account, nft_account, user_account, owner_account } = t.context.accounts;

  const add_auth_token_owner_result = await l2e_account.call(l2e_account, 'add_auth_token_owner', { owner_address: owner_account });
  t.deepEqual(add_auth_token_owner_result, true);
console.log(`NEAR.parse 1N=====================${NEAR.parse("1 N").toString()}`);
1000000000000000000000000
  // test approve_for_spender
  const approve_for_spender_result = await owner_account
    .call(l2e_account, 'approve_for_spender', {
      spender: user_account.accountId,
      main_token_amount: NEAR.parse("1 N").toString(),
      ft_amount: NEAR.parse("100 N").toString(),
      // token_metadata: {
      //   "title": "L2E.TOP Chain Near Network",
      //   "description": "Near Network and L2E.TOP Joint Certification Reward.",
      //   "copies": 1,
      //   "media": "",
      // },
      // erc20: ft_account.accountId,
      // erc721: nft_account.accountId,
    }, { gas: "300000000000000", attachedDeposit: NEAR.parse("5 N").toString() });
    console.log("consolelog------------------test_approve_for_spender-------------------");
    // console.log(JSON.stringify(approve_for_spender_result));
    t.deepEqual(approve_for_spender_result, true);

  const all_spender_claim_for_owner_address = await owner_account.call(l2e_account, 'get_all_spender_claim_for_owner', {});
  console.log(`test_get_all_spender_claim_for_owner after approve_for_spender: ${all_spender_claim_for_owner_address}`)
  t.deepEqual(all_spender_claim_for_owner_address, [['user-account.test.near','10001',false]]);

  const all_owner_rewards_for_spender_address = await user_account.call(l2e_account, 'get_all_owner_rewards_for_spender', {});
  console.log(`test_get_all_owner_rewards_for_spender after approve_for_spender: ${all_owner_rewards_for_spender_address}`)
  t.deepEqual(all_owner_rewards_for_spender_address, [["owner-account.test.near",1,100]]);

  // test transfer_nft_from
  console.log("consolelog------------------nft_token nft metadata1-------------------");
  const nft_metadata = await nft_account.view('nft_token', { token_id: '10001' });
  console.log(JSON.stringify(nft_metadata));
  setTimeout(() => {
    console.log('test_approve_for_spender sleep 5s');
  }, 5000);

  const transfer_nft_from_result = await user_account
    .call(l2e_account, 'transfer_nft_from', { 
      owner: owner_account.accountId, 
      erc721_address: nft_account.accountId 
    }, { gas: "300000000000000" });

  console.log("consolelog------------------test_transfer_nft_from-------------------");
  t.deepEqual(transfer_nft_from_result, true);
  console.log("test_transfer_nft_from end");
  console.log("consolelog------------------nft_token nft metadata2-------------------");
  const nft_metadata2= await nft_account.view('nft_token', { token_id: '10001' });
  console.log(JSON.stringify(nft_metadata2));

  console.log("consolelog------------------check near balances for spender-------------------");
  console.log(JSON.stringify((await user_account.balance()).available.toHuman()));
  console.log("consolelog------------------check ft token for spender-------------------");
  const ft_metadata = await ft_account.view('ft_balance_of', { account_id: user_account.accountId });
  console.log(JSON.stringify(ft_metadata));

  // check nft id cord
  const nft_id_cord = await owner_account.call(l2e_account, 'get_all_spender_claim_for_owner', {});
  console.log(`test_get_all_spender_claim_for_owner after transfer_nft_from end: ${nft_id_cord}`)
  t.deepEqual(nft_id_cord, [['user-account.test.near','10001',true]]);

  // test transfer_balances_from
  const transfer_balances_from_result = await user_account.call(
    l2e_account,
    'transfer_balances_from',
    {
      owner: owner_account.accountId,
      erc20: ft_account.accountId,
    }, { gas: "300000000000000" });
  console.log("consolelog------------------test_transfer_balances_from-------------------");
  // console.log(JSON.stringify(transfer_balances_from_result));
  t.deepEqual(transfer_balances_from_result, true);
  console.log("test_transfer_balances_from end");

  console.log("consolelog------------------check near balances for spender2-------------------");
  console.log(JSON.stringify((await user_account.balance()).available.toHuman()));

  console.log("consolelog------------------check ft token for spender2-------------------");
  const ft_metadata2 = await ft_account.view('ft_balance_of', { account_id: user_account.accountId });
  console.log(JSON.stringify(ft_metadata2));

  const all_spender_transfer_balances_from_end= await owner_account.call(l2e_account, 'get_all_spender_claim_for_owner', {});
  console.log(`test_get_all_spender_claim_for_owner after transfer_balances_from end: ${all_spender_transfer_balances_from_end}`)
  t.deepEqual(all_spender_transfer_balances_from_end, []);

  const all_owner_transfer_balances_from_end = await user_account.call(l2e_account, 'get_all_owner_rewards_for_spender', {});
  console.log(`test_get_all_owner_rewards_for_spender after transfer_balances_from end: ${all_owner_transfer_balances_from_end}`)
  t.deepEqual(all_owner_transfer_balances_from_end, []);

});
