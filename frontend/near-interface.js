/* Talking with a contract often involves transforming data, we recommend you to encapsulate that logic into a class */

import { utils } from 'near-api-js'

export class Contract {

  constructor({ contractId, walletToUse }) {
    this.contractId = contractId;
    this.wallet = walletToUse;
  }

  async getCandidates() {
    return await this.wallet.viewMethod({contractId: this.contractId, method: "get_candidates"});
  }

  async getVerifiedCandidates() {
    return await this.wallet.viewMethod({contractId: this.contractId, method: "get_verified_candidates"});
  }

  async setCandidate(rawCandidate, maxToken) {
    return await this.wallet.callMethod({contractId: this.contractId, method: "set_candidates", args:{candidate: rawCandidate, amount: maxToken}});
  }

  async setVerifiedCandidate(verifiedCandidate) {
    return await this.wallet.callMethod({contractId: this.contractId, method: "set_verified_candidates", args:{candidate: verifiedCandidate}});
  }

  async removeCandidate() {
    return await this.wallet.callMethod({contractId: this.contractId, method: "remove_candidates"});
  }

  async removeVerifiedCandidate() {
    return await this.wallet.callMethod({contractId: this.contractId, method: "remove_verified_candidates"});
  }

  async donate(amount) {
    let deposit = utils.format.parseNearAmount(amount.toString())
    let response = await this.wallet.callMethod({ contractId: this.contractId, method: "donate", deposit })
    return response
  }

  async getDonationFromTransaction(txhash) {
    let donation_amount = await this.wallet.getTransactionResult(txhash);
    return utils.format.formatNearAmount(donation_amount);
  }
}