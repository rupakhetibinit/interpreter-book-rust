pub mod eval;
pub mod object;
pub use eval::eval_program;

#[cfg(test)]
mod tests;
