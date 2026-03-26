// Applying #[fixture] to an enum must produce a clear compile error.
use crucible_macros::fixture;

#[fixture]
pub enum MyFixture {
    Variant,
}

fn main() {}
