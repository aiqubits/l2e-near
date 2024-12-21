import { useState, useEffect, useContext } from 'react';

import { NearContext } from '@/wallets/near';
import styles from '@/styles/app.module.css';
import { HelloNearContract } from '../../config';
import { Cards } from '@/components/cards';

// Contract that the app will interact with
const CONTRACT = HelloNearContract;

export default function HelloNear() {
  const { signedAccountId, wallet } = useContext(NearContext);

  const [greeting, setGreeting] = useState('loading operation...');
  const [newGreeting, setNewGreeting] = useState('loading operation...');

  const [allRewards, setAllRewards] = useState('waiting for query...');

  const [ownerId, setOwnerId] = useState('waiting for query...');
  const [allowances, setallowances] = useState('waiting for query...');
  const [transferNftFromResult, setTransferNftFromResult] = useState('loading operation...');
  const [transferNftFrom, setTransferNftFrom] = useState({
    ownerid: '',
    nftid: '',
  });
  const [transferFtFromResult, setTransferFtFromResult] = useState('loading operation...');
  const [transferFtFrom, setTransferFtFrom] = useState({
    ownerid: '',
    ftid: '',
  });

  const transferNftFromChange = (e) => {
    e.preventDefault(); //禁用刷新
    const { name, value } = e.target;
    setTransferNftFrom((prevData) => ({
      ...prevData,
      [name]: value,
    }));
  };

  const transferBalancesFromChange = (e) => {
    e.preventDefault(); //禁用刷新
    const { name, value } = e.target;
    setTransferFtFrom((prevData) => ({
      ...prevData,
      [name]: value,
    }));
  };

  const [loggedIn, setLoggedIn] = useState(false);
  const [showSpinner, setShowSpinner] = useState(false);

  useEffect(() => {
    if (!wallet) return;

    wallet.viewMethod({ contractId: CONTRACT, method: 'get_greeting' }).then(
      greeting => setGreeting(greeting)
    );
  }, [wallet]);

  const searchRewardsForMe = async () => {
    const allRewardsResult = await wallet.callMethod({ contractId: CONTRACT, method: 'get_all_owner_rewards_for_spender', args: {} });
    setAllRewards(allRewardsResult);
  };

  const searchAllowancesForMe = async () => {
    const allowancesResult = await wallet.callMethod({ contractId: CONTRACT, method: 'get_allowances_for_spender', args: { owner: ownerId } });
    setallowances(allowancesResult);
  };

  useEffect(() => {
    setLoggedIn(!!signedAccountId);
  }, [signedAccountId]);

  const saveGreeting = async () => {
    setShowSpinner(true);
    await wallet.callMethod({ contractId: CONTRACT, method: 'set_greeting', args: { greeting: newGreeting } });
    const greeting = await wallet.viewMethod({ contractId: CONTRACT, method: 'get_greeting' });
    setGreeting(greeting);
    setShowSpinner(false);
  };

  const transferNftFromSubmit = async (e) => {
    setShowSpinner(true);
    e.preventDefault(); //禁用刷新
    // 验证逻辑
    const argsReal = {
      owner: transferNftFrom.ownerid,
    }
    if (transferNftFrom.nftid) {
      argsReal.erc721_address = transferNftFrom.nftid;
    }
    const result = await wallet.callMethod({ contractId: CONTRACT, method: 'transfer_nft_from', args: argsReal });
    setTransferNftFromResult(result)
    setShowSpinner(false);
  };

  const transferFtFromSubmit = async (e) => {
    setShowSpinner(true);
    e.preventDefault(); //禁用默认值
    // 验证逻辑
    const argsReal = {
      owner: transferFtFrom.ownerid,
    }
    if (transferFtFrom.ftid) {
      argsReal.erc20_address = transferFtFrom.ftid;
    }
    const result = await wallet.callMethod({ contractId: CONTRACT, method: 'transfer_balances_from', args: argsReal });
    setTransferFtFromResult(result)
    setShowSpinner(false);
  };

  return (
    <main className={styles.main}>
      <div className={styles.description}>
        <p>
          Interacting with the contract: &nbsp;
          <code className={styles.code}>{CONTRACT}</code>
        </p>
      </div>

      <div className={styles.center}>

        <div className="m-4">
          <h1 className="w-100">
            All Rewards List:
          </h1>
          <div hidden={!loggedIn}>
            <h1 className="w-100"><code>{allRewards}</code></h1>
            <div className="input-group" >
              <div className="input-group-append">
                <button className="btn btn-secondary" onClick={searchRewardsForMe}>
                  <span hidden={showSpinner}> SearchRewardsForMe </span>
                  <i
                    className="spinner-border spinner-border-sm"
                    hidden={!showSpinner}
                  ></i>
                </button>
              </div>
            </div>

          </div>
        </div>


        <div className="m-4">
          <h1 className="w-100">
            Allowances List:
          </h1>
          <div hidden={!loggedIn}>
            <h1 className="w-100"><code>{allowances}</code></h1>
          <div className="input-group" hidden={!loggedIn}>
            <input
              type="text"
              className="form-control w-20"
              placeholder="testnetwork.testnet"
              onChange={t => setOwnerId(t.target.value)}
              required
            />

            <div className="input-group-append full-width">
              <button className="btn btn-secondary" onClick={searchAllowancesForMe}>
                <span hidden={showSpinner}> Search Allowances For Me </span>
                <i
                  className="spinner-border spinner-border-sm"
                  hidden={!showSpinner}
                ></i>
              </button>
            </div>
          </div>

          </div>
        </div>


        <div className="m-4">
          <h1 className="w-100" >
           Claim NFT Response:
          </h1>
          <div hidden={!loggedIn}>
            <h1 className="w-100"><code>{transferNftFromResult}</code></h1>
          </div>
          <div className="input-group" hidden={!loggedIn}>
          <form onSubmit={transferNftFromSubmit}>
            <div>
              <input
                type="text"
                id="ownerid"
                name="ownerid"
                className="form-control w-20"
                placeholder="Owner Account Address"
                onChange={transferNftFromChange}
                required
              />
            </div>

            <div>
              <input
                type="text"
                id="nftid"
                name="nftid"
                className="form-control w-20"
                placeholder="NFT Address"
                onChange={transferNftFromChange}
              />
            </div>

            <div className="input-group-append">
              <button className="btn btn-secondary" type="submit">
                <span hidden={showSpinner}> Claim NFT </span>
                <i
                  className="spinner-border spinner-border-sm"
                  hidden={!showSpinner}
                ></i>
              </button>
            </div>
            </form>

          </div>
        </div>

        <div className="m-4">
          <h1 className="w-100">
            Claim Token Response:
          </h1>
          <div hidden={!loggedIn}>
            <h1 className="w-100"><code>{transferFtFromResult}</code></h1>
          </div>
          <div className="input-group" hidden={!loggedIn}>
          <form onSubmit={transferFtFromSubmit}>
            <input
              type="text"
              id="ownerid"
              name="ownerid"
              className="form-control w-20"
              placeholder="Owner Account Address"
              onChange={transferBalancesFromChange}
              required
            />
            <input
              type="text"
              id="ftid"
              name="ftid"
              className="form-control w-20"
              placeholder="FT Address"
              onChange={transferBalancesFromChange}
            />
            <div className="input-group-append">
              <button className="btn btn-secondary" type="submit">
                <span hidden={showSpinner}> Claim Token </span>
                <i
                  className="spinner-border spinner-border-sm"
                  hidden={!showSpinner}
                ></i>
              </button>
            </div>
            </form>
          </div>
        </div>

        <br />

        {/* <div className="m-4">
          <h1 className="w-100">
            Get greeting: <code>{greeting}</code>
          </h1>
          <div className="input-group" hidden={!loggedIn}>
            <input
              type="text"
              className="form-control w-20"
              placeholder="Set greeting"
              onChange={t => setNewGreeting(t.target.value)}
            />
            <div className="input-group-append">
              <button className="btn btn-secondary" onClick={saveGreeting}>
                <span hidden={showSpinner}> Set </span>
                <i
                  className="spinner-border spinner-border-sm"
                  hidden={!showSpinner}
                ></i>
              </button>
            </div>
          </div>
        </div> */}

        <div className="w-100 text-end align-text-center" hidden={loggedIn}>
          <p className="m-0"> Please login to operate </p>
        </div>

      </div>

      <Cards />
    </main>
  );
}