use std::ffi::CString;

mod extern_api;

use extern_api::*;

#[derive(Debug)]

/// # Description
/// A wrapper for ramulator
/// the only field is the pointer to ramulator
pub struct RamulatorWrapper {
    data: *mut libc::c_void,
}

impl RamulatorWrapper {
    /// create a new ramulator wrapper
    /// # Arguments
    /// stat_name: the file name to save the statistics
    pub fn new(config_name: &str, stat_name: &str) -> Self {
        unsafe {
            // get c str from config_name
            let config_name = CString::new(config_name).unwrap();
            let stats_name = CString::new(stat_name).unwrap();
            let ramulator = get_ramulator(
                config_name.as_ptr() as *const libc::c_void,
                stats_name.as_ptr() as *const libc::c_void,
            );

            RamulatorWrapper { data: ramulator }
        }
    }
}

impl Drop for RamulatorWrapper {
    fn drop(&mut self) {
        unsafe {
            let ramulator_ptr = self.data as *mut libc::c_void;

            delete_ramulator(ramulator_ptr);
        }
    }
}

impl RamulatorWrapper {
    /// send a request to ramulator
    pub fn send(&mut self, addr: u64, is_write: bool) {
        unsafe {
            let ramulator_ptr = self.data as *mut libc::c_void;
            ramulator_send(ramulator_ptr, addr, is_write);
        }
    }
    /// get the next returned from ramulator
    pub fn get(&self) -> u64 {
        unsafe {
            let ramulator_ptr = self.data as *const libc::c_void;
            ramulator_get(ramulator_ptr)
        }
    }
    /// pop the next returned from ramulator
    pub fn pop(&mut self) -> u64 {
        unsafe {
            let ramulator_ptr = self.data as *mut libc::c_void;
            ramulator_pop(ramulator_ptr)
        }
    }
    /// update the cycle of ramulator
    pub fn cycle(&mut self) {
        unsafe {
            let ramulator_ptr = self.data as *mut libc::c_void;
            ramulator_cycle(ramulator_ptr);
        }
    }
    /// check if the returned is available
    pub fn ret_available(&mut self) -> bool {
        unsafe {
            let ramulator_ptr = self.data as *mut libc::c_void;

            ramulator_ret_available(ramulator_ptr)
        }
    }
    /// check if the remulator is ready to receive a request
    pub fn available(&mut self, addr: u64, is_write: bool) -> bool {
        unsafe {
            let ramulator_ptr = self.data as *mut libc::c_void;

            ramulator_available(ramulator_ptr, addr, is_write)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn ramulator_wrapper_test() {
        use super::*;
        let mut ramulator = RamulatorWrapper::new("HBM-config.cfg", "test1.txt");
        ramulator.send(1, false);
        while !ramulator.ret_available() {
            ramulator.cycle();
        }
        let ret = ramulator.get();
        assert_eq!(ret, 1);
        assert!(ramulator.ret_available());
        ramulator.cycle();
        ramulator.pop();
        ramulator.cycle();

        assert!(!ramulator.ret_available());
        assert!(ramulator.available(0, false));
    }

    #[test]
    fn ramulator_wrapper_full_test() {
        use super::RamulatorWrapper;
        let mut ramulator = RamulatorWrapper::new("HBM-config.cfg", "test2.txt");
        let mut cycle = 0;
        let count = 10u64;
        let mut all_req: HashSet<_> = (1..count).into_iter().map(|i| i * 64).collect();
        for i in 1..count {
            while !ramulator.available(i * 64, false) {
                ramulator.cycle();
                cycle += 1;
            }
            ramulator.send(i * 64, false);
            ramulator.cycle();
        }
        for _i in 1..count {
            while !ramulator.ret_available() {
                ramulator.cycle();
                cycle += 1;
            }
            let result = ramulator.pop();
            ramulator.cycle();

            //assert!(all_req.contains(&result));
            println!("{}", result);
            all_req.remove(&result);
        }
        for _i in 0..1000 {
            ramulator.cycle();
        }
        assert!(!ramulator.ret_available());

        println!("cycle: {}", cycle);
    }
}
