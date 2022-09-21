import React from "react";
import Contract from "./near-interface";

export function SignInPrompt(props) {
  return (
    <main>
      <h3>Welcome to DXH Token donation!</h3>
      <p>
        Your token will be store in our pool in NEAR blockchain. After login,
        you will choose how much token for donating to our project. We will
        share equally to all verified candidates
      </p>
      <p>
        Do not worry, this app runs in the test network ("testnet"). It works
        just like the main network ("mainnet"), but using NEAR Tokens that are
        only for testing!
      </p>
      <br />
      <p style={{ textAlign: "center" }}>
        <button onClick={() => props.wallet.signIn()}>
          Sign in with NEAR Wallet
        </button>
      </p>
    </main>
  );
}

export function SignOutButton(props) {
  <button style={{ float: "right" }} onClick={() => props.wallet.signOut()}>
    Sign out {props.wallet.getAccountId()}
  </button>;

  /*
  const ct = new Contract({
    contractId: process.env.CONTRACT_NAME,
    walletToUse: props.wallet
  });
  return (
    <button
      style={{ float: "right" }}
      onClick={async () => {
        const res = await ct.getCandidates();
        const plain = Buffer.from(res.receipts_outcome[0].outcome.status.SuccessValue, 'base64').toString('utf8');
        console.info(plain)
      }}
    >
      Sign out {props.wallet.getAccountId()}
    </button>
  );
  */
}

export function EducationalText() {
  return (
    <>
      <p>
        Look at that! A Hello World app! This greeting is stored on the NEAR
        blockchain. Check it out:
      </p>
      <ol>
        <li>
          Look in <code>src/App.js</code> and <code>src/utils.js</code> – you'll
          see <code>get_greeting</code> and <code>set_greeting</code> being
          called on <code>contract</code>. What's this?
        </li>
        <li>
          Ultimately, this <code>contract</code> code is defined in{" "}
          <code>assembly/main.ts</code> – this is the source code for your{" "}
          <a
            target="_blank"
            rel="noreferrer"
            href="https://docs.near.org/docs/develop/contracts/overview"
          >
            smart contract
          </a>
          .
        </li>
        <li>
          When you run <code>npm run dev</code>, the code in{" "}
          <code>assembly/main.ts</code> gets deployed to the NEAR testnet. You
          can see how this happens by looking in <code>package.json</code> at
          the <code>scripts</code> section to find the <code>dev</code> command.
        </li>
      </ol>
      <hr />
      <p>
        To keep learning, check out{" "}
        <a target="_blank" rel="noreferrer" href="https://docs.near.org">
          the NEAR docs
        </a>{" "}
        or look through some{" "}
        <a target="_blank" rel="noreferrer" href="https://examples.near.org">
          example apps
        </a>
        .
      </p>
    </>
  );
}
