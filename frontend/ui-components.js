import React from "react";

export function SignInPrompt(props) {
  return (
    <main>
      <h3>Welcome to DXH Token donation!</h3>
      <p>
        Your token will be store in our pool in NEAR blockchain. After login,
        you will choose how much token for donating to our project. We will
        share equally to all verified candidates.
      </p>
      <p>
        Do not worry, this app runs in the test network ("testnet"). It works
        just like the main network ("mainnet"), but using NEAR Tokens that are
        only for testing!
      </p>
      <br />
      <p style={{ textAlign: "center" }}>
        <button className="login-class" onClick={() => props.wallet.signIn()}>
          Sign in with NEAR Wallet
        </button>
      </p>
    </main>
  );
}



export function SignOutButton(props) {
  return <button className="login-class" style={{ float: "right" }} onClick={() => props.wallet.signOut()}>
    Sign out {props.wallet.getAccountId()}
  </button>;
}


export function GetCandidate(props) {
  
  return (
    <button className="loginClass"
      //style={{ float: "left", padding: "20px" }}
    >
      check {props.wallet.getAccountId()}
    </button>
  );
}

export function GetVerifiedCandidate(props) {
  const ct = new Contract({
    contractId: process.env.CONTRACT_NAME,
    walletToUse: props.wallet
  });
  return (
    <button className="loginClass"
      style={{ padding: "20px" }}
      onClick={async () => {
        const res = await ct.getVerifiedCandidates();
        const plain = Buffer.from(res.receipts_outcome[0].outcome.status.SuccessValue, 'base64').toString('utf8');
        console.info(plain)
      }}
    >
      check {props.wallet.getAccountId()}
    </button>
  );
}

export function DonateButton(props) {
  const ct = new Contract({
    contractId: process.env.CONTRACT_NAME,
    walletToUse: props.wallet
  });
  return (
    <div style={ {float: "left", padding: "20px"} }>
      onClick={ async () => { 
        const res = await ct.donate();
    }}
    </div>
  );
}