import Web3 from "web3";
import { describe } from "mocha";
import { step } from "mocha-steps";
import { expect } from "chai";
import { HOST_URL, FAITH, FAITH_P, DEFAULT_GAS } from "../config";
import { opcodesInfo } from "./contracts/contracts_info";
import { AbiItem } from "web3-utils";

const web3 = new Web3(HOST_URL);
describe("Test solidity opcodes", () => {
	web3.eth.accounts.wallet.add(FAITH_P);
	let contract_address;

	step("Opcodes should works", async function () {
		let tx = await web3.eth.accounts.signTransaction(
			{
				from: FAITH,
				data: opcodesInfo.bytecode,
				gas: DEFAULT_GAS,
			},
			FAITH_P
		);
		let receipt = await web3.eth.sendSignedTransaction(tx.rawTransaction);

		expect(receipt.transactionHash).to.not.be.null;
		contract_address = receipt.contractAddress;

		const opcodes = new web3.eth.Contract(opcodesInfo.abi as AbiItem[], contract_address);
		await opcodes.methods.test().call();
		await opcodes.methods.test_stop().call();
	}).timeout(60000);

	step("Call invalid opcode", async function () {
		const opcodes = new web3.eth.Contract(opcodesInfo.abi as AbiItem[], contract_address);
		try {
			await opcodes.methods.test_invalid().send({
				from: FAITH,
				to: contract_address,
				gas: DEFAULT_GAS,
			});
		} catch (receipt) {
			expect(receipt.receipt.status).to.be.false;
		}
	}).timeout(60000);

	step("Call revert opcode", async function () {
		const opcodes = new web3.eth.Contract(opcodesInfo.abi as AbiItem[], contract_address);
		try {
			await opcodes.methods.test_revert().send({
				from: FAITH,
				to: contract_address,
				gas: DEFAULT_GAS,
			});
		} catch (receipt) {
			expect(receipt.receipt.status).to.be.false;
		}
	}).timeout(60000);
});
