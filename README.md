# zip_clone

Zip an iterator to a repeatedly cloned object.

Pass an owned object that implements `Clone` to create an iterator that zips
the original iterator with clones of the object.

One iteration returns the original object, hence using one fewer clones than
`iter.zip(repeat_with(|| cloned.clone()))`.

This is useful for loops where a value is cloned for each iteration, but is not
used after the iteration.

Instead of cloning a value 10 times using:
```rust
let mut v = vec![String::new(); 10];
let s = String::from("Hello");
for i in 0..10 {
    v[i] = s.clone();
}
```
clone the value 9 times using:
```rust
use zip_clone::ZipClone as _;

let mut v = vec![String::new(); 10];
let s = String::from("Hello");
for (i, s) in (0..10).zip_clone(s) {
    v[i] = s;
}
```

This is especially useful when an iterator usually returns a single value, to avoid expensive cloning for the common case:
```rust
use zip_clone::ZipClone as _;

let home_city = "Birmingham"; // use ; if person claims multiple home cities
let mut v = vec![];
let s = String::from("Hello from ");
for (city, mut message) in home_city.split(';').zip_clone(s) {
    message.push_str(city);
    v.push(message);
}
```

`zip_clone` avoids cloning if items are skipped. The following code uses the
original `String` for the single value produced, avoiding any cloning.
```rust
use zip_clone::ZipClone as _;

let s = String::from("Hello");
let _ = (0..10).zip_clone(s).last();
```
