# zip_clone

Zip an iterator to a repeatedly cloned value.
Returns an iterator of 2-tuples containing an iterator item and a clone of the value.

```rust
use zip_clone::ZipClone as _;

let mut iter = vec![2, 3, 4].into_iter().zip_clone("abc".to_owned());
assert_eq!(iter.next(), Some((2, "abc".to_owned())));
assert_eq!(iter.next(), Some((3, "abc".to_owned())));
assert_eq!(iter.next(), Some((4, "abc".to_owned())));
assert_eq!(iter.next(), None);
```

One iteration returns the original value, using one fewer clones than
`iter.zip(repeat_with(|| cloned.clone()))`.

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
let mut v = vec![String::new(); 10];
let hello = String::from("Hello");
for (elem, hello) in v.iter_mut().zip_clone(hello) {
    // `hello` cloned 9 times, 1 element gets the original `hello`
    *elem = hello;
}
```

This is especially useful when an iterator *commonly* returns a single value, but can return more values, to avoid cloning for the common case:
```rust
let messages = get_email_recepients(&email)
    .split(',')
    .zip_clone(String::from("Sent to "))
    .map(|(recepient, mut message)| {
        message.push_str(recepient);
        message
    })
    .collect::<Vec<String>>();
```

`zip_clone` avoids cloning if items are skipped using methods including `last`, `nth` and `skip`.
The following code uses the original `String` for the single value produced, avoiding any cloning.
```rust
let hello = String::from("Hello");
let _ = (0..10).zip_clone(hello).last();
```

For other methods, if possible, it is better to filter the iterator before adding `zip_clone`:
```rust
let mut v = vec![String::new(); 10];
let hello = String::from("Hello");
for (elem, hello) in v.iter_mut().take(5).zip_clone(hello) {
    // `hello` cloned 4 times, 1 element gets the original `hello`
    *elem = hello;
}
```
