# rust-number-printer
Prints numbers as text (e.g. 115 -> "one hundred and fifteen")

In your `Cargo.toml`:


```toml
[dependencies.printer]
git = "https://github.com/bkputnam/rust-number-printer.git"
```

In your rust code:

```rust
extern crate printer;

fn main() {
    println!("{}", printer::format(567));
    // prints "five hundred and sixty-seven"
}
```