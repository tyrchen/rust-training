mod assist;

pub use assist::db_assist;

use pgx::prelude::*;

pgx::pg_module_magic!();

#[pg_extern]
fn hello_db_assist() -> &'static str {
    "Hello, db_assist"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

    #[pg_test]
    fn test_hello_db_assist() {
        assert_eq!("Hello, db_assist", crate::hello_db_assist());
    }
}

/// This module is required by `cargo pgx test` invocations.
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
