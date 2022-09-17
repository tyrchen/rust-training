mod sortable_id;

use pgx::*;

pub use sortable_id::*;

pg_module_magic!();

#[pg_extern]
fn hello_pgx_live() -> &'static str {
    "Hello, pgx_live"
}

#[pg_extern]
fn my_generate_series(start: i64, end: i64, step: default!(i64, 1)) -> impl Iterator<Item = i64> {
    (start..=end).step_by(step as usize)
}

#[pg_extern]
fn to_lowercase(s: Option<String>) -> Option<String> {
    info!("to_lowercase called with {:?}", s);
    s.map(|v| v.to_lowercase())
}

#[pg_extern]
fn check_json_schema(v: Json, schema: Json) -> bool {
    jsonschema::is_valid(&schema.0, &v.0)
}

#[pg_extern]
fn check_jsonb_schema(v: JsonB, schema: Json) -> bool {
    jsonschema::is_valid(&schema.0, &v.0)
}

#[pg_extern]
fn extract_ts(ts: &str) -> i64 {
    let sql = format!(
        "select (extract(epoch from timestamptz '{}') * 1000)::bigint",
        ts
    );
    Spi::get_one(&sql).expect("SPI result was null")
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_pgx_live() {
        assert_eq!("Hello, pgx_live", crate::hello_pgx_live());
    }
}

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
