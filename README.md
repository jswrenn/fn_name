# `fn_name`
Macros that produce the name of the function they're invoked within.

## Uninstantiated Names
The `uninstantiated!` macro produces the name of the surrounding function or method, *without* generics instantiated; e.g.:
```rust
struct GenericType<A>(A);

impl<A> GenericType<A> {
    fn generic_method<B>(self, _: B) -> &'static str {
        fn_name::uninstantiated!()
    }
}

fn main() {
    assert_eq!(
        GenericType(42u8).generic_method(false),
        "GenericType<_>::generic_method"
    );
}
```

## Instantiated Names
The `instantiated!` macro produces the name of the surrounding function or method, *including* instantiated generics (if any); e.g.:
```rust
struct GenericType<A>(A);

impl<A> GenericType<A> {
    fn generic_method<B>(self, _: B) -> &'static str {
        fn_name::instantiated!()
    }
}

fn main() {
    assert_eq!(
        GenericType(42u8).generic_method(false),
        "GenericType<u8>::generic_method<bool>"
    );
}
```

## Limitations
The expansion of these macros is not (yet) const evaluable; their implementations rely on `core::any::type_name`, which is not a `const fn`.
