use std::ffi::CString;
use std::ptr;

use libnode_rs;
use libnode_rs::sys;

pub fn main() -> std::io::Result<()> {
  // Register a napi module and inject it into the JavaScript runtime
  libnode_rs::napi_module_register("my_native_extension", |env, exports| unsafe {
    // Create number
    let mut raw_value = ptr::null_mut();
    sys::napi::napi_create_uint32(env, 42, &mut raw_value);

    // Set number on exports object
    sys::napi::napi_set_named_property(
      env,
      exports.cast(),
      CString::new("hello").unwrap().as_ptr(),
      raw_value,
    );

    // Return exports object
    exports
  });

  // Execute JavaScript and access the native extensions via process._linkedBinding
  libnode_rs::eval_blocking(r#"
    console.log('Hello World')
    console.log(process._linkedBinding("my_native_extension"))
  "#)?;

  // Currently can't spawn Nodejs more than once
  // libnode_rs::eval_blocking(r#"
  //   console.log('Hello World')
  //   //console.log(process._linkedBinding("alsh:foo"))
  // "#)?;
  Ok(())
}
