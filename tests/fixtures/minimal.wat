(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func))
  (import "" "init_callback" (func $init_callback (type 0)))
  (import "" "run_callback" (func $run_callback (type 0)))
  (func $_init (type 1)
    i32.const 1048576
    i32.const 18
    call $init_callback)
  (func $_run (type 1)
    i32.const 1
    i32.const 0
    call $run_callback)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048594))
  (global (;2;) i32 (i32.const 1048594))
  (export "memory" (memory 0))
  (export "_init" (func $_init))
  (export "_run" (func $_run))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (data (;0;) (i32.const 1048576) "{\22name\22:\22minimal\22}"))
