use std::{ffi::{c_int, c_uint, CString}, ptr};

use libnode_rs;
use libnode_rs::sys;

#[no_mangle]
pub unsafe extern "C" fn register_napi_module(
  env: sys::napi::napi_env,
  exports: sys::napi::napi_value,
) -> sys::napi::napi_value {
  println!("Hello world");
  let mut raw_value = ptr::null_mut();
  sys::napi::napi_get_undefined(env, &mut raw_value);
  raw_value.cast()
}

pub fn main() {
  unsafe {

    let nm = Box::new(sys::napi::napi_module {
      nm_version: 8 as c_int,
      nm_flags: 0 as c_uint,
      nm_filename: CString::new("register_napi_module").unwrap().as_ptr(),
      nm_register_func: Some(register_napi_module),
      nm_modname: CString::new("register_napi_module").unwrap().as_ptr(),
      nm_priv: ptr::null_mut(),
      reserved: [ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut()],
    });

    let nm = Box::into_raw(nm);
    let status = sys::napi::napi_module_register(nm);
    println!("{:?}", status);
  };

  libnode_rs::eval_blocking(r"
    require('register_napi_module');
    console.log('[start] Process A');
    setTimeout(() => console.log('[end]   Process A'), 1000);
  ").unwrap();
}
