export const incrementerInfo = {
	bytecode:
		"608060405234801561001057600080fd5b5060405161018c38038061018c8339818101604052602081101561003357600080fd5b810190808051906020019092919050505080600081905550506101318061005b6000396000f3fe6080604052348015600f57600080fd5b5060043610603c5760003560e01c80637cf5dab01460415780638381f58a14606c578063d826f88f146088575b600080fd5b606a60048036036020811015605557600080fd5b81019080803590602001909291905050506090565b005b607260ec565b6040518082815260200191505060405180910390f35b608e60f2565b005b80600054016000819055503373ffffffffffffffffffffffffffffffffffffffff167fb182275171042022ff972a26edbd0171bccc74463bd22e56dbbeba4e93b7a668826040518082815260200191505060405180910390a250565b60005481565b6000808190555056fea2646970667358221220e20e87bcf3085e9302330bfcfc42af9eeff3a82cf3d5bb636c607ae9363c595a64736f6c634300060c0033",
	opcodes:
		"PUSH1 0x80 PUSH1 0x40 MSTORE CALLVALUE DUP1 ISZERO PUSH2 0x10 JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST POP PUSH1 0x40 MLOAD PUSH2 0x18C CODESIZE SUB DUP1 PUSH2 0x18C DUP4 CODECOPY DUP2 DUP2 ADD PUSH1 0x40 MSTORE PUSH1 0x20 DUP2 LT ISZERO PUSH2 0x33 JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST DUP2 ADD SWAP1 DUP1 DUP1 MLOAD SWAP1 PUSH1 0x20 ADD SWAP1 SWAP3 SWAP2 SWAP1 POP POP POP DUP1 PUSH1 0x0 DUP2 SWAP1 SSTORE POP POP PUSH2 0x131 DUP1 PUSH2 0x5B PUSH1 0x0 CODECOPY PUSH1 0x0 RETURN INVALID PUSH1 0x80 PUSH1 0x40 MSTORE CALLVALUE DUP1 ISZERO PUSH1 0xF JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST POP PUSH1 0x4 CALLDATASIZE LT PUSH1 0x3C JUMPI PUSH1 0x0 CALLDATALOAD PUSH1 0xE0 SHR DUP1 PUSH4 0x7CF5DAB0 EQ PUSH1 0x41 JUMPI DUP1 PUSH4 0x8381F58A EQ PUSH1 0x6C JUMPI DUP1 PUSH4 0xD826F88F EQ PUSH1 0x88 JUMPI JUMPDEST PUSH1 0x0 DUP1 REVERT JUMPDEST PUSH1 0x6A PUSH1 0x4 DUP1 CALLDATASIZE SUB PUSH1 0x20 DUP2 LT ISZERO PUSH1 0x55 JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST DUP2 ADD SWAP1 DUP1 DUP1 CALLDATALOAD SWAP1 PUSH1 0x20 ADD SWAP1 SWAP3 SWAP2 SWAP1 POP POP POP PUSH1 0x90 JUMP JUMPDEST STOP JUMPDEST PUSH1 0x72 PUSH1 0xEC JUMP JUMPDEST PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 RETURN JUMPDEST PUSH1 0x8E PUSH1 0xF2 JUMP JUMPDEST STOP JUMPDEST DUP1 PUSH1 0x0 SLOAD ADD PUSH1 0x0 DUP2 SWAP1 SSTORE POP CALLER PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF AND PUSH32 0xB182275171042022FF972A26EDBD0171BCCC74463BD22E56DBBEBA4E93B7A668 DUP3 PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 LOG2 POP JUMP JUMPDEST PUSH1 0x0 SLOAD DUP2 JUMP JUMPDEST PUSH1 0x0 DUP1 DUP2 SWAP1 SSTORE POP JUMP INVALID LOG2 PUSH5 0x6970667358 0x22 SLT KECCAK256 0xE2 0xE DUP8 0xBC RETURN ADDMOD 0x5E SWAP4 MUL CALLER SIGNEXTEND 0xFC 0xFC TIMESTAMP 0xAF SWAP15 0xEF RETURN 0xA8 0x2C RETURN 0xD5 0xBB PUSH4 0x6C607AE9 CALLDATASIZE EXTCODECOPY MSIZE GAS PUSH5 0x736F6C6343 STOP MOD 0xC STOP CALLER ",
	abi: [
		{
			inputs: [
				{
					internalType: "uint256",
					name: "_initialNumber",
					type: "uint256",
				},
			],
			stateMutability: "nonpayable",
			type: "constructor",
		},
		{
			anonymous: false,
			inputs: [
				{
					indexed: true,
					internalType: "address",
					name: "sender",
					type: "address",
				},
				{
					indexed: false,
					internalType: "uint256",
					name: "value",
					type: "uint256",
				},
			],
			name: "Increment",
			type: "event",
		},
		{
			inputs: [
				{
					internalType: "uint256",
					name: "_value",
					type: "uint256",
				},
			],
			name: "increment",
			outputs: [],
			stateMutability: "nonpayable",
			type: "function",
		},
		{
			inputs: [],
			name: "number",
			outputs: [
				{
					internalType: "uint256",
					name: "",
					type: "uint256",
				},
			],
			stateMutability: "view",
			type: "function",
		},
		{
			inputs: [],
			name: "reset",
			outputs: [],
			stateMutability: "nonpayable",
			type: "function",
		},
	],
};
