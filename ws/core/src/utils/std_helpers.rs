

pub fn push_if_some<T>(vec: &mut Vec<T>, value: Option<T>) {
  if let Some(v) = value {
      vec.push(v);
  }
}