# `implicit-trait`

Create implicit traits to add methods to existing types.

```rust
use implicit_trait::implicit_trait;

// Assume Foo is from another crate, so we can't implement methods on it.
pub struct Foo {
    pub bar: i32,
    pub baz: String,
}

// Define new methods on Foo
#[implicit_trait]
impl FooExt for Foo {
    fn bar(&self) -> i32 {
        self.bar
    }

    fn baz(&self) -> &str {
        &self.baz
    }
}

// Use the new methods
fn main() {
    let foo = Foo {
        bar: 42,
        baz: "hello".to_string(),
    };
    assert_eq!(foo.bar(), 42);
    assert_eq!(foo.baz(), "hello");
}
```