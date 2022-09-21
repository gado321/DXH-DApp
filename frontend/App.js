import 'regenerator-runtime/runtime';
import React from 'react';

import './assets/global.css';

import { EducationalText, SignInPrompt, SignOutButton } from './ui-components';


export default function App() {

  /// If user not signed-in with wallet - show prompt
  if (!window.walletConnection.isSignedIn()) {
    // Sign-in flow will reload the page later
    return <SignInPrompt/>;
  }

  return (
    <>
      <SignOutButton accountId={window.accountId}/>
      <main className={uiPleaseWait ? 'please-wait' : ''}>
        <h1>
          Thanks for donating!
        </h1>
        <EducationalText/>
      </main>
    </>
  );
}
