import { useEffect, useState } from 'react';

import '@/styles/globals.css';
import { Navigation } from '@/components/navigation';

import { Wallet, NearContext } from '@/wallets/near';
import { NetworkId } from '@/config';
import { ThemeProvider } from 'next-themes';

const wallet = new Wallet({ networkId: NetworkId });

export default function MyApp({ Component, pageProps }) {
  const [signedAccountId, setSignedAccountId] = useState('');

  useEffect(() => { wallet.startUp(setSignedAccountId) }, []);

  return (
    <ThemeProvider forcedTheme={pageProps.initialTheme || undefined} defaultTheme="system" attribute="class">
    <NearContext.Provider value={{ wallet, signedAccountId }}>
      <Navigation />
      <Component {...pageProps} />
    </NearContext.Provider>
    </ThemeProvider>
  );
}
