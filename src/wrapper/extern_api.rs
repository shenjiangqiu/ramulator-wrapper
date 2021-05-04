#[link(name = "dramsim2_wrapper_c")]
extern "C" {
    pub(super) fn get_dramsim2() -> *mut libc::c_void;
    pub(super) fn delete_dramsim2(dramsim: *mut libc::c_void);
    pub(super) fn dramsim2_send(
        dramsim: *mut libc::c_void,
        addr: u64,
        is_write: libc::boolean_t,
    ) -> libc::boolean_t;
    pub(super) fn dramsim2_get(dramsim: *mut libc::c_void) -> u64;
    pub(super) fn dramsim2_tick(dramsim: *mut libc::c_void);
    pub(super) fn dramsim2_ret_available(dramsim: *mut libc::c_void) -> libc::boolean_t;
    pub(super) fn dramsim2_available(dramsim: *mut libc::c_void, addr: u64, is_write: libc::boolean_t) -> libc::boolean_t;
    pub(super) fn dramsim2_get_channel_id(dramsim: *mut libc::c_void, addr: u64) -> libc::c_int;
}