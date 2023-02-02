import Web3 from "web3";
import { describe } from "mocha";
import { expect } from "chai";
import { HOST_URL, BLOCK_TIMESTAMP, BLOCK_GAS_LIMIT } from "../config";

const web3 = new Web3(HOST_URL);
describe("Test Block RPC", () => {
	it("The block number should not be zero", async () => {
		expect(await web3.eth.getBlockNumber()).to.not.equal(0);
	});

	it("Get block by hash", async () => {
		let latest_block = await web3.eth.getBlock("latest");
		let block = await web3.eth.getBlock(latest_block.hash);
		expect(block.hash).to.be.equal(latest_block.hash);
	});

	it("Get block by number", async () => {
		let block = await web3.eth.getBlock(1);
		expect(block.number).not.null;
	});

	it("Get block by number", async () => {
		let block = await web3.eth.getBlock(1);
		expect(block.number).not.null;
	});

	it("Should return the genesis block", async () => {
		let block = await web3.eth.getBlock(0);
		expect(block).to.include({
			author: "0x0000000000000000000000000000000000000000",
			difficulty: "0",
			extraData: "0x",
			gasLimit: BLOCK_GAS_LIMIT,
			gasUsed: 0,
			logsBloom:
				"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
			miner: "0x0000000000000000000000000000000000000000",
			number: 0,
			receiptsRoot: "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
			size: 504,
			timestamp: 0,
			totalDifficulty: "0",
			transactionsRoot: "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
		});

		expect(block.nonce).to.eql("0x0000000000000000");
		expect(block.timestamp).to.be.a("number");
		expect(block.transactions).to.be.a("array").empty;
		expect(block.uncles).to.be.a("array").empty;
		expect(block.sha3Uncles).to.equal(
			"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"
		);
		expect(block.hash).to.be.a("string").lengthOf(66);
		expect(block.parentHash).to.be.a("string").lengthOf(66);
		expect(block.timestamp).to.be.a("number");
	});

	it("Should include previous block hash as parent", async () => {
		let block = await web3.eth.getBlock("latest");
		// previous block
		let previous_block = await web3.eth.getBlock(block.number - 1);

		expect(block.hash).to.not.equal(previous_block.hash);
		expect(block.parentHash).to.equal(previous_block.hash);
	});

	// TODO: FIX ME
	it.skip("Should have valid timestamp after block production", async () => {
		const block = await web3.eth.getBlock("latest");
		// previous block
		const previous_block = await web3.eth.getBlock(block.number - 1);

		expect(Number(block.timestamp) - Number(previous_block.timestamp)).to.be.eq(
			BLOCK_TIMESTAMP
		);
	});

	it("Should the taged block valid", async () => {
		expect((await web3.eth.getBlock("earliest")).number).to.equal(0);
		expect((await web3.eth.getBlock("latest")).number).gt(0);
	});

	// TODO: FIND A BETTER PLACE
	// it("Should get transactions count by pending block", async () => {
	// 	expect(await web3.eth.getBlockTransactionCount("pending")).to.equal(0);
	// });

	it("Should return null if the block doesnt exist", async () => {
		expect(
			await web3.eth.getBlockTransactionCount(
				"0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
			)
		).to.null;
	});

	it("Should return null when no uncle was found", async () => {
		expect(await web3.eth.getUncle(0, 0)).to.be.null;
	});
});
