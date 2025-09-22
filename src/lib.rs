//! # segtr
//!
//! `segtrs` is a library crate with miscellaneous utilities, many involving
//! mathematical tasks like factoring, or computing a term of a certain
//! sequence. As a new Rust user, it also serves to practice Rust, maintaining
//! a simple crate, while sharing my code with others.

pub mod numt;
pub mod bigint;
pub mod fib;
pub mod io;

pub use bigint::BigInt;
pub use fib::FibonacciIterator;
