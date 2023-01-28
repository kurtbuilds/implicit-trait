/// Can't compile because it puts a private struct in a pub trait
use implicit_trait::implicit_trait;

pub(crate) struct Bar;

struct Foo;

#[implicit_trait(pub)]
impl FooExt for Foo {
    fn bar(&self) -> Bar {
        Bar
    }
}


fn main() {
}