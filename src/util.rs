use chrono::prelude::*;
pub fn get_current_time() -> String {
  let now = Utc::now();
  let t = now.format("%Y-%m-%d %H:%M:%S").to_string();
  t
}