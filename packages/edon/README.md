# Embed Nodejs within your Rust 🚀

Embed the fully featured Nodejs runtime into your Rust application! 

Features:
- Native Nodejs extensions via standard napi bindings
- Support for worker threads
- TODO ~Nodejs statically linked to allow creation of single binary outputs~ (help wanted)

# Usage

## Simple Example

```rust
use edon;

pub fn main() -> std::io::Result<()> {
  // Execute JavaScript and TypeScript with
  libnode_rs::eval_blocking(r#"
    const message: string = "Hello World TypeScript"
    console.log(message)
    console.log(process._linkedBinding("my_native_extension"))
  "#)?;

  Ok(())
}
```

## Native Extensions

```rust
pub fn main() -> std::io::Result<()> {
  // Inject a native extension into the JavaScript runtime
  // to create/interact with JavaScript

  // Note: This shares the same thread-safe idiosyncrasies as napi
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
    console.log(process._linkedBinding("my_native_extension")) // { hello: 42 }
  "#)?;

Ok(())
}
```
