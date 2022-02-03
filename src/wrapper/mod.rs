use libc;

mod extern_api;

use extern_api::*;

#[derive(Debug)]
pub struct RamulatorWrapper {
    data: u64,
}

impl RamulatorWrapper {
    pub fn new() -> Self {
        unsafe {
            let ramulator = get_ramulator();

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
    pub fn send(&mut self, addr: u64, is_write: bool)  {
        unsafe {
            let ramulator_ptr = self.data as *mut libc::c_void;
            ramulator_send(ramulator_ptr, addr, is_write as libc::boolean_t);
            
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

            match ramulator_ret_available(ramulator_ptr) {
                0 => false,
                _ => true,
            }
        }
    }
    pub fn available(&mut self, addr: u64, is_write: bool) -> bool {
        unsafe {
            let ramulator_ptr = self.data as *mut libc::c_void;

            match ramulator_available(ramulator_ptr, addr, is_write as libc::boolean_t) {
                0 => false,
                _ => true,
            }
        }
    }
    
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    

    #[test]
    fn ramulator_wrapper_test() {
        use super::*;
        let mut ramulator = RamulatorWrapper::new();
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
        let mut ramulator = RamulatorWrapper::new();
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
        for _i in 0..1000{
            ramulator.cycle();
        }
        assert_eq!(ramulator.ret_available(), false);

        println!("cycle: {}", cycle);
    }
}

