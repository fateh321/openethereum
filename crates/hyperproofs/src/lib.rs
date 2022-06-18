

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
