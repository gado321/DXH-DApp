/* Talking with a contract often involves transforming data, we recommend you to encapsulate that logic into a class */

import { utils } from "near-api-js";

export default class Contract {
  constructor({ contractId, walletToUse }) {
    this.contractId = contractId;
    this.wallet = walletToUse;
  }

  async getCandidates() {
    return await this.wallet.viewMethod({
      contractId: this.contractId,
      method: "get_candidates",
    });
  }

  async getVerifiedCandidates() {
    return await this.wallet.viewMethod({
      contractId: this.contractId,
      method: "get_verified_candidates",
    });
  }

  async setCandidate(rawCandidate) {
    console.info(rawCandidate);
    return await this.wallet.callMethod({
      contractId: this.contractId,
      method: "set_candidate",
      args: { candidate: rawCandidate },
      gas: 30000000000000
    });
  }

  async setVerifiedCandidate(verifiedCandidate, maxToken) {
    return await this.wallet.callMethod({
      contractId: this.contractId,
      method: "set_verified_candidate",
      args: { candidate: verifiedCandidate, amount: maxToken },
    });
  }

  async removeCandidate(candidate) {
    return await this.wallet.callMethod({
      contractId: this.contractId,
      args: { candidate },
      method: "remove_candidate",
    });
  }

  async removeVerifiedCandidate(candidate) {
    return await this.wallet.callMethod({
      contractId: this.contractId,
      args: { candidate },
      method: "remove_verified_candidate",
    });
  }

  async donate(amount) {
    // let deposit = utils.format.parseNearAmount(amount.toString())
    let response = await this.wallet.callMethod({
      contractId: "dev-1663407143254-90994928167650",
      method: "ft_transfer",
      args: { receiver_id: this.contractId, amount },
      deposit: 1,
      gas: 30000000000000
    });
    return response;
  }

  async triggerDonattion() {
    // let deposit = utils.format.parseNearAmount(amount.toString())
    let response = await this.wallet.callMethod({
      contractId: this.contractId,
      method: "donate",
      gas: 300000000000000
    });
    return response;
  }

  async getDonationFromTransaction(txhash) {
    let donation_amount = await this.wallet.getTransactionResult(txhash);
    return utils.format.formatNearAmount(donation_amount);
  }
}
