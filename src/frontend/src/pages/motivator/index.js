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

  const [allSpenderClaim, setAllSpenderClaim] = useState('loading...');

  const [approvedForSpenderResult, setApproveForSpenderResult] = useState('loading...');
  const [approvedForSpender, setApproveForSpender] = useState({
    spenderid: '',
    mainamount: '',
    ftamount: '',
    tokenmetadata: '',
    ftid: '',
    nftid: '',
  });
  const approveForSpenderChange = (e) => {
    e.preventDefault(); //禁用默认值
    const { name, value } = e.target;
    setApproveForSpender((prevData) => ({
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

  useEffect(() => {
    if (!wallet) return;

    wallet.viewMethod({ contractId: CONTRACT, method: 'get_all_spender_claim_for_owner' }).then(
      allSpenderClaim => setAllSpenderClaim(allSpenderClaim)
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

  const approveForSpenderSubmit = async () => {
    setShowSpinner(true);
    const result = await wallet.callMethod({ 
      contractId: CONTRACT, 
      method: 'approve_for_spender', 
      args: { 
        spender: approvedForSpender.spenderid, 
        main_token_amount: approvedForSpender.mainamount, 
        ft_token_amount: approvedForSpender.ftamount, 
        token_metadata: approvedForSpender.tokenmetadata, 
        ft_token_id: approvedForSpender.ftid, 
        nft_token_id: approvedForSpender.nftid 
      } 
    });
    setApproveForSpenderResult(result);
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
            All Spenders Claim List:
          </h1>
          <div hidden={!loggedIn}>
            <h1 className="w-100"><code>{allSpenderClaim}</code></h1>
          </div>

        </div>

        <div className="m-4">
          <h1 className="w-100">
            Approve For Spender Response:
          </h1>
          <div hidden={!loggedIn}>
            <h1 className="w-100"><code>{approvedForSpenderResult}</code></h1>
          </div>
          <div className="input-group" hidden={!loggedIn}>
            <form onSubmit={approveForSpenderSubmit}>
              <div>
                <input
                  type="text"
                  id="spenderid"
                  name="spenderid"
                  className="form-control w-20"
                  placeholder="Spender Account ID"
                  onChange={approveForSpenderChange}
                  required
                />
              </div>

              <div>
                <input
                  type="text"
                  id="mainamount"
                  name="mainamount"
                  className="form-control w-20"
                  placeholder="Main Token Amount"
                  onChange={approveForSpenderChange}
                  required
                />
              </div>

              <div>
                <input
                  type="text"
                  id="ftamount"
                  name="ftamount"
                  className="form-control w-20"
                  placeholder="FT Amount"
                  onChange={approveForSpenderChange}
                  required
                />
              </div>

              <div>
                <input
                  type="text"
                  id="tokenmetadata"
                  name="tokenmetadata"
                  className="form-control w-20"
                  placeholder="Main Token Amount"
                  onChange={approveForSpenderChange}
                />
              </div>

              <div>
                <input
                  type="text"
                  id="ftid"
                  name="ftid"
                  className="form-control w-20"
                  placeholder="FT Address"
                  onChange={approveForSpenderChange}
                />
              </div>

              <div>
                <input
                  type="text"
                  id="nftid"
                  name="nftid"
                  className="form-control w-20"
                  placeholder="NFT Address"
                  onChange={approveForSpenderChange}
                />
              </div>

              <div className="input-group-append">
                <button className="btn btn-secondary" type="submit">
                  <span hidden={showSpinner}> Approve For Spender </span>
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

        <div className="m-4">
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
        </div>


        <div className="w-100 text-end align-text-center" hidden={loggedIn}>
          <p className="m-0"> Please login to change the state </p>
        </div>

      </div>

      <Cards />
    </main>
  );
}