# shrink_fit_wrapper

An _extremely_ simple, "idiomatic" `shrink_to_fit` on any mut.

## Why making this crate?

If you didn't know, [`std::collections`](https://doc.rust-lang.org/stable/std/collections/index.html) reallocate themselves when you want to add an item in it but the initial/current capacity cannot hold up.

However, they don't reallocate when you remove item(s) which can often causes "memory leaks" problem(s) when you use them as global state.

This crate allow you to do that: `shrink_to_fit` on any [`std::collections`] "mut".

## How to use?

1. Install this crate into your project.

2. Wrap any [`Vec`] or [`HashMap`] with the [`ShrinkFitWrapper`] struct:

```rust
use shrink_fit_wrapper::ShrinkFitWrapper;

let mut my_vec = ShrinkFitWrapper::new(Vec::<u32>::with_capacity(3));

// You can also put `shrink_to_fit` period with it.
// let mut my_vec = ShrinkFitWrapper::new(Vec::<u32>::with_capacity(3)).set_shrink_duration_cycle(Duration::from_secs(2));

// use `.as_mut()` to borrow it mutably.
my_vec.as_mut().push(2);

// wait and shrink fit at another time.
// std::thread::sleep(Duration::from_secs(2));
// drop(my_vec.as_mut());

// Checking if the length correspond to the capacity
// assert_eq!(my_vec.len(), my_vec.capacity());

```

*Note*: It is worth noting that by setting a `shrink_duration_cycle`, the wrapper doesn't periodically `shrink_to_fit` the underlying container at that duration. See [`ShrinkFitWrapperMutGuard::drop` implementation](https://github.com/tonymushah/shrink_fit_wrapper/blob/0b17005e22d749915957a497728d8ce2d64075a1/src/lib.rs#L174).

## License

MIT
