(module
  (type (;0;) (func (param i32 i32)))

  (import "" "init_callback" (func $init_callback (type 0)))
  (import "" "run_callback" (func $run_callback (type 0)))

  (func (export "_init")
    i32.const 1048576
    i32.const 36
    call $init_callback)
  (func (export "_run") (type 0)
    i32.const 1
    i32.const 0
    call $run_callback)
  (func (export "_malloc") (param i32) (result i32)
    i32.const 0)

  (data (;0;) (i32.const 1048576) "{\22name\22:\22minimal\22,\22write\22:{},\22read\22:{}}")
  (memory (;0;) 17)
  (export "memory" (memory 0)))
