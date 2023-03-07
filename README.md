
# leptos-tracked

This crate provides utility traits for interacting with [`leptos`](https://github.com/leptos-rs/leptos) signals.

The provided utilities provide access to standard library traits or functions without having to pass closure's to the signals `update` function.

## Examples

Use any of the `tracked_*` functions to update a mutable signal. 

```rust
use leptos::create_signal;

let (read_i32, write_i32) = create_signal(cx, 4 as i32);
let (read_bool, write_bool) = create_signal(cx, false);
let (read_str, write_str) = create_signal(cx, String::from("Hello"));
let (read_vec, write_vec) = create_signal(cx, vec![1, 2, 3]);

use leptos_tracked::Mul;

write_i32.tracked_mul(10);
assert!(read_i32.get(), 40);

use leptos_tracked::Toggle;

write_bool.tracked_toggle()
assert!(read_bool.get(), true);

use leptos_tracked::{Add, Extend};

write_str.tracked_add(" ");
write_str.tracked_extend(vec!["World", "!"])
assert!(read_bool.get(), "Hello World!");

use leptos_tracked::{TrackedVec};

write_vec.tracked_pop();
write_vec.tracked_push(4)
assert!(read_vec.get(), vec![1, 2, 4])

write_vec.tracked_clear();
assert!(read_vec.get(), vec![] as Vec<i32>)
```

