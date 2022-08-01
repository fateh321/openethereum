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

const _amountADesired = 10000000;
const _amountBDesired = 20000000;
const _amountAMin = 1;
const _amountBMin = 1;
const _deadline = 1234567891234567;
const _to = address;
// Contract Tx
const router = new web3.eth.Contract(abi, contractAddress);
const encoded = router.methods.addLiquidity(tokenAddress1, tokenAddress2, _amountADesired, _amountBDesired, _amountAMin, _amountBMin, _to, _deadline).encodeABI();
// erc20.methods.transfer(receiver,_value).call();
// const encoded = erc20.methods.balanceOf(address).call();
// erc20.methods.balanceOf(address).call()
// erc20.methods
//   .transfer(receiver, "100")
//   .send({ from: address }, function (err, res) {
//     if (err) {
//       console.log("An error occured", err)
//       return
//     }
//     console.log("Hash of the transaction: " + res)
//   })

const routertx = async () => {
   console.log(
      `adding liquidity in contract at address ${contractAddress}`
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