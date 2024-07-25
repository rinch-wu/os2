extern "C" {
    fn _num_app();
}
/// Get the total number of applications.
pub fn get_num_app() -> usize {
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

pub fn get_app_data(app_id: usize) -> &'static [u8] {
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_satrt = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    assert!(app_id < num_app);
    unsafe {
        core::slice::from_raw_parts(
            app_satrt[app_id] as *const u8,
            app_satrt[app_id + 1] - app_satrt[app_id],
        )
    }
}
