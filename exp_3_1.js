const csv = require('csv-parser');
// var csv = require('fast-csv');
const Web3 = require('web3');
const fs = require('fs');
const path = require("path");
const abi=JSON.parse(fs.readFileSync(path.resolve(__dirname, "contract_uniswap/router/router_sol_UniswapV2Router02.abi")).toString());

const web3 = new Web3('http://localhost:8540');
const tokenAddress1 = '0x4FF947e19ab44afA198A3DdEaaeD817b4a8417FF';
const tokenAddress2 = '0xdDa66C80C54c37d65B960AC8dFd2F0fDD2449B38';

const contractAddress = '0x5bc532C8910EA2934a92A22d5dF3c868C91C9631';

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
		const _amountIn = i+1;
		const _amountOutMin = 0;
		var _path; 
		if (i%2 == 0){
			_path = [tokenAddress2, tokenAddress1];
		}else{
			_path = [tokenAddress1, tokenAddress2]
		};
		
		const _to = '0x65e154ef9a2967e922936415bb0e2204be87b64c';
		const _deadline = 1234567891234567;
		// Initialization
		const privKey = keys[i].Privkey; // Genesis private key
		const address = keys[i].PubKey;
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



	}

	}
)


