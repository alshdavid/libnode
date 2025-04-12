use super::start_blocking;

pub fn eval_blocking<Code: AsRef<str>>(code: Code) -> crate::Result<()> {
  start_blocking(&["-e", code.as_ref()])?;
  Ok(())
}