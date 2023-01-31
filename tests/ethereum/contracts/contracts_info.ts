export const incrementerInfo = {
	bytecode:
		"608060405234801561001057600080fd5b5060e38061001f6000396000f3fe6080604052348015600f57600080fd5b5060043610603c5760003560e01c80637cf5dab01460415780638381f58a14606c578063d826f88f146088575b600080fd5b606a60048036036020811015605557600080fd5b81019080803590602001909291905050506090565b005b6072609e565b6040518082815260200191505060405180910390f35b608e60a4565b005b806000540160008190555050565b60005481565b6000808190555056fea26469706673582212205b917418df2725621d64be3f13daa34886e4d05007618ab8d690ff7ac6f8342a64736f6c634300060c0033",
	opcodes:
		"PUSH1 0x80 PUSH1 0x40 MSTORE CALLVALUE DUP1 ISZERO PUSH2 0x10 JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST POP PUSH1 0xE3 DUP1 PUSH2 0x1F PUSH1 0x0 CODECOPY PUSH1 0x0 RETURN INVALID PUSH1 0x80 PUSH1 0x40 MSTORE CALLVALUE DUP1 ISZERO PUSH1 0xF JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST POP PUSH1 0x4 CALLDATASIZE LT PUSH1 0x3C JUMPI PUSH1 0x0 CALLDATALOAD PUSH1 0xE0 SHR DUP1 PUSH4 0x7CF5DAB0 EQ PUSH1 0x41 JUMPI DUP1 PUSH4 0x8381F58A EQ PUSH1 0x6C JUMPI DUP1 PUSH4 0xD826F88F EQ PUSH1 0x88 JUMPI JUMPDEST PUSH1 0x0 DUP1 REVERT JUMPDEST PUSH1 0x6A PUSH1 0x4 DUP1 CALLDATASIZE SUB PUSH1 0x20 DUP2 LT ISZERO PUSH1 0x55 JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST DUP2 ADD SWAP1 DUP1 DUP1 CALLDATALOAD SWAP1 PUSH1 0x20 ADD SWAP1 SWAP3 SWAP2 SWAP1 POP POP POP PUSH1 0x90 JUMP JUMPDEST STOP JUMPDEST PUSH1 0x72 PUSH1 0x9E JUMP JUMPDEST PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 RETURN JUMPDEST PUSH1 0x8E PUSH1 0xA4 JUMP JUMPDEST STOP JUMPDEST DUP1 PUSH1 0x0 SLOAD ADD PUSH1 0x0 DUP2 SWAP1 SSTORE POP POP JUMP JUMPDEST PUSH1 0x0 SLOAD DUP2 JUMP JUMPDEST PUSH1 0x0 DUP1 DUP2 SWAP1 SSTORE POP JUMP INVALID LOG2 PUSH5 0x6970667358 0x22 SLT KECCAK256 JUMPDEST SWAP2 PUSH21 0x18DF2725621D64BE3F13DAA34886E4D05007618AB8 0xD6 SWAP1 SELFDESTRUCT PUSH27 0xC6F8342A64736F6C634300060C0033000000000000000000000000 ",
	abi: [
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