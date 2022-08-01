const csv = require('csv-parser');
// var csv = require('fast-csv');
const Web3 = require('web3');
const fs = require('fs');
const path = require("path");
const abi=JSON.parse(fs.readFileSync(path.resolve(__dirname, "contract_uniswap/router/router_sol_ERC20.abi")).toString());

const web3 = new Web3('http://localhost:8540');
const tokenAddress1 = '0x4FF947e19ab44afA198A3DdEaaeD817b4a8417FF';
const tokenAddress2 = '0xdDa66C80C54c37d65B960AC8dFd2F0fDD2449B38';


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
		
		const _value = 100000

		// Initialization
		const privKey = keys[i].Privkey; // Genesis private key
		const address = keys[i].PubKey;
		const from_address = '0x93a88B7893FCDb130ab9209f63AB2e6854e617A1';
		// Contract Tx
		const erc20_1 = new web3.eth.Contract(abi, tokenAddress2);
		const encoded = erc20_1.methods.transferFrom(from_address, address, _value).encodeABI();

		const erc20_1tx = async () => {
		   console.log(
		      `Calling the transfer to ${address}  in contract at address ${tokenAddress2}`
		   );
		   const createTransaction = await web3.eth.accounts.signTransaction(
		      {
		         from: address,
		         to: tokenAddress2,
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
		erc20_1tx();



	}


	}
)


