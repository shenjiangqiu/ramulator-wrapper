extern {
    pub(super) fn get_ramulator(config_name:*const libc::c_void) -> *mut libc::c_void;
    pub(super) fn delete_ramulator(ramulator: *mut libc::c_void);
    pub(super) fn ramulator_send(
        ramulator: *mut libc::c_void,
        addr: u64,
        is_write: libc::boolean_t,
    );
    pub(super) fn ramulator_get(ramulator: *const libc::c_void) -> u64;
    pub(super) fn ramulator_pop(ramulator: *mut libc::c_void) -> u64;

    pub(super) fn ramulator_cycle(ramulator: *mut libc::c_void);
    pub(super) fn ramulator_ret_available(ramulator: *mut libc::c_void) -> libc::boolean_t;
    pub(super) fn ramulator_available(ramulator: *mut libc::c_void, addr: u64, is_write: libc::boolean_t) -> libc::boolean_t;
}