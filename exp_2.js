const csv = require('csv-parser');
// var csv = require('fast-csv');
const Web3 = require('web3');
const fs = require('fs');
const abi = JSON.parse(fs.readFileSync('/home/srisht/project_repo/openethereum/contract_easy/uniswap_sol_uniswap.abi').toString());

const web3 = new Web3('http://localhost:8540');
const erc20Address1 = '0x4FF947e19ab44afA198A3DdEaaeD817b4a8417FF';
const erc20Address2 = '0xdDa66C80C54c37d65B960AC8dFd2F0fDD2449B38';
const contractAddress = '0x99D35b17cDF0E1de571F985d0EF4089C3C4d4e39';
const receiver = '0x65e154ef9a2967e922936415bb0e2204be87b64c';
const _value = 1;


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
	for (let i = 0; i < 200; i++) {
		const _value = 1+i;
		// Initialization
		const privKey = keys[i].Privkey; // Genesis private key
		const address = keys[i].PubKey;
		// Contract Tx
		const uniswap = new web3.eth.Contract(abi, contractAddress);
		const encoded = uniswap.methods.dtransfer(erc20Address1, erc20Address2, receiver,_value).encodeABI();

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



	}

	}
)


