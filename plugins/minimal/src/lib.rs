const REGISTRATION: &str = r#"{"name":"minimal","write":{},"read":{}}"#;
const RUN_RESULT: &str = r#"{"error":null,"state":{"owned":{"data":{}},"borrowed":{}}}"#;

#[no_mangle]
pub extern "C" fn _init() {
    let slice = REGISTRATION.as_bytes();
    unsafe { ffi::init_callback(slice.as_ptr() as i32, slice.len() as i32) };
}

#[no_mangle]
pub extern "C" fn _run(_ptr: i32, _len: i32) {
    let slice = RUN_RESULT.as_bytes();
    unsafe { ffi::run_callback(slice.as_ptr() as i32, slice.len() as i32) };
}

#[no_mangle]
pub extern "C" fn _malloc(len: i32) -> i32 {
    let vec = Vec::<u8>::with_capacity(len as usize);
    core::mem::ManuallyDrop::new(vec).as_mut_ptr() as i32
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
