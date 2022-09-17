# pgx_live

## What to cover

- pgx general intro
  - install CLI: cargo install pgx
  - initialization: cargo pgx init
  - create project: cargp pgx new pgx_live
  - generated skeleton
    - Cargo.toml
    - src/lib.rs
- first function: my_generate_series(start, end, step)
  - no default / with default - see output of `\df` & `cargo pgx schema`
  - comparison with generate_series()
- second function: to_lowercase(s)
  - Option arg / Option return
  - logging: info!()
- user defined type: SortedId
  - introduce rust ecosystem: cargo add uuid7 / serde
  - data structure with PostgresType / Eq / Hash / Ord
  - generate_sortable_id()
  - try with sql
- json schema
  - cargo add json-schema
- spi function

```rust
#[pg_extern]
fn extract_ts(ts: &str) -> i64 {
    let sql = format!(
        "select (extract(epoch from timestamptz '{}') * 1000)::bigint",
        ts
    );
    Spi::get_one(&sql).expect("SPI result was null")
}
```

## What hasn't been covered

- triggers
- aggregates
  - state / finalize
- shared memory
  - heapless
- bg worker
  - _PG_init()
  - while BackgroundWorker::wait_latch(Some(Duration::from_secs(10))) {}
