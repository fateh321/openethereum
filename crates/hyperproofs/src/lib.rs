

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// pub fn init() {

// unsafe { initVc() };

// }
extern crate libloading as lib;
use std::thread;
use std::ffi::{CString, CStr};
use std::sync::mpsc;
use std::time::Duration;
use ethereum_types::{Address, BigEndianHash, H160, H256, U256};
use std::str::FromStr;
use keccak_hash::keccak;

static mut SHARD: u64 = 0u64;
static mut LASTCOMMITROUND: u64 = 999u64;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AggProof{
    pub proof: String,
    pub ready: bool,
    pub address: Vec<Address>,
    pub balance: Vec<U256>,
}
impl AggProof{
    pub fn new() -> Self {
        AggProof{
            proof: String::new(),
            ready:false,
            address: Vec::new(),
            balance: Vec::new(),
        }
    }
    pub fn concat_hash(x: H160, y: H256) -> H160{
        let l = keccak([x.as_bytes(), y.as_bytes()].concat());
        H160::from(l)
    }
    pub fn create_proof(&mut self) -> (){
        if self.address.len() == 0{
            return
        }
        self.ready = false;
        for i in 0..self.address.len(){
            pushAddressCommit(self.address[i].to_low_u64_be().rem_euclid(2u64.pow(16)),0u64);
        }
        match agg(0u64) {
           Ok(T) => {
               self.proof = T.0;
               self.ready = true;
           },
            _ => {},
        }

    }
    pub fn set_author_shard(address: Address) -> u64 {
        let _s1 = Address::from_str("00bd138abd70e2f00903268f3db08f2d25677c9e").unwrap();
        let _s2 = Address::from_str("00aa39d30f0d20ff03a22ccfc30b7efbfca597c2").unwrap();
        match address {
            x if x==_s1  => unsafe{SHARD = 0u64;
            0u64},
            x if x==_s2 => unsafe{SHARD = 1u64;
                1u64},
            _ => unsafe{SHARD = 999u64; println!("panic!, shard not recognised");
                999u64},
        }
    }
    pub fn get_shard() -> u64 {
        unsafe {
            let o = SHARD;
        o }
    }

    pub fn set_last_commit_shard(round: u64){
        unsafe{LASTCOMMITROUND = round; }
    }

    pub fn get_last_commit_round() -> u64 {
        unsafe {
            let o = LASTCOMMITROUND;
            o }
    }

    pub fn shard_count() -> u64 {
        2u64
    }
    pub fn author_shard(address: Address) -> u64 {
        let _s1 = Address::from_str("00bd138abd70e2f00903268f3db08f2d25677c9e").unwrap();
        let _s2 = Address::from_str("00aa39d30f0d20ff03a22ccfc30b7efbfca597c2").unwrap();
        match address {
           x if x==_s1  => 0u64,
            x if x==_s2 => 1u64,
            _ => 999u64,
        }

    }
    pub fn init(round:u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(r:u64) -> i64 > = lib.get(b"initVc")?;
            Ok(func(round))
        }
    }
    pub fn agg(nativeShard: u64) -> lib::Result<(String,bool)>{
        let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(s: u64) -> aggVc_return > = lib.get(b"aggVc")?;
            match func(nativeShard) {
                output => match output.r1 {
                    1u8 => Ok((CStr::from_ptr(output.r0)
                                   .to_string_lossy()
                                   .into_owned(), true)
                    ),
                    _ => Ok((CStr::from_ptr(output.r0)
                                 .to_string_lossy()
                                 .into_owned(), false)
                    ),
                }
            }
        }
    }

    pub fn pushAddressDelta(address: u64, delta: String, shard: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
        let c_delta = CString::new(delta)?;
        let go_str_delta = GoString {
            p: c_delta.as_ptr(),
            n: c_delta.as_bytes().len() as isize,
        };
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(a: u64, d: GoString, s: u64) -> i64 > = lib.get(b"pushAddressDeltaVc")?;
            Ok(func(address,go_str_delta,shard))
        }
    }
    pub fn resetAddressDelta(shard: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(s: u64) -> i64 > = lib.get(b"resetAddressDeltaVc")?;
            Ok(func(shard))
        }
    }
    //push address for which proof needs to be aggregated
    pub fn pushAddressCommit(address: u64, shard: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(a: u64, s: u64) -> i64 > = lib.get(b"pushAddressCommitVc")?;
            Ok(func(address,shard))
        }
    }
    //push address for which proof needs to be aggregated
    pub fn resetAddressCommit(shard: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(s: u64) -> i64 > = lib.get(b"resetAddressCommitVc")?;
            Ok(func(shard))
        }
    }

    pub fn pushAddressBalanceVerify(address: u64, bal: String, shard: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
        let c_bal = CString::new(bal)?;
        let go_str_bal = GoString {
            p: c_bal.as_ptr(),
            n: c_bal.as_bytes().len() as isize,
        };
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(a: u64, b: GoString, s: u64) -> i64 > = lib.get(b"pushAddressBalanceVerifyVc")?;
            Ok(func(address,go_str_bal,shard))
        }
    }
    pub fn resetAddressBalanceVerify(shard: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(s: u64) -> i64 > = lib.get(b"resetAddressBalanceVerifyVc")?;
            Ok(func(shard))
        }
    }
    pub fn verifyProof(input: String, shard:u64,round:u64) -> lib::Result<bool>{
        let c_input = CString::new(input)?;
        let go_str_input = GoString {
            p: c_input.as_ptr(),
            n: c_input.as_bytes().len() as isize,
        };
        let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(i: GoString, s:u64, r:u64) -> u8> = lib.get(b"verifyProofVc")?;
            match func(go_str_input, shard, round) {
                1u8 => Ok(true),
                _ => Ok(false)
            }
        }
    }
    pub fn commit(nativeShard: u64, round: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(n: u64, r: u64) -> i64 > = lib.get(b"commitVc")?;
            Ok(func(nativeShard,round))
        }
    }
    pub fn updateTree(nativeShard: u64) -> lib::Result<i64>{
        let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn(n: u64) -> i64 > = lib.get(b"updateShardProofTreeVc")?;
            Ok(func(nativeShard))
        }
    }
    pub fn resetPrevCommit() -> lib::Result<i64>{
        let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
        unsafe {
            let func: lib::Symbol<unsafe extern "C" fn() -> i64 > = lib.get(b"prevDigestResetVc")?;
            Ok(func())
        }
    }
}
pub fn init(round:u64) -> lib::Result<i64>{
    let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern "C" fn(r:u64) -> i64 > = lib.get(b"initVc")?;
        Ok(func(round))
    }
}

pub fn pushAddressDelta(address: u64, delta: String, shard: u64) -> lib::Result<i64>{
    let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
    let c_delta = CString::new(delta)?;
    let go_str_delta = GoString {
      p: c_delta.as_ptr(),
      n: c_delta.as_bytes().len() as isize,
    };
    unsafe {
        let func: lib::Symbol<unsafe extern "C" fn(a: u64, d: GoString, s: u64) -> i64 > = lib.get(b"pushAddressDeltaVc")?;
        Ok(func(address,go_str_delta,shard))
    }
}
//push address for which proof needs to be aggregated
pub fn pushAddressCommit(address: u64, shard: u64) -> lib::Result<i64>{
    let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern "C" fn(a: u64, s: u64) -> i64 > = lib.get(b"pushAddressCommitVc")?;
        Ok(func(address,shard))
    }
}

pub fn pushAddressBalanceVerify(address: u64, bal: String, shard: u64) -> lib::Result<i64>{
    let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
    let c_bal = CString::new(bal)?;
    let go_str_bal = GoString {
      p: c_bal.as_ptr(),
      n: c_bal.as_bytes().len() as isize,
    };
    unsafe {
        let func: lib::Symbol<unsafe extern "C" fn(a: u64, b: GoString, s: u64) -> i64 > = lib.get(b"pushAddressBalanceVerifyVc")?;
        Ok(func(address,go_str_bal,shard))
    }
}

pub fn agg(nativeShard: u64) -> lib::Result<(String,bool)>{
    let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern "C" fn(s: u64) -> aggVc_return > = lib.get(b"aggVc")?;
        match func(nativeShard) {
            output => match output.r1 {
                1u8 => Ok((CStr::from_ptr(output.r0)
                        .to_string_lossy()
                        .into_owned(), true)
                        ),
                _ => Ok((CStr::from_ptr(output.r0)
                        .to_string_lossy()
                        .into_owned(), false)
                        ),
            } 
        }
            }
}

pub fn commit(nativeShard: u64, round: u64) -> lib::Result<i64>{
    let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern "C" fn(n: u64, r: u64) -> i64 > = lib.get(b"commitVc")?;
        Ok(func(nativeShard,round))
    }
}

pub fn verifyProof(input: String, shard:u64,round:u64) -> lib::Result<bool>{
    let c_input = CString::new(input)?;
    let go_str_input = GoString {
      p: c_input.as_ptr(),
      n: c_input.as_bytes().len() as isize,
    };
    let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
    unsafe {        
        let func: lib::Symbol<unsafe extern "C" fn(i: GoString, s:u64, r:u64) -> u8> = lib.get(b"verifyProofVc")?;
        match func(go_str_input, shard, round) {
            1u8 => Ok(true),
            _ => Ok(false)
        } 
    }
}

pub fn demoProofShard() -> lib::Result<String>{
    let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
    unsafe {        
        let func: lib::Symbol<unsafe extern "C" fn() -> *mut ::std::os::raw::c_char> = lib.get(b"demoProof")?;
        Ok(CStr::from_ptr(func()).to_string_lossy().into_owned())
        // match func() {
            // _ => return Ok("as".to_string()),
            // GoString{p:_,n:_} => return Ok("as".to_string()), 
            //return Ok(CStr::from_ptr(p).to_string_lossy().into_owned()),
        // }
    }
}

pub fn demoVerifyShard(input: String) -> lib::Result<u8>{
    let c_input = CString::new(input)?;
    let go_str_input = GoString {
      p: c_input.as_ptr(),
      n: c_input.as_bytes().len() as isize,
    };
    let lib = lib::Library::new("/home/srisht/libhyper/hyperproofs-go/libshard.so")?;
    unsafe {        
        let func: lib::Symbol<unsafe extern "C" fn(i: GoString) -> u8> = lib.get(b"demoVerify")?;
        Ok(func(go_str_input))
    }
}
// pub fn update(){
//     unsafe { BenchmarkVCSCommit() };
// }

fn main() {
    // let (tx, rx) = mpsc::channel();
    // let proof = demoProofShard();
    // // let mut proof2;
    // let handle = thread::spawn(move|| {
    //     let proof2 = demoProofShard();
    //     tx.send(proof2).unwrap();
    //     // match proof2 {
    //     //     Ok(p) => println!("{:?}",demoVerifyShard(p)),
    //     //     _ => println!("error fuck"),
    //     // };

    // });
    // thread::sleep(Duration::from_secs(10));
    // match proof {
    //     Ok(p) => println!("proof is {:?}",demoVerifyShard(p)),
    //     _ => println!("error fuck"),
    // };

    // let received = rx.recv().unwrap();
       
    // match received {
    //     Ok(p) => println!("proof2 is {:?}",demoVerifyShard(p)),
    //     _ => println!("error fuck"),
    // };

    // // handle.join().unwrap();
    // // match proof2 {
    // //     Ok(p) => println!("{:?}",demoVerifyShard(p)),
    // //     _ => println!("error fuck"),
    // // };    
}
