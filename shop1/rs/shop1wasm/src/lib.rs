// Code generated by schema tool; DO NOT EDIT.

use shop1impl::*;
use wasmvmhost::*;

#[no_mangle]
fn on_call(index: i32) {
    WasmVmHost::connect();
    on_dispatch(index);
}

#[no_mangle]
fn on_load() {
    WasmVmHost::connect();
    on_dispatch(-1);
}
