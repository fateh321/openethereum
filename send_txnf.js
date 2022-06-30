const Web3 = require('web3');

// Variables definition
const privKey =
 '4f3ea91012fc27131fdf2a62568725654726c04c46572c3eb00754b5455fe3e7'; // Genesis private key
const addressFrom = '0x93a88B7893FCDb130ab9209f63AB2e6854e617A1';
const addressTo0 = '0xeA404440859F0503f8959759766eF56A494A6f8C';

// const addressTo0 = '0x65e154ef9a2967e922936415bb0e2204be87b64c';
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



