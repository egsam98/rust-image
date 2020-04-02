#[macro_export]
macro_rules! json_response {
   ($code:expr, $($json:tt)+) => {
       json!($($json)+).to_response(Some(Status::from_code($code).unwrap()))
   };
   ($($json:tt)+) => {
        json!($($json)+).to_response(None)
   };
}