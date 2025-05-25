# zip_clone

Zip an iterator to a repeatedly cloned value.

Pass a value that implements `Clone` to create an iterator that zips
the original iterator with clones of the value.

One iteration returns the original value, using one fewer clones than
`iter.zip(repeat_with(|| cloned.clone()))`.

This is useful for loops where a value is cloned for each iteration, but is not
used after the iteration.

Instead of cloning the `String` 10 times using:
```rust
let mut v = vec![String::new(); 10];
let hello = String::from("Hello");
for elem in v.iter_mut() {
    // `hello` cloned 10 times
    *elem = hello.clone();
}
```
clone the `String` 9 times using:
```rust
use zip_clone::ZipClone as _;

let mut v = vec![String::new(); 10];
let hello = String::from("Hello");
for (elem, hello) in v.iter_mut().zip_clone(hello) {
    // `hello` cloned 9 times, 1 element gets the original `hello`
    *elem = hello;
}
```

This is especially useful when an iterator *commonly* returns a single value, but can return more values, to avoid cloning for the common case:
```rust
let recepients = get_email_recepients(&email); // separated by ,
let mut v = vec![];
let s = String::from("Sent to ");
for (recepient, mut message) in recepients.split(',').zip_clone(s) {
    message.push_str(recepient);
    v.push(message);
}
```

`zip_clone` avoids cloning if items are skipped using methods including `last`, `nth` and `skip`.
The following code uses the original `String` for the single value produced, avoiding any cloning.
```rust
use zip_clone::ZipClone as _;

let hello = String::from("Hello");
let _ = (0..10).zip_clone(hello).last();
```

For other methods, if possible, it is better to filter the iterator before adding `zip_clone`:
```rust
use zip_clone::ZipClone as _;

let mut v = vec![String::new(); 10];
let hello = String::from("Hello");
for (elem, hello) in v.iter_mut().take(5).zip_clone(hello) {
    // `hello` cloned 4 times, 1 element gets the original `hello`
    *elem = hello;
}
```
