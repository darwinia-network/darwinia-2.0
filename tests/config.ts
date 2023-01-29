import Web3 from "web3";
import { JsonRpcResponse } from "web3-core-helpers";

export const CHAIN_ID = 43;
export const HOST_URL = "http://127.0.0.1:9933";
export const BLOCK_TIMESTAMP = 12; // 12 seconds per para chain block
// TODO: CHECK AGAIN
export const BLOCK_GAS_LIMIT = 9375000;

// Accounts builtin
export const FAITH = "0xC0F0f4ab324C46e55D02D0033343B4Be8A55532d";
export const FAITH_P = "0xb9d2ea9a615f3165812e8d44de0d24da9bbd164b65c4f0573e1ce2c8dbd9c8df";

export function customRequest(web3: Web3, method: string, params: any[]) {
	return new Promise<JsonRpcResponse>((resolve, reject) => {
		(web3.currentProvider as any).send(
			{
				jsonrpc: "2.0",
				id: 1,
				method,
				params,
			},
			(error: Error | null, result?: JsonRpcResponse) => {
				if (error) {
					reject(
						`Failed to send custom request (${method} (${params.join(",")})): ${
							error.message || error.toString()
						}`
					);
				}
				resolve(result);
			}
		);
	});
}
