const Web3 = require('web3');

// Variables definition
var ethers = require('ethers');  
var crypto = require('crypto');

var id0 = crypto.randomBytes(32).toString('hex');
var privKey0 = "0x"+id0;
var addressTo0 = "0x939aACc3965392dEF82783A0FC7Ffc76435bA5F4"

// new ethers.Wallet(privKey0);

var id1 = crypto.randomBytes(32).toString('hex');
var privKey1 = "0x"+id1;
var addressTo1 = new ethers.Wallet(privKey1);

const privKey =
 '262ca48c689471e77309bfd778628170f03b12c3c745670deff9d53b5e40066a'; // Genesis private key
const addressFrom = '0x00aa39d30f0d20ff03a22ccfc30b7efbfca597c2';
// const addressTo0 = '0x65e154ef9a2967e922936415bb0e2204be87b64c';
// const addressTo1 = '0x65e154ef9a2967e922936415bb0e2204be87b64b';
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
         from: addressTo0,
         to: "0x15DC7a72f526BD85AfdC5a977Dab25A6CD076330"
,
         value: web1.utils.toWei('10', 'ether'),
         gas: '21000',
      },
      '0x2d12d84ac35a5fdf4e39f1f77815595a4ce2babcc538e6e5e6fb3bc715d8bc2f'

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
deploy1();




