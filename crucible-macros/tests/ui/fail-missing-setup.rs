// A fixture struct without a setup() impl must not compile — the generated reset() body
// calls Self::setup(), which produces a "no function or associated item named `setup`"
// compiler error.
use crucible_macros::fixture;

#[fixture]
pub struct MyFixture {
    pub value: i32,
}

fn main() {
    let mut f = MyFixture { value: 0 };
    f.reset();
}
