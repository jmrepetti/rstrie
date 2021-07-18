# rstrie

Rust radix trie implementation. NSFW.


Wikipeadia link about [Radix Tree](https://en.wikipedia.org/wiki/Radix_tree).

## Usage

```rust
use rstrie::{RSTrie};

let mut tr = RSTrie::<u8>::new();
tr.add("romane", 1);
tr.add("romanus", 2);
tr.add("romulus", 3);
tr.add("rubens", 4);
tr.add("ruber", 5);
tr.add("rubicon", 6);
tr.add("rubicundus", 7);
assert_eq!(Some(1), tr.find("romane").unwrap().value);
assert_eq!(Some(2), tr.find("romanus").unwrap().value);
assert_eq!(Some(3), tr.find("romulus").unwrap().value);
assert_eq!(Some(4), tr.find("rubens").unwrap().value);
assert_eq!(Some(5), tr.find("ruber").unwrap().value);
assert_eq!(Some(6), tr.find("rubicon").unwrap().value);
assert_eq!(Some(7), tr.find("rubicundus").unwrap().value);
```

The tree look like this, although in this library the keys/fragments and associated value belongs to the node, not to the edges.

![wikipedia image of a patricia trie](https://upload.wikimedia.org/wikipedia/commons/thumb/a/ae/Patricia_trie.svg/700px-Patricia_trie.svg.png)


Associated values can be custom types like functions.

```rust
use rstrie::{RSTrie};

type FnType = fn(i32, i32) -> i32;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

let mut tr = RSTrie::<FnType>::new();
let add_fn: FnType = add;
tr.add("add_fn", add_fn);
let adder = tr.find("add_fn").unwrap().value.unwrap();
assert_eq!(42, adder(20,22));
```

Some types, like String, are not supported for values yet, but I hope in the future this won't be a problem. This is a free time learning project, if you find it useful you are welcome to contribute.

## TODO

The public API should stay the same, but the logic of inserts and find may change in the process of making the code cleaner and simple.

- deletion
- maybe placeholders in keys to allow pattern matching. Example: '/users/:id' or '/whatever/*'.
