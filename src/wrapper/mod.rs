use std::ffi::CString;

use libc;

mod extern_api;

use extern_api::*;

#[derive(Debug)]
pub struct RamulatorWrapper {
    data: u64,
}

impl RamulatorWrapper {
    pub fn new(config_name: &str) -> Self {
        unsafe {
            // get c str from config_name
            let c_str = CString::new(config_name).unwrap();
            let ramulator = get_ramulator(c_str.as_ptr() as *const libc::c_void);

            RamulatorWrapper {
                data: ramulator as u64,
            }
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
    pub fn send(&mut self, addr: u64, is_write: bool) {
        unsafe {
            let ramulator_ptr = self.data as *mut libc::c_void;
            ramulator_send(ramulator_ptr, addr, is_write);
        }
    }
    pub fn get(&self) -> u64 {
        unsafe {
            let ramulator_ptr = self.data as *const libc::c_void;
            ramulator_get(ramulator_ptr)
        }
    }
    pub fn pop(&mut self) -> u64 {
        unsafe {
            let ramulator_ptr = self.data as *mut libc::c_void;
            ramulator_pop(ramulator_ptr)
        }
    }
    pub fn cycle(&mut self) {
        unsafe {
            let ramulator_ptr = self.data as *mut libc::c_void;
            ramulator_cycle(ramulator_ptr);
        }
    }
    pub fn ret_available(&mut self) -> bool {
        unsafe {
            let ramulator_ptr = self.data as *mut libc::c_void;

            ramulator_ret_available(ramulator_ptr)
        }
    }
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
        let mut ramulator = RamulatorWrapper::new("test1.txt");
        ramulator.send(1, false);
        while !ramulator.ret_available() {
            ramulator.cycle();
        }
        let ret = ramulator.get();
        assert_eq!(ret, 1);
        assert_eq!(ramulator.ret_available(), true);
        ramulator.cycle();
        ramulator.pop();
        ramulator.cycle();

        assert_eq!(ramulator.ret_available(), false);
        assert_eq!(ramulator.available(0, false), true);
    }

    #[test]
    fn ramulator_wrapper_full_test() {
        use super::RamulatorWrapper;
        let mut ramulator = RamulatorWrapper::new("test2.txt");
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
        assert_eq!(ramulator.ret_available(), false);

        println!("cycle: {}", cycle);
    }
}
