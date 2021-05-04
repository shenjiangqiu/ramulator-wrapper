use libc;

mod extern_api;

use extern_api::*;


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn get_channel_id_test() {
        use super::Dramsim2Wrapper;
        let mut dramsim = Dramsim2Wrapper::new();
        println!("channel_id: {}", dramsim.get_channel_id(64));
    }

    #[test]
    fn dramsim2_wrapper_test() {
        use super::*;
        let mut dramsim = Dramsim2Wrapper::new();
        dramsim.send(1, false);
        while !dramsim.ret_available() {
            dramsim.tick();
        }
        let ret = dramsim.get();
        assert_eq!(ret, 1);
        dramsim.tick();
        assert_eq!(dramsim.ret_available(), false);
        assert_eq!(dramsim.available(0, false), true);
    }

    #[test]
    fn dramsim2_wrapper_full_test() {
        use super::Dramsim2Wrapper;
        let mut dramsim = Dramsim2Wrapper::new();
        let mut cycle = 0;
        let count = 10u64;
        let mut all_req: HashSet<_> = (1..count).into_iter().map(|i| i * 64).collect();
        for i in 1..count {
            while !dramsim.available(i * 64, false) {
                dramsim.tick();
                cycle += 1;
            }
            let result = dramsim.send(i * 64, false);
            dramsim.tick();

            assert_eq!(result, true);
        }
        for _i in 1..count {
            while !dramsim.ret_available() {
                dramsim.tick();
                cycle += 1;
            }
            let result = dramsim.get();
            dramsim.tick();

            //assert!(all_req.contains(&result));
            println!("{}", result);
            all_req.remove(&result);
        }
        println!("cycle: {}", cycle);
    }
}


#[derive(Debug)]
pub struct Dramsim2Wrapper<'a> {
    data: &'a mut libc::c_void,
}

impl<'a> Dramsim2Wrapper<'a> {
    pub fn new() -> Self {
        unsafe {
            let dramsim2 = get_dramsim2();

            Dramsim2Wrapper {
                data: &mut *dramsim2,
            }
        }
    }
}

impl<'a> Drop for Dramsim2Wrapper<'a> {
    fn drop(&mut self) {
        unsafe {
            let dramsim_ptr = self.data as *mut libc::c_void;

            delete_dramsim2(dramsim_ptr);
        }
    }
}

impl<'a> Dramsim2Wrapper<'a> {
    pub fn send(&mut self, addr: u64, is_write: bool) -> bool {
        unsafe {
            let dramsim_ptr = self.data as *mut libc::c_void;
            let ret = dramsim2_send(dramsim_ptr, addr, is_write as libc::boolean_t);
            match ret {
                0 => false,
                _ => true,
            }
        }
    }
    pub fn get(&mut self) -> u64 {
        unsafe {
            let dramsim_ptr = self.data as *mut libc::c_void;
            dramsim2_get(dramsim_ptr)
        }
    }
    pub fn tick(&mut self) {
        unsafe {
            let dramsim_ptr = self.data as *mut libc::c_void;
            dramsim2_tick(dramsim_ptr);
        }
    }
    pub fn ret_available(&mut self) -> bool {
        unsafe {
            let dramsim_ptr = self.data as *mut libc::c_void;

            match dramsim2_ret_available(dramsim_ptr) {
                0 => false,
                _ => true,
            }
        }
    }
    pub fn available(&mut self, addr: u64, is_write: bool) -> bool {
        unsafe {
            let dramsim_ptr = self.data as *mut libc::c_void;

            match dramsim2_available(dramsim_ptr, addr, is_write as libc::boolean_t) {
                0 => false,
                _ => true,
            }
        }
    }
    pub fn get_channel_id(&mut self, addr: u64) -> i32 {
        unsafe {
            let dramsim_ptr = self.data as *mut libc::c_void;

            dramsim2_get_channel_id(dramsim_ptr, addr)
        }
    }
}
