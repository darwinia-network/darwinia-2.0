import Web3 from "web3";
import { describe } from "mocha";
import { step } from "mocha-steps";
import { expect } from "chai";
import { HOST_URL, FAITH, FAITH_P, customRequest, DEFAULT_GAS } from "../config";
import { incrementerInfo } from "./contracts/contracts_info";
import { AbiItem } from "web3-utils";

const web3 = new Web3(HOST_URL);
describe("Test contract", () => {
	let deploy_address;

	it("Deploy first", async function () {
		let tx = await web3.eth.accounts.signTransaction(
			{
				from: FAITH,
				data: incrementerInfo.bytecode,
				gas: DEFAULT_GAS,
				nonce: await web3.eth.getTransactionCount(FAITH) + 1,
			},
			FAITH_P
		);
		let receipt = await web3.eth.sendSignedTransaction(tx.rawTransaction);
		deploy_address = receipt.contractAddress;

		console.log(deploy_address);
	});

	it("Get default number", async function () {
		const inc = new web3.eth.Contract(incrementerInfo.abi as AbiItem[], deploy_address);
		expect(await inc.methods.number().call()).to.be.equal("0");
	});
});
