pub trait Storage<T, E> {
  fn get(key: String) -> String;
  fn set(key: String, value: String) -> Result<T, E>;
}