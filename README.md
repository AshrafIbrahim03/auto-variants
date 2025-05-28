# Auto Variants
This is a crate that exposes a derive macro that returns all of an enum's variants. Here's a simple example:

```rust 
#[derive(Variants, PartialEq, Debug)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}
let correct_list = [
  Directions::Up,
  Directions::Down,
  Directions::Left,
  Directions::Right,
];
assert_eq!(correct_list, Directions::variants());
```

This is a pretty small example, but might save a lot of time if it's a big enum. 

This is implemented so that a fixed sized array is made at compile time that is made up of all enum variants. A reference is returned when using the `variants` method so this function is very lean.
