use crate::bindings::foo;

#[no_mangle]
pub extern "C" fn use_foo(foo: *mut foo) {
    unsafe {
        println!("{}", (*foo).bar);
    }
}
