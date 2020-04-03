#[macro_export]
macro_rules! json_response {
   ($code:expr, $($json:tt)+) => {
       json!($($json)+).to_response(Some(Status::from_code($code).unwrap()))
   };
   ($($json:tt)+) => {
        json!($($json)+).to_response(None)
   };
}

#[macro_export]
macro_rules! try_str {
    ($expr:expr) => {
        $expr.map_err(|e| e.to_string())?
    };
    ($expr:expr, $msg: expr) => {
        $expr.map_err(|_| $msg)?
    };
}