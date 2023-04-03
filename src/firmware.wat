(module
  (import "env" "pinMode" (func $pinMode (param i32 i32)))
  (import "env" "digitalWrite" (func $digitalWrite (param i32 i32)))
  (import "env" "delay" (func $delay (param i32)))
  (func
    (export "_start")
    (call $pinMode
      (i32.const 2)
      (i32.const 1))
    (loop $loop
      (call $digitalWrite
        (i32.const 2)
        (i32.const 0))
      (call $delay
        (i32.const 300))
      (call $digitalWrite
        (i32.const 2)
        (i32.const 1))
      (call $delay
        (i32.const 300))
      (br $loop))))
