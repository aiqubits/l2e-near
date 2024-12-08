import { useState, useEffect, useContext } from 'react';

import { NearContext } from '@/wallets/near';
import styles from '@/styles/app.module.css';
import { HelloNearContract } from '../../config';
import { Cards } from '@/components/cards';

// Contract that the app will interact with
const CONTRACT = HelloNearContract;

export default function HelloNear() {
  const { signedAccountId, wallet } = useContext(NearContext);

  const [greeting, setGreeting] = useState('loading...');
  const [newGreeting, setNewGreeting] = useState('loading...');

  const [ftAddress, setFtAddress] = useState('waiting for query...');
  const [nftAddress, setNftAddress] = useState('waiting for query...');
  const [contractAddressResult, setContractAddressResult] = useState('loading operation...');
  const [contractAddress, setContractAddress] = useState({
    ftid: '',
    nftid: '',
  });

  const addContractAddressChange = (e) => {
    e.preventDefault(); //禁用默认值
    const { name, value } = e.target;
    setContractAddress((prevData) => ({
      ...prevData,
      [name]: value,
    }));
  };

  const [authTokenOwnerAddress, setGetAuthTokenOwnerAddress] = useState('loading operation...');
  const [newAuthTokenOwnerAddress, setNewAuthTokenOwnerAddress] = useState('loading operation...');

  const [adminAddress, setAdminAddress] = useState('loading operation...');
  const [newAdminAddress, setNewAdminAddress] = useState('loading operation...');


  const [loggedIn, setLoggedIn] = useState(false);
  const [showSpinner, setShowSpinner] = useState(false);

  useEffect(() => {
    if (!wallet) return;

    wallet.viewMethod({ contractId: CONTRACT, method: 'get_erc20_address' }).then(
      ftAddress => setFtAddress(ftAddress)
    );
  }, [wallet]);

  useEffect(() => {
    if (!wallet) return;

    wallet.viewMethod({ contractId: CONTRACT, method: 'get_erc721_address' }).then(
      nftAddress => setNftAddress(nftAddress)
    );
  }, [wallet]);

  useEffect(() => {
    if (!wallet) return;

    wallet.viewMethod({ contractId: CONTRACT, method: 'get_admin_address' }).then(
      adminAddress => setAdminAddress(adminAddress)
    );
  }, [wallet]);

  useEffect(() => {
    if (!wallet) return;

    wallet.viewMethod({ contractId: CONTRACT, method: 'get_auth_token_owner' }).then(
      authTokenOwnerAddress => setGetAuthTokenOwnerAddress(authTokenOwnerAddress)
    );
  }, [wallet]);

  useEffect(() => {
    if (!wallet) return;

    wallet.viewMethod({ contractId: CONTRACT, method: 'get_greeting' }).then(
      greeting => setGreeting(greeting)
    );
  }, [wallet]);

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

  const addContractAddressSubmit = async (e) => {
    e.preventDefault(); //禁用默认值
    // todo 验证逻辑
    const result = await wallet.callMethod({ contractId: CONTRACT, method: 'add_contract_address', args: { erc20_address: contractAddress.ftid, erc721_address: contractAddress.nftid } });
    setContractAddressResult(result)
    const ftAddress = await wallet.viewMethod({ contractId: CONTRACT, method: 'get_erc20_address' });
    const nftAddress = await wallet.viewMethod({ contractId: CONTRACT, method: 'get_erc721_address' });

    setFtAddress(ftAddress)
    setNftAddress(nftAddress)
  };

  const addAuthTokenOwnerAddress = async () => {
    setShowSpinner(true);
    await wallet.callMethod({ contractId: CONTRACT, method: 'add_auth_token_owner', args: { owner_address: newAuthTokenOwnerAddress } });
    const authTokenOwnerAddress = await wallet.viewMethod({ contractId: CONTRACT, method: 'get_auth_token_owner' });
    setGetAuthTokenOwnerAddress(authTokenOwnerAddress);

    setShowSpinner(false);
  }

  const addAdminAddress = async () => {
    setShowSpinner(true);
    await wallet.callMethod({ contractId: CONTRACT, method: 'add_admin_address', args: { new_admin_address: newAdminAddress } });
    const adminAddress = await wallet.viewMethod({ contractId: CONTRACT, method: 'get_admin_address' });
    setAdminAddress(adminAddress);

    setShowSpinner(false);
  }

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
            <p>FT Address List: <code hidden={!loggedIn}>{ftAddress}</code></p>
            <p>NFT Address List: <code hidden={!loggedIn}>{nftAddress}</code></p>
          </h1>
          <h1>
             Add Contract Address:
          </h1>
          <div hidden={!loggedIn}>
            <h1 className="w-100"><code>{contractAddressResult}</code></h1>
          </div>
          <form onSubmit={addContractAddressSubmit} hidden={!loggedIn}>
            <div>
              <input
                type="text"
                id="ftid"
                name="ownerid"
                className="form-control w-20"
                placeholder="FT Address"
                onChange={addContractAddressChange}
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
                onChange={addContractAddressChange}
                required
              />
            </div>

            <div className="input-group-append">
              <button className="btn btn-secondary" type="submit">
                <span hidden={showSpinner}> Add Contract address </span>
                <i
                  className="spinner-border spinner-border-sm"
                  hidden={!showSpinner}
                ></i>
              </button>
            </div>
          </form>

        </div>

        <div className="m-4">
          <h1 className="w-100">
            Auth Address List: <code hidden={!loggedIn}>{authTokenOwnerAddress}</code>
          </h1>
          <div className="input-group" hidden={!loggedIn}>
            <input
              type="text"
              className="form-control w-20"
              placeholder="Add Auth Token Owner Address"
              onChange={t => setNewAuthTokenOwnerAddress(t.target.value)}
            />
            <div className="input-group-append">
              <button className="btn btn-secondary" onClick={addAuthTokenOwnerAddress}>
                <span hidden={showSpinner}> Add </span>
                <i
                  className="spinner-border spinner-border-sm"
                  hidden={!showSpinner}
                ></i>
              </button>
            </div>
          </div>
        </div>

        <div className="m-4">
          <h1 className="w-100">
            Admin Address List: <code hidden={!loggedIn}>{adminAddress}</code>
          </h1>
          <div className="input-group" hidden={!loggedIn}>
            <input
              type="text"
              className="form-control w-20"
              placeholder="Add Admin Address"
              onChange={t => setNewAdminAddress(t.target.value)}
            />
            <div className="input-group-append">
              <button className="btn btn-secondary" onClick={addAdminAddress}>
                <span hidden={showSpinner}> Add </span>
                <i
                  className="spinner-border spinner-border-sm"
                  hidden={!showSpinner}
                ></i>
              </button>
            </div>
          </div>

        </div>

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