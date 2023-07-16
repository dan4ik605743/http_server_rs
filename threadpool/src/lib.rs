//! Crate to use thread pool.
//! # Example
//! ``` ignore
//! use threadpool::ThreadPool;
//!
//! fn main() -> Result<()> {
//!     let n_jobs = 8;
//!     let n_workers = 4;
//!     let pool = ThreadPool::build(workers)?;
//!
//!     for _ in 0..n_jobs {
//!         pool.execute(move || { ... });    
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod threadpool;

pub use crate::threadpool::ThreadPool;
