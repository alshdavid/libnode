use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_property_names".as_bytes();
type SIGNATURE = fn(env: napi_env, object: napi_value, result: *mut napi_value) -> napi_status;
static CACHE: OnceLock<super::super::super::libnode::DynSymbol<SIGNATURE>> = OnceLock::new();

pub unsafe fn napi_get_property_names(
  env: napi_env,
  object: napi_value,
  result: *mut napi_value,
) -> napi_status {
  CACHE.get_or_init(|| super::super::super::libnode::libnode_sym(SYMBOL))(env, object, result)
}
