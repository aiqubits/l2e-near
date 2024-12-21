import { useState, useEffect, useContext } from 'react';
import { utils  } from 'near-api-js';
import { NearContext, THIRTY_TGAS  } from '@/wallets/near';
import styles from '@/styles/app.module.css';
import { HelloNearContract } from '../../config';
import { Cards } from '@/components/cards';

// Contract that the app will interact with
const CONTRACT = HelloNearContract;

export default function HelloNear() {
  const { signedAccountId, wallet } = useContext(NearContext);

  const [greeting, setGreeting] = useState('loading...');
  const [newGreeting, setNewGreeting] = useState('loading...');

  const [allSpenderClaim, setAllSpenderClaim] = useState('waiting for query...');

  const [approvedForSpenderResult, setApproveForSpenderResult] = useState('loading operation...');
  const [approvedForSpender, setApproveForSpender] = useState({
    spenderid: '',
    mainamount: '',
    ftamount: '',
    tokenmetadata: '',
    ftid: '',
    nftid: '',
  });
  const approveForSpenderChange = (e) => {
    e.preventDefault(); //禁用刷新
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

  // 不使用useEffect，而使用手动点击按钮查询的方式
  const searchForMe = async () => {
    // let all_spender_result = new Array();
    const all_spender_result = await wallet.callMethod({ contractId: CONTRACT, method: 'get_all_spender_claim_for_owner', args: {} });
    console.log("++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
    console.log(all_spender_result);
    // console.log(all_spender_result[0]);

    setAllSpenderClaim(all_spender_result);
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

  const approveForSpenderSubmit = async (e) => {
    e.preventDefault(); //禁用默认刷新
    console.log("+-=--------------------------------------------");
    setShowSpinner(true);
    const argsReal = {
      spender: approvedForSpender.spenderid, 
      main_token_amount: utils.format.parseNearAmount(approvedForSpender.mainamount),
      ft_amount: utils.format.parseNearAmount(approvedForSpender.ftamount),
    };

    if (approvedForSpender.tokenmetadata) {
      argsReal.token_metadata = approvedForSpender.tokenmetadata;
    }

    if (approvedForSpender.ftid) {
      argsReal.erc20_address = approvedForSpender.ftid;
    }

    if (approvedForSpender.nftid) {
      argsReal.erc721_address = approvedForSpender.nftid;
    }
    console.log("++++++++++++++++++++++++argsReal.main_token_amount++++++++++++++++++++++++++++++++");
    console.log(argsReal.main_token_amount);
    console.log(argsReal);
    const result = await wallet.callMethod({
      contractId: CONTRACT, 
      method: 'approve_for_spender', 
      args: argsReal,
      gas: 30000000000000*3,
      deposit: argsReal.main_token_amount, // YoctoNear*3  10_u128.pow(24)
    });
    console.log('-------------------------------result-----------------------------------');
    console.log(result);
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
            <div className="input-group-append">
                <button className="btn btn-secondary" onClick={searchForMe}>
                  <span hidden={showSpinner}> Search For Me </span>
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
            Approve For Spender Response:
          </h1>
          <div hidden={!loggedIn}>
            <h1 className="w-100"><code>{approvedForSpenderResult}</code></h1>
          </div>
          <div className="input-group" hidden={!loggedIn}>
            <form method="post" onSubmit={approveForSpenderSubmit}>
              <div>
                <input
                  type="text"
                  id="spenderid"
                  name="spenderid"
                  className="form-control w-20"
                  placeholder="Spender Account ID"
                  value={approvedForSpender.spenderid}
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
                  value={approvedForSpender.mainamount}
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
                  value={approvedForSpender.ftamount}
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
                  placeholder="Token Metadata"
                  value={approvedForSpender.tokenmetadata}
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
                  value={approvedForSpender.ftid}
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
                  value={approvedForSpender.nftid}
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