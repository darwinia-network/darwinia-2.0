import Web3 from "web3";
import { describe } from "mocha";
import { step } from "mocha-steps";
import { expect } from "chai";
import { HOST_URL, FAITH, FAITH_P, DEFAULT_GAS } from "../config";
import { incrementerInfo } from "./contracts/contracts_info";
import { AbiItem } from "web3-utils";

const web3 = new Web3(HOST_URL);
describe("Test contract", () => {
	const inc = new web3.eth.Contract(incrementerInfo.abi as AbiItem[]);
	let contract_address;

	step("Deploy contract", async () => {
		let data = inc.deploy({
			data: incrementerInfo.bytecode,
			arguments: [5],
		});
		let tx = await web3.eth.accounts.signTransaction(
			{
				from: FAITH,
				data: data.encodeABI(),
				gas: DEFAULT_GAS,
			},
			FAITH_P
		);
		let receipt = await web3.eth.sendSignedTransaction(tx.rawTransaction);

		expect(receipt.transactionHash).to.not.be.null;
		contract_address = receipt.contractAddress;
	}).timeout(60000);

	step("Get default number", async function () {
		const inc = new web3.eth.Contract(incrementerInfo.abi as AbiItem[], contract_address);
		expect(await inc.methods.number().call()).to.be.equal("5");
	});

	step("Increase number", async function () {
		const inc = new web3.eth.Contract(incrementerInfo.abi as AbiItem[], contract_address);
		let tx = await web3.eth.accounts.signTransaction(
			{
				from: FAITH,
				to: contract_address,
				data: inc.methods.increment(3).encodeABI(),
				gas: DEFAULT_GAS,
			},
			FAITH_P
		);
		let receipt = await web3.eth.sendSignedTransaction(tx.rawTransaction);

		expect(receipt.transactionHash).to.not.be.null;
		expect(await inc.methods.number().call()).to.be.equal("8");
	}).timeout(60000);

	step("Reset number", async function () {
		const inc = new web3.eth.Contract(incrementerInfo.abi as AbiItem[], contract_address);
		let tx = await web3.eth.accounts.signTransaction(
			{
				from: FAITH,
				to: contract_address,
				data: inc.methods.reset().encodeABI(),
				gas: DEFAULT_GAS,
			},
			FAITH_P
		);
		let receipt = await web3.eth.sendSignedTransaction(tx.rawTransaction);

		expect(receipt.transactionHash).to.not.be.null;
		expect(await inc.methods.number().call()).to.be.equal("0");
	}).timeout(60000);
});
