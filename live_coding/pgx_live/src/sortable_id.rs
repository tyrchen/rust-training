use pgx::cstr_core::CStr;
use pgx::*;
use serde::{Deserialize, Serialize};
use uuid7::{uuid7, Uuid};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    PostgresType,
    PostgresEq,
    PostgresOrd,
    PostgresHash,
)]
#[inoutfuncs]
pub struct SortableId {
    inner: Uuid,
}

impl InOutFuncs for SortableId {
    fn input(input: &CStr) -> Self
    where
        Self: Sized,
    {
        let id = input
            .to_str()
            .expect("input is not a valid UTF-8 string")
            .parse()
            .expect("input is not a valid UUID");
        Self { inner: id }
    }

    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&self.inner.to_string());
    }
}

impl Default for SortableId {
    fn default() -> Self {
        Self { inner: uuid7() }
    }
}

#[pg_extern]
fn generate_sortable_id() -> SortableId {
    Default::default()
}
