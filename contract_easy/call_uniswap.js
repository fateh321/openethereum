const Web3 = require('web3');
const fs = require('fs');
const abi = JSON.parse(fs.readFileSync('/home/srisht/project_repo/openethereum/contract_easy/uniswap_sol_uniswap.abi').toString());

// Initialization
const privKey =
   '4f3ea91012fc27131fdf2a62568725654726c04c46572c3eb00754b5455fe3e7'; // Genesis private key
const address = '0x93a88B7893FCDb130ab9209f63AB2e6854e617A1';
const web3 = new Web3('http://localhost:8540');
const erc20Address = '0x4FF947e19ab44afA198A3DdEaaeD817b4a8417FF';
const contractAddress = '0xdDa66C80C54c37d65B960AC8dFd2F0fDD2449B38';
const receiver = '0x65e154ef9a2967e922936415bb0e2204be87b64c';
const _value = 8;
// Contract Tx
const uniswap = new web3.eth.Contract(abi, contractAddress);
const encoded = uniswap.methods.dtransfer(erc20Address,receiver,_value).encodeABI();
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

const uniswaptx = async () => {
   console.log(
      `Calling the transfer to ${receiver}  in contract at address ${contractAddress}`
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
uniswaptx();