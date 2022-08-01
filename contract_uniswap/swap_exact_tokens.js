const Web3 = require('web3');
const fs = require('fs');
const path = require("path");
const abi=JSON.parse(fs.readFileSync(path.resolve(__dirname, "router/router_sol_UniswapV2Router02.abi")).toString());

// Initialization
const privKey =
   '4f3ea91012fc27131fdf2a62568725654726c04c46572c3eb00754b5455fe3e7'; // Genesis private key
const address = '0x93a88B7893FCDb130ab9209f63AB2e6854e617A1';
const web3 = new Web3('http://localhost:8540');
const tokenAddress1 = '0x4FF947e19ab44afA198A3DdEaaeD817b4a8417FF';
const tokenAddress2 = '0xdDa66C80C54c37d65B960AC8dFd2F0fDD2449B38';

const contractAddress = '0x5bc532C8910EA2934a92A22d5dF3c868C91C9631';

const _amountIn = 10;
const _amountOutMin = 0;
const _path = [tokenAddress1, tokenAddress2];
const _to = '0x65e154ef9a2967e922936415bb0e2204be87b64c';
const _deadline = 1234567891234567;

// Contract Tx
const router = new web3.eth.Contract(abi, contractAddress);
const encoded = router.methods.swapExactTokensForTokens(_amountIn, _amountOutMin, _path, _to, _deadline).encodeABI();

const routertx = async () => {
   console.log(
      `swapping ${_amountIn} tokens in contract at address ${contractAddress}`
   );
   const createTransaction = await web3.eth.accounts.signTransaction(
      {
         from: address,
         to: contractAddress,
         data: encoded,
         gas: '429496',
      },
      privKey
   );
const createReceipt = await web3.eth.sendSignedTransaction(
      createTransaction.rawTransaction
   );
   console.log(`Tx successfull with hash: ${createReceipt.transactionHash}`);
};
routertx();