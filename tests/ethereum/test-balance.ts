import Web3 from "web3";
import { describe } from "mocha";
import { step } from "mocha-steps";
import { expect } from "chai";
import { HOST_URL, FAITH, FAITH_P, customRequest } from "../config";

const web3 = new Web3(HOST_URL);
describe("Test balances", () => {
	let init;
	const value = "0x200";
	step("Account has correct balance", async function () {
		init = await web3.eth.getBalance(FAITH);
		expect(Number(init)).to.be.greaterThan(Number(value));
	});

	step("Balance should be updated after transfer", async function () {
		let to = "0x1111111111111111111111111111111111111111";
		let gasPrice = "0x3B9ACA00"; // 1000000000

		let tx = await web3.eth.accounts.signTransaction(
			{
				from: FAITH,
				to: to,
				value: value,
				gasPrice: gasPrice,
				gas: "0x100000",
			},
			FAITH_P
		);
		await customRequest(web3, "eth_sendRawTransaction", [tx.rawTransaction]);

		setTimeout(async function () {
			const expectedFromBalance = (
				BigInt(init) -
				BigInt(21000) * BigInt(gasPrice) -
				BigInt(value)
			).toString();
			const expectedToBalance = Number(value).toString();

			expect(await web3.eth.getBalance(FAITH)).to.equal(expectedFromBalance);
			expect(await web3.eth.getBalance(to)).to.equal(expectedToBalance);
		}, 20000);
	});
});
