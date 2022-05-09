# clone-macro

A simple macro to make cloning data before passing it into a `move` closure or block.

This macro is intentionally designed to be compatible with
`rustfmt` formatting.

You can use this macro throughout your crate without needing to explicitly
import it every time as follows:
```rust
#[macro_use]
extern crate clone_macro;

/* ... */

clone!(/* ... */);
```

Otherwise, you can `use` it as normal.
```rust
use clone_macro::clone;

/* ... */
clone!(/* ... */);
```

## Syntax
The `clone!` macro takes a comma separated list of either one of two forms
which can have an optional `mut` prefix modifier, followed by an arbitrary
expression.

For example, the following is a valid call
```rust

let a = 1;
let b = 2;

clone!([mut a, b], ());
```

and desugars down to:
```rust
let a = 1;
let b = 2;

{
    let mut a = a.clone();
    let b = b.clone();

    ()
};
```

The clone list can also take a second form, which is an arbitrary expression
followed by `as` and the name of the variable. For example:
```rust


let s = &quot;Hello, there!&quot;;

clone!([{ s.len() } as len], move || {
    assert_eq!(len, &quot;Hello, there!&quot;.len());
});
```

The above desugars into:
```rust

let s = &quot;Hello, there!&quot;;

{
    let len = &quot;Hello, there!&quot;.len();

    move || {
        assert_eq!(len, &quot;Hello, there!&quot;.len());
    }
};
```

This macro is most useful when the second argument is a closure, and is what
it is intended to work with, though not strictly so.

All forms mentioned above can be mixed and matched, including adding a `mut` modifier
for the second form as:
```rust
mut { $expr } as $ident
```

## Examples
### Basic Usage

```rust
use clone_macro::clone;

let s = &quot;You are a beautiful being!&quot;.to_string();

let c = clone!([s], move || {
    println!(&quot;{s}&quot;);
});

c();

// `s` wasn&#x27;t directly moved, rather, cloned first, then moved; therefore,
// we can still use `s`
assert_eq!(s.as_str(), &quot;You are a beautiful being!&quot;);
```

We can also declare the cloned `move` as `mut`:
```rust
use clone_macro::clone;

let a = 7;
let b = 0;
let d = 12;

le
t mut c = clone!([a, mut b, d], move || {
    b = 42 - a - d;

    println!(&quot;a + b + d = {}&quot;, a + b + d);
});

c();

assert_eq!(a, 7);
assert_eq!(b, 0);
assert_eq!(d, 12);
```

### Advanced Usage
We can clone arbitrary expressions:
```rust
use clone_macro::clone;

struct MyStruct {
    some_field: String,
}

let s = MyStruct {
    some_field: &quot;Beyond measure.&quot;.to_string(),
};

let mut c = clone!([{ s.some_field } as some_field, mut { s.some_field } as mut_some_field], move || {
    mut_some_field.clear();

    assert!(mut_some_field.is_empty());

    assert_eq!(some_field.as_str(), &quot;Beyond measure.&quot;);
});

c();

assert_eq!(s.some_field.as_str(), &quot;Beyond measure.&quot;);
```

License: MIT