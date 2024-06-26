const Web3 = require('web3');

// Initialization
const bytecode = '608060405234801561001057600080fd5b50604051610ca9380380610ca98339818101604052602081101561003357600080fd5b8101908080519060200190929190505050806002819055506002546000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000208190555050610c09806100a06000396000f3fe608060405234801561001057600080fd5b50600436106100935760003560e01c8063313ce56711610066578063313ce5671461022557806370a082311461024957806395d89b41146102a1578063a9059cbb14610324578063dd62ed3e1461038a57610093565b806306fdde0314610098578063095ea7b31461011b57806318160ddd1461018157806323b872dd1461019f575b600080fd5b6100a0610402565b6040518080602001828103825283818151815260200191508051906020019080838360005b838110156100e05780820151818401526020810190506100c5565b50505050905090810190601f16801561010d5780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b6101676004803603604081101561013157600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff1690602001909291908035906020019092919050505061043b565b604051808215151515815260200191505060405180910390f35b61018961052d565b6040518082815260200191505060405180910390f35b61020b600480360360608110156101b557600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190803573ffffffffffffffffffffffffffffffffffffffff16906020019092919080359060200190929190505050610537565b604051808215151515815260200191505060405180910390f35b61022d6108b2565b604051808260ff1660ff16815260200191505060405180910390f35b61028b6004803603602081101561025f57600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff1690602001909291905050506108b7565b6040518082815260200191505060405180910390f35b6102a96108ff565b6040518080602001828103825283818151815260200191508051906020019080838360005b838110156102e95780820151818401526020810190506102ce565b50505050905090810190601f1680156103165780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b6103706004803603604081101561033a57600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff16906020019092919080359060200190929190505050610938565b604051808215151515815260200191505060405180910390f35b6103ec600480360360408110156103a057600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190803573ffffffffffffffffffffffffffffffffffffffff169060200190929190505050610b1a565b6040518082815260200191505060405180910390f35b6040518060400160405280600a81526020017f455243323042617369630000000000000000000000000000000000000000000081525081565b600081600160003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925846040518082815260200191505060405180910390a36001905092915050565b6000600254905090565b60008060008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205482111561058457600080fd5b600160008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205482111561060d57600080fd5b61065e826000808773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054610ba190919063ffffffff16565b6000808673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000208190555061072f82600160008773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054610ba190919063ffffffff16565b600160008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550610800826000808673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054610bb890919063ffffffff16565b6000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef846040518082815260200191505060405180910390a3600190509392505050565b601281565b60008060008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020549050919050565b6040518060400160405280600381526020017f425343000000000000000000000000000000000000000000000000000000000081525081565b60008060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205482111561098557600080fd5b6109d6826000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054610ba190919063ffffffff16565b6000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550610a69826000808673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054610bb890919063ffffffff16565b6000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef846040518082815260200191505060405180910390a36001905092915050565b6000600160008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054905092915050565b600082821115610bad57fe5b818303905092915050565b600080828401905083811015610bca57fe5b809150509291505056fea265627a7a72315820b5898fcfe77f8de11c039ca37a3771a76e80988b1a6b000a524fbaea56c721a164736f6c63430005110032';
const abi = [
   {
      "inputs": [
         {
            "internalType": "uint256",
            "name": "total",
            "type": "uint256"
         }
      ],
      "payable": false,
      "stateMutability": "nonpayable",
      "type": "constructor"
   },
   {
      "anonymous": false,
      "inputs": [
         {
            "indexed": true,
            "internalType": "address",
            "name": "tokenOwner",
            "type": "address"
         },
         {
            "indexed": true,
            "internalType": "address",
            "name": "spender",
            "type": "address"
         },
         {
            "indexed": false,
            "internalType": "uint256",
            "name": "tokens",
            "type": "uint256"
         }
      ],
      "name": "Approval",
      "type": "event"
   },
   {
      "anonymous": false,
      "inputs": [
         {
            "indexed": true,
            "internalType": "address",
            "name": "from",
            "type": "address"
         },
         {
            "indexed": true,
            "internalType": "address",
            "name": "to",
            "type": "address"
         },
         {
            "indexed": false,
            "internalType": "uint256",
            "name": "tokens",
            "type": "uint256"
         }
      ],
      "name": "Transfer",
      "type": "event"
   },
   {
      "constant": true,
      "inputs": [
         {
            "internalType": "address",
            "name": "owner",
            "type": "address"
         },
         {
            "internalType": "address",
            "name": "delegate",
            "type": "address"
         }
      ],
      "name": "allowance",
      "outputs": [
         {
            "internalType": "uint256",
            "name": "",
            "type": "uint256"
         }
      ],
      "payable": false,
      "stateMutability": "view",
      "type": "function"
   },
   {
      "constant": false,
      "inputs": [
         {
            "internalType": "address",
            "name": "delegate",
            "type": "address"
         },
         {
            "internalType": "uint256",
            "name": "numTokens",
            "type": "uint256"
         }
      ],
      "name": "approve",
      "outputs": [
         {
            "internalType": "bool",
            "name": "",
            "type": "bool"
         }
      ],
      "payable": false,
      "stateMutability": "nonpayable",
      "type": "function"
   },
   {
      "constant": true,
      "inputs": [
         {
            "internalType": "address",
            "name": "tokenOwner",
            "type": "address"
         }
      ],
      "name": "balanceOf",
      "outputs": [
         {
            "internalType": "uint256",
            "name": "",
            "type": "uint256"
         }
      ],
      "payable": false,
      "stateMutability": "view",
      "type": "function"
   },
   {
      "constant": true,
      "inputs": [],
      "name": "decimals",
      "outputs": [
         {
            "internalType": "uint8",
            "name": "",
            "type": "uint8"
         }
      ],
      "payable": false,
      "stateMutability": "view",
      "type": "function"
   },
   {
      "constant": true,
      "inputs": [],
      "name": "name",
      "outputs": [
         {
            "internalType": "string",
            "name": "",
            "type": "string"
         }
      ],
      "payable": false,
      "stateMutability": "view",
      "type": "function"
   },
   {
      "constant": true,
      "inputs": [],
      "name": "symbol",
      "outputs": [
         {
            "internalType": "string",
            "name": "",
            "type": "string"
         }
      ],
      "payable": false,
      "stateMutability": "view",
      "type": "function"
   },
   {
      "constant": true,
      "inputs": [],
      "name": "totalSupply",
      "outputs": [
         {
            "internalType": "uint256",
            "name": "",
            "type": "uint256"
         }
      ],
      "payable": false,
      "stateMutability": "view",
      "type": "function"
   },
   {
      "constant": false,
      "inputs": [
         {
            "internalType": "address",
            "name": "receiver",
            "type": "address"
         },
         {
            "internalType": "uint256",
            "name": "numTokens",
            "type": "uint256"
         }
      ],
      "name": "transfer",
      "outputs": [
         {
            "internalType": "bool",
            "name": "",
            "type": "bool"
         }
      ],
      "payable": false,
      "stateMutability": "nonpayable",
      "type": "function"
   },
   {
      "constant": false,
      "inputs": [
         {
            "internalType": "address",
            "name": "owner",
            "type": "address"
         },
         {
            "internalType": "address",
            "name": "buyer",
            "type": "address"
         },
         {
            "internalType": "uint256",
            "name": "numTokens",
            "type": "uint256"
         }
      ],
      "name": "transferFrom",
      "outputs": [
         {
            "internalType": "bool",
            "name": "",
            "type": "bool"
         }
      ],
      "payable": false,
      "stateMutability": "nonpayable",
      "type": "function"
   }
];
const privKey =
   '4f3ea91012fc27131fdf2a62568725654726c04c46572c3eb00754b5455fe3e7'; // Genesis private key
const address = '0x93a88B7893FCDb130ab9209f63AB2e6854e617A1';
const web3 = new Web3('http://localhost:8540');
// Deploy contract
const deploy = async () => {
   console.log('Attempting to deploy from account:', address);
const erc20 = new web3.eth.Contract(abi);
const erc20Tx = erc20.deploy({
      data: bytecode,
      arguments: [50000],
   });
const createTransaction = await web3.eth.accounts.signTransaction(
      {
         from: address,
         data: erc20Tx.encodeABI(),
         gas: '429000',
      },
      privKey
   );
const createReceipt = await web3.eth.sendSignedTransaction(
      createTransaction.rawTransaction
   );
   console.log('Contract deployed at address', createReceipt.contractAddress);
};
deploy();