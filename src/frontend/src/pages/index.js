import Image from 'next/image';

import L2eLogo from '/public/l2eq.png';
import NextLogo from '/public/next.svg';
import styles from '@/styles/app.module.css';
import { Cards } from '@/components/cards';

export default function Home() {
  return (
    <main className={styles.main}>
      <div className={styles.description}> </div>

      <div className={styles.center}>
        <Image
          className={styles.logo}
          src={L2eLogo}
          alt="L2e Logo"
          width={360}
          height={360}
          priority
        />
        {/* <h3 className="ms-2 me-3 text-dark"> + </h3>
        <Image
          className={styles.logo}
          src={NextLogo}
          alt="Next.js Logo"
          width={300 * .58}
          height={61 * .58}
          priority
        /> */}
      </div>

      <div className={styles.grid}>
        <Cards />
      </div>
    </main>
  );
}