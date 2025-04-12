use std::ffi::c_void;
use std::sync::OnceLock;

use super::super::*;

const SYMBOL: &[u8] = "napi_get_dataview_info".as_bytes();
type SIGNATURE = fn(
  env: napi_env,
  dataview: napi_value,
  bytelength: *mut usize,
  data: *mut *mut c_void,
  arraybuffer: *mut napi_value,
  byte_offset: *mut usize,
) -> napi_status;
static CACHE: OnceLock<super::super::super::libnode::DynSymbol<SIGNATURE>> = OnceLock::new();



pub unsafe fn napi_get_dataview_info(
  env: napi_env,
  dataview: napi_value,
  bytelength: *mut usize,
  data: *mut *mut c_void,
  arraybuffer: *mut napi_value,
  byte_offset: *mut usize,
) -> napi_status {
  CACHE.get_or_init(|| super::super::super::libnode::libnode_sym(SYMBOL))(env, dataview, bytelength, data, arraybuffer, byte_offset)
}
