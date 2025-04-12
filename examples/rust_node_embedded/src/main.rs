use libnode_rs;

pub fn main() {
  libnode_rs::eval_blocking(r"
    console.log('[start] Process A')
    setTimeout(() => console.log('[end]   Process A'), 5000)
  ").unwrap();
}
