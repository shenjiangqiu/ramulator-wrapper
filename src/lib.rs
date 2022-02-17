//! # a wrapper for ramulator
//! - the crate ramulator_wrapper is a wrapper of famouse dram simulator:ramulator.original url:https://github.com/CMU-SAFARI/ramulator
//! - in order to better use the ramulator, I use another modified version of ramulator:https://github.com/shenjiangqiu/ramulator
//! - the modified version of ramulator support multithread, and fix some bugs.
//! # Example
//! ```
//! use ramulator_wrapper::RamulatorWrapper;
//! let mut ramulator = RamulatorWrapper::new("ramulator/config/DDR3_micron_32M_8B_x8_sg125_tCK_C9.ini");
//! ```
//! let mut ramulator = RamulatorWrapper::new("test2.txt");
//! let mut cycle = 0;
//! let count = 10u64;
//! let mut all_req: HashSet<_> = (1..count).into_iter().map(|i| i * 64).collect();
//! for i in 1..count {
//!     while !ramulator.available(i * 64, false) {
//!         ramulator.cycle();
//!         cycle += 1;
//!     }
//!     ramulator.send(i * 64, false);
//!     ramulator.cycle();
//! }
//! for _i in 1..count {
//!     while !ramulator.ret_available() {
//!         ramulator.cycle();
//!         cycle += 1;
//!     }
//!     let result = ramulator.pop();
//!     ramulator.cycle();
//!
//!     //assert!(all_req.contains(&result));
//!     println!("{}", result);
//!     all_req.remove(&result);
//! }
//! for _i in 0..1000 {
//!     ramulator.cycle();
//! }
//! assert_eq!(ramulator.ret_available(), false);
//!
//! println!("cycle: {}", cycle);
//! ```
mod wrapper;
pub use wrapper::RamulatorWrapper;
