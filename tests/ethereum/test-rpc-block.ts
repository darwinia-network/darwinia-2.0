import Web3 from "web3";
import { describe } from 'mocha';
import { expect  } from "chai";
import { config } from "../config";

const web3 = new Web3(config.host);
describe('Test Block RPC', () => {
	it("The block number should not be zero", async function () {
		expect(await web3.eth.getBlockNumber()).to.not.equal(0);
	});

	


});
