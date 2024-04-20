# zip_clone

Zip an iterator to a repeately cloned object.

Pass an owned object that implements `Clone` to create an iterator that zips
the original iterator with clones of the object.

One iteration returns the original object, hence using one fewer clones than
`iter.zip(repeat_with(|| cloned.clone()))`.

This is useful for loops where a value is cloned for each iteration, but is not
used after the iteration.

Instead of cloning a value 10 times using:
```rust
let v = vec![];
let s = String::from("Hello");
for i in 0..10 {
    let s = s.clone();
    vec.push(s);
}
```
clone the value 9 times using:
```rust
use zip_clone::ZipClone as _;

let v = vec![];
let s = String::from("Hello");
for (i, s) in (0..10).zip_clone(s) {
    vec.push(s);
}
```
