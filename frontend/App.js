import "regenerator-runtime/runtime";
import React from "react";
import { Table } from "antd";
import {
  AndroidOutlined,
  AppleOutlined,
  CodeSandboxCircleFilled,
} from "@ant-design/icons";
import { Tabs } from "antd";
import { Layout } from "antd";
import { useState, useEffect, useRef } from "react";
const { v4: uuidv4 } = require("uuid");
import Contract from "./near-interface";
import "./assets/global.css";

import {
  EducationalText,
  GetCandidate,
  SignInPrompt,
  SignOutButton,
} from "./ui-components";

const { Header, Footer, Sider, Content } = Layout;

export default function App(props) {
  /// If user not signed-in with wallet - show prompt
  if (!props.isSignedIn) {
    // Sign-in flow will reload the page later
    return <SignInPrompt wallet={props.wallet} />;
  }

  const candidateColumns = [
    {
      title: "Candidate Profiles",
      dataIndex: "candidate",
      width: "100%",
    },
  ];
  const verifiedCandidateColumns = [
    {
      title: "Verified Candidate Profiles",
      dataIndex: "candidate",
      width: "100%",
    },
  ];

  const [candidateProfiles, setCandidateProfiles] = useState([]);
  const [verifiedCandidateProfiles, setVerifiedCandidateProfiles] = useState(
    []
  );
  const getCandidates = async () => {
    const ct = new Contract({
      contractId: process.env.CONTRACT_NAME,
      walletToUse: props.wallet,
    });
    let res = await ct.getCandidates();
    setCandidateProfiles(res);
  };
  const getVerifiedCandidates = async () => {
    const ct = new Contract({
      contractId: process.env.CONTRACT_NAME,
      walletToUse: props.wallet,
    });
    let res = await ct.getVerifiedCandidates();
    let candidates = [];
    for (const x of res) {
      candidates.push({ candidate: x, key: uuidv4() });
    }
    setVerifiedCandidateProfiles(candidates);
  };
  useEffect(() => {
    getCandidates();
    getVerifiedCandidates();
  }, []);

  const donorInputRef = useRef(null);
  const candidateInputRef = useRef(null);

  const donate = async () => {
    const ct = new Contract({
      contractId: process.env.CONTRACT_NAME,
      walletToUse: props.wallet,
    });
    await ct.donate(donorInputRef.current.value);
    alert("done!");
  };

  const submitCandidateProfile = async () => {
    const ct = new Contract({
      contractId: process.env.CONTRACT_NAME,
      walletToUse: props.wallet,
    });
    await ct.setCandidate(JSON.stringify({
      data: candidateInputRef.current.value,
      publicKey: props.wallet.accountId,
    }));
    alert("done!");
  };

  const triggerDonation = async () => {
    const ct = new Contract({
      contractId: process.env.CONTRACT_NAME,
      walletToUse: props.wallet,
    });
    const res = await ct.triggerDonattion();
    alert("done!");
  };

  const setVerifiedCandidate = async (item) => {
    const ct = new Contract({
      contractId: process.env.CONTRACT_NAME,
      walletToUse: props.wallet,
    });
    const res = await ct.setVerifiedCandidate(item, "100");
    console.info("done");
    alert("done!");
  };

  const removeVerifiedCandidate = async (item) => {
    const ct = new Contract({
      contractId: process.env.CONTRACT_NAME,
      walletToUse: props.wallet,
    });
    const res = await ct.removeVerifiedCandidate(item);
    alert("done!");
  };

  const removeCandidate = async (item) => {
    const ct = new Contract({
      contractId: process.env.CONTRACT_NAME,
      walletToUse: props.wallet,
    });
    console.info(item);
    const res = await ct.removeCandidate(item);
    console.info(res);
    alert("done!");
  };
  return (
    <>
      <main>
        <div className="content">
          <div className="header">
            <SignOutButton wallet={props.wallet} />
          </div>
          <div className="candidate-table-place">
            {props.wallet.accountId !== "upi05.testnet" ? (
              <div className="verified">
                <Table
                  style={{}}
                  columns={verifiedCandidateColumns}
                  dataSource={verifiedCandidateProfiles}
                  pagination={{
                    pageSize: 50,
                  }}
                  scroll={{
                    y: 240,
                  }}
                />
              </div>
            ) : (
              <div className="raw">
                <table>
                  <thead>Candidate Profiles</thead>
                  <tbody>
                    {candidateProfiles.map((item, i) => {
                      return [
                        <tr key={i}>
                          <td>{item}</td>
                          <td>
                            <button onClick={() => setVerifiedCandidate(item)}>
                              verify
                            </button>
                          </td>
                          <td>
                            <button onClick={() => removeCandidate(item)}>
                              remove
                            </button>
                          </td>
                        </tr>,
                      ];
                    })}
                  </tbody>
                </table>
              </div>
            )}
          </div>
          {props.wallet.accountId === "upi05.testnet" ? (
            <div
              className="submit-place"
              style={{
                padding: "20px",
                marginTop: "50px",
                marginLeft: "10px",
                borderRadius: "10px",
              }}
            >
              <div className="donate-form">
                <form>
                  <label>
                    For validators:
                    <input
                      type="button"
                      onClick={triggerDonation}
                      style={{ marginTop: "15px" }}
                      value="Trigger donation from pool"
                    />
                  </label>
                </form>
              </div>
            </div>
          ) : (
            <div
              className="submit-place"
              style={{
                padding: "20px",
                marginTop: "50px",
                marginLeft: "10px",
                borderRadius: "10px",
              }}
            >
              <div className="donate-form">
                <form>
                  <label>
                    For donors:
                    <input
                      type="text"
                      name="Number of Token"
                      placeholder="Number of token"
                      style={{ marginTop: "10px", width: "170px" }}
                      ref={donorInputRef}
                    />
                  </label>
                  <input
                    type="button"
                    onClick={donate}
                    style={{ marginTop: "15px" }}
                    value="Donate"
                  />
                </form>
              </div>

              <div className="submit-form" style={{ marginTop: "70px" }}>
                <form>
                  <label>
                    For candidates:
                    <textarea
                      type="text"
                      name="Cadidate profile"
                      style={{ width: "100%", backgroundColor: "gray" }}
                      placeholder="Candidate profile..."
                      ref={candidateInputRef}
                    />
                  </label>
                  <input
                    type="button"
                    onClick={submitCandidateProfile}
                    style={{ marginTop: "10px" }}
                    value="Submit"
                  />
                </form>
              </div>
            </div>
          )}
        </div>
      </main>
    </>
  );
}
