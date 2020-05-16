#[macro_export]
macro_rules! load {
    () => {
        use $crate::prelude::*;

        #[no_mangle]
        /// Initialize the plugin by exposing its configuration to the engine.
        pub extern "C" fn _init() {
            // Explicit type to improve compiler error for plugin authors.
            let registration: Registration = init();

            $crate::init(registration);
        }

        #[no_mangle]
        /// Run the plugin on every game update.
        pub extern "C" fn _run(ptr: i32, len: i32) {
            let mut state = unsafe { State::from_raw(ptr as *mut u8, len as usize) };
            let result: Result<()> = run(&mut state);

            $crate::run(state, result);
        }

        #[no_mangle]
        /// Allocate memory on the guest.
        pub extern "C" fn _malloc(len: i32) -> i32 {
            $crate::malloc(len)
        }
    };
}
