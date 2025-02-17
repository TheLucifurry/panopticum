/// Creates new String
#[macro_export]
macro_rules! s {
  () => {
      String::new()
  };
  ($s:expr) => {
      String::from($s)
  };
}


pub fn push_if_some<T>(vec: &mut Vec<T>, value: Option<T>) {
  if let Some(v) = value {
      vec.push(v);
  }
}
