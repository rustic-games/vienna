const REGISTRATION: &str = r#"{"name":"minimal"}"#;

#[no_mangle]
pub extern "C" fn _init() {
    let slice = REGISTRATION.as_bytes();
    unsafe { ffi::init_callback(slice.as_ptr() as i32, slice.len() as i32) };
}

#[no_mangle]
pub extern "C" fn _run() {
    let mut slice: Box<[u8]> = vec![].into_boxed_slice();
    unsafe { ffi::run_callback(slice.as_mut_ptr() as i32, slice.len() as i32) };
}

pub mod ffi {
    #[link(wasm_import_module = "")]
    extern "C" {
        #[no_mangle]
        pub fn init_callback(ptr: i32, len: i32);
        #[no_mangle]
        pub fn run_callback(ptr: i32, len: i32);
    }
}
