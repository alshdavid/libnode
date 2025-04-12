use std::ffi::c_char;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_named_property".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  object: napi_value,
  utf8name: *const c_char,
  result: *mut napi_value,
) -> napi_status;
static CACHE: OnceLock<super::super::super::libnode::DynSymbol<SIGNATURE>> = OnceLock::new();



pub unsafe fn napi_get_named_property(
  env: napi_env,
  object: napi_value,
  utf8name: *const c_char,
  result: *mut napi_value,
) -> napi_status {
  CACHE.get_or_init(|| super::super::super::libnode::libnode_sym(SYMBOL))(env, object, utf8name, result)
}
