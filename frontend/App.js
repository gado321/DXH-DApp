import 'regenerator-runtime/runtime';
import React from 'react';

import './assets/global.css';

import { EducationalText, SignInPrompt, SignOutButton } from './ui-components';


export default function App(props) {
  /// If user not signed-in with wallet - show prompt
  if (!props.isSignedIn) {
    // Sign-in flow will reload the page later
    return <SignInPrompt wallet={props.wallet}/>;
  }

  return (
    <>
      <SignOutButton wallet={props.wallet}/>
      <main className={true ? 'please-wait' : ''}>
        <h1>
          Thanks for donating!
        </h1>
      </main>
    </>
  );
}
