<h1>Simple macro that works just like C's ##</h1>

<h2> The exception is that it uses the `~` sign instead of `##` </h2>

<h3>Example</h3>

```rust
use c_like_concat::concat;

#[derive(Debug, Eq, PartialEq)]
struct FuzzBuzz;

fn main() {
    assert_eq!(concat!(0 ~ x ~ 42), 0x42);
    assert_eq!(concat!(Fuzz ~ Buzz), FuzzBuzz);
    // Sadly, but following code works
    // as `stringify!` will expand to "concat! (Fuzz ~ Buzz)",
    // not to "FuzzBuzz"
    assert_eq!(stringify!(concat!(Fuzz ~ Buzz)), "concat! (Fuzz ~ Buzz)");
}

```
