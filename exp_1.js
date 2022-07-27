const csv = require('csv-parser');
// var csv = require('fast-csv');
const fs = require('fs');
const Web3 = require('web3');
const web1 = new Web3('http://localhost:8540');



var queryParameter = ()=> new Promise( resolve =>{
	var keys = [];
	fs.createReadStream('out.csv')
	  .pipe(csv())
	  .on('data', row => {
	    keys.push(row);
	    // console.log(row);
	  })
	  .on('end',()=>{
          resolve(keys)
      })
})
var keys = [];
queryParameter().then((res)=>
	{keys = res;
	console.log("fuck you");
	let len = keys.length;
	console.log(len);
	for (let i = 0; i < 500; i++) {
		addressFrom = keys[i].PubKey;
		addressTo1 = '0x65e154ef9a2967e922936415bb0e2204be87b64b';
		privKey = keys[i].Privkey;
		const deploy = async () => {
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
	deploy();
	// console.log("hi");
	}

	}
)




