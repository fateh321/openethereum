const createCsvWriter = require('csv-writer').createObjectCsvWriter;
const fastcsv = require('fast-csv');
const fs = require('graceful-fs');
const ws = fs.createWriteStream("balance.csv");

var ethers = require('ethers');  
var crypto = require('crypto');

const pubPrivKeys = createCsvWriter({
  path: 'out.csv',
  header: [
    {id: 'privKey', title: 'Privkey'},
    {id: 'pubKey', title: 'PubKey'},
  ]
});

// "0x004ec07d2329997267Ec62b4166639513386F32E": { "balance": "10000000000000000000000" },
// "0x0351EFB27D108959614b9D5c710e0cEAD7060C1D: { balance: 10000000000000000000000 },"
const bal = createCsvWriter({
  path: 'balance.csv',
  header: [
    {id: 'pubKey', title: 'PubKey'},
  ]
});

const data = [];
const balance = [];

let num = 500;
//clearing file
fs.writeFile('test.txt','', err => {
if (err) {
  console.error(err);
}
// file written successfully
});
for (let i = 0; i < num; i++) {
var id = crypto.randomBytes(32).toString('hex');
var privateKey = "0x"+id;
var wallet = new ethers.Wallet(privateKey);
  data[i] = {
    privKey: privateKey,
    pubKey: wallet.address
  } ;
  let a = "\"";
  let b = "\": { \"balance\" : \"10000000000000000000000\" },\n";
  let res = a+wallet.address+b;
  balance[i] =  {
    pubKey: res,
  };
  fs.appendFile('test.txt', res, err => {
  if (err) {
    console.error(err);
  }
  // file written successfully
});
}

setTimeout(function(){
pubPrivKeys
  .writeRecords(data)
  .then(()=> console.log('The CSV file was written successfully'));

}, 500); 


// bal
//   .writeRecords(balance)
//   .then(()=> console.log('The CSV file was written successfully'));
