const Web3 = require('web3');

// Variables definition
const privKey =
 '262ca48c689471e77309bfd778628170f03b12c3c745670deff9d53b5e40066a'; // Genesis private key
const addressFrom = '0x00aa39d30f0d20ff03a22ccfc30b7efbfca597c2';
const addressTo0 = '0x65e154ef9a2967e922936415bb0e2204be87b64c';
const addressTo1 = '0x65e154ef9a2967e922936415bb0e2204be87b64b';
const web0 = new Web3('http://localhost:8540');
const web1 = new Web3('http://localhost:8540');

// Create transaction
const deploy0 = async () => {
   console.log(
      `Attempting to make transaction from ${addressFrom} to ${addressTo0}`
   );

   const createTransaction = await web0.eth.accounts.signTransaction(
      {
         from: addressFrom,
         to: addressTo0,
         value: web0.utils.toWei('100', 'ether'),
         gas: '21000',
      },
      privKey
   );

   // Deploy transaction
   const createReceipt = await web0.eth.sendSignedTransaction(
      createTransaction.rawTransaction
   );
   // console.log(
   //    `Transaction successful with hash: ${createReceipt.transactionHash}`
   // );
};

const deploy1 = async () => {
   console.log(
      `Attempting to make transaction from ${addressFrom} to ${addressTo1}`
   );

   const createTransaction = await web1.eth.accounts.signTransaction(
      {
         from: addressFrom,
         to: addressTo1,
         value: web1.utils.toWei('100', 'ether'),
         gas: '21000',
      },
      privKey
   );

   // Deploy transaction
   const createReceipt = await web1.eth.sendSignedTransaction(
      createTransaction.rawTransaction
   );
   // console.log(
   //    `Transaction successful with hash: ${createReceipt.transactionHash}`
   // );
};

deploy0();
// deploy1();



