use implicit_trait::implicit_trait;

pub struct Foo {
    pub bar: i32,
    pub baz: String,
}

#[implicit_trait]
impl FooExt for Foo {
    fn bar(&self) -> i32 {
        self.bar
    }

    fn baz(&self) -> &str {
        &self.baz
    }
}

fn main() {
    let foo = Foo {
        bar: 42,
        baz: "hello".to_string(),
    };
    assert_eq!(foo.bar(), 42);
    assert_eq!(foo.baz(), "hello");
}