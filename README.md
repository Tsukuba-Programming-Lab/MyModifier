# My Modifier

```rust
#[modifier]
trait Dev {}

#[modifier_callee(Dev)]
fn for_developper() {
    // ...
}

fn main() {
    // Ok
    dev! {{
        for_developper();
    }}

    // Compile error
    for_developper();
}
```

## Examples

- [simple](examples/simple)

```
$ cargo run -p example_simple
Hello, World!
```
