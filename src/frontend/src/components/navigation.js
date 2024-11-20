import Image from 'next/image';
import Link from 'next/link';
import { useEffect, useState, useContext } from 'react';

import { NearContext } from '@/wallets/near';
import NearLogo from '/public/near.svg';

import styles from '@/styles/app.module.css';
import { useTheme } from 'next-themes';

export const Navigation = () => {
  const { signedAccountId, wallet } = useContext(NearContext);
  const [action, setAction] = useState(() => { });
  const [label, setLabel] = useState('Loading...');
  const { theme, setTheme } = useTheme();

  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);

    if (!wallet) return;

    if (signedAccountId) {
      setAction(() => wallet.signOut);
      setLabel(`Logout ${signedAccountId}`);
    } else {
      setAction(() => wallet.signIn);
      setLabel('Login');
    }
  }, [signedAccountId, wallet]);

  if (!mounted) {
    return null; // 或者返回一个占位符
  }

  return (
    <nav className="navbar navbar-expand-lg">
      <div className="container-fluid">
      <Link href="/" rel="noopener noreferrer">
          <Image priority src={NearLogo} alt="NEAR" width="90" height="72" className="d-inline-block align-text-top" />
      </Link>
      </div>

      <div className="container-fluid">
      <Link
        href="/hello-near"
        className={styles.card}
        rel="noopener noreferrer"
      >
        <h3>
          Greeting
        </h3>
      </Link>
      </div>

      <div className="container-fluid">
      <Link
        href="/participant"
        className={styles.card}
        rel="noopener noreferrer"
      >
        <h3>
          Participant
        </h3>
      </Link>
      </div>

      <div className="container-fluid">
      <Link
        href="/motivator"
        className={styles.card}
        rel="noopener noreferrer"
      >
        <h3>
          Motivator
        </h3>
      </Link>
      </div>

      <div className="container-fluid">
      <Link
        href="/admin"
        className={styles.card}
        rel="noopener noreferrer"
      >
        <h3>
          Administrator
        </h3>
      </Link>
      </div>

      <div className="container-fluid">

        <div className='navbar-nav pt-1'>

        <button className="btn btn-elementary" onClick={() => setTheme(theme === 'light' ? 'dark' : 'light')}>
        Toggle {theme === 'light' ? 'Dark' : 'Light'}
        </button>

          <button className="btn btn-secondary" onClick={action} > {label} </button>
        </div>

      </div>
    </nav>
  );
};