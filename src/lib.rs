#[allow(unused_imports)]
use pgrx::prelude::*;

pgrx::pg_module_magic!();

mod api;
mod bgw;
mod clf;
mod executor;
mod guc;
mod util;

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
