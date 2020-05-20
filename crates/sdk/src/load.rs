#[macro_export]
macro_rules! load {
    () => {
        use $crate::prelude::*;

        #[no_mangle]
        /// Initialize the plugin by exposing its configuration to the engine.
        pub extern "C" fn _init() {
            // Explicit type to improve compiler error for plugin authors.
            let registration: Registration = init();

            $crate::init(&registration);
        }

        #[no_mangle]
        /// Run the plugin on every game update.
        pub extern "C" fn _run(ptr: i32, len: i32) {
            // Get data transfered from host to guest.
            let transfer = unsafe { StateTransfer::from_raw(ptr as *mut u8, len as usize) };

            // Destructure into SDK, state and events.
            let $crate::Data {
                sdk,
                mut state,
                events,
            } = transfer.into();

            // Explicit type to improve compiler error for plugin authors.
            let result: Result<()> = run(&sdk, &mut state, &events);

            $crate::run(state, result);
        }

        #[no_mangle]
        /// Allocate memory on the guest.
        pub extern "C" fn _malloc(len: i32) -> i32 {
            $crate::malloc(len)
        }
    };
}
