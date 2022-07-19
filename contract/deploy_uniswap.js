const Web3 = require('web3');
const fs = require('fs');
const bytecode=fs.readFileSync('/home/srisht/project_repo/openethereum/contract/uniswap_sol_Token.bin').toString();
const abi=JSON.parse(fs.readFileSync('/home/srisht/project_repo/openethereum/contract/uniswap_sol_Token.abi').toString());

const privKey =
   '4f3ea91012fc27131fdf2a62568725654726c04c46572c3eb00754b5455fe3e7'; // Genesis private key
const address = '0x93a88B7893FCDb130ab9209f63AB2e6854e617A1';
const web3 = new Web3('http://localhost:8540');
// Deploy contract
const deploy = async () => {
   console.log('Attempting to deploy from account:', address);
const uniswap = new web3.eth.Contract(abi);
const uniswapTx = uniswap.deploy({
      data: bytecode,
   });
const createTransaction = await web3.eth.accounts.signTransaction(
      {
         from: address,
         data: uniswapTx.encodeABI(),
         gas: '4290000',
      },
      privKey
   );
const createReceipt = await web3.eth.sendSignedTransaction(
      createTransaction.rawTransaction
   );
   console.log('Contract deployed at address', createReceipt.contractAddress);
};
deploy();