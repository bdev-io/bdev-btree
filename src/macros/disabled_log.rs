#![allow(unused_attributes)]

#[macro_export]
macro_rules! trace {
  (target: $target:expr, $($arg:tt)+) => ($crate::io::_eprint($($arg)+));
  ($($arg:tt)+) => {
    let output = stringify!($($arg)+);
    println!("{}", output);
  }
}

#[macro_export]
macro_rules! debug {
  (target: $target:expr, $($arg:tt)+) => ($crate::io::_eprint($($arg)+));
  ($($arg:tt)+) => {
    let output = stringify!($($arg)+);
    println!("{}", output);
  }
}

#[macro_export]
macro_rules! info {
  (target: $target:expr, $($arg:tt)+) => ($crate::io::_eprint($($arg)+));
  ($($arg:tt)+) => {
    let output = stringify!($($arg)+);
    println!("{}", output);
  }
}

#[macro_export]
macro_rules! warn {
  (target: $target:expr, $($arg:tt)+) => ($crate::io::_eprint($($arg)+));
  ($($arg:tt)+) => {
    let output = stringify!($($arg)+);
    println!("{}", output);
  }
}

#[macro_export]
macro_rules! error {
  (target: $target:expr, $($arg:tt)+) => ($crate::io::_eprint($($arg)+));
  ($($arg:tt)+) => {
    let output = stringify!($($arg)+);
    println!("{}", output);
  }
}

