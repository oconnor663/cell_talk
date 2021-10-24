Interior Mutability Intermediate and Advanced Rust Programmers

The mutable aliasing rule has loopholes: interior mutability.
Mutex
  - Normally: Append to a string from multiple threads.
    - example from Firehose
    - This talk is for people who have learned enough Rust that this example
      feels pretty comfortable. If you haven't written any Rust yourself yet,
      the talk will be pretty hard to follow, but more importantly it might not
      be clear *why we care* about any of this.
    - The first time we see this example, the focus is usually on what Arc is
      doing, and how that interacts with .clone() and the move closure. But
      today, we're going to dive deep on what Mutex is doing.
      - Of course, we all know what it's doing: it's locking the String so that
        only one thread mutates it at a time. That's the same in any language.
        But let's take a close look at it, from the perspective of the mutable
        aliasing rule in Rust.
  - Here: Take two aliasing references to the same string and increment it.
RwLock
  - why not always use RwLock? hold that thought
Atomics
  - kind of like a mutex, but at the hardware level
  - into_iter() vs into_par_iter() contention benchmarks
  - link to Atomic Weapons
RefCell
  - Aliasing on a single thread, without getting the OS or the hardware involved.
  - This is kind of exotic. Aliasing between different threads is sort of the
    whole point of multithreading, but most single threaded programs can be
    written without any aliasing.
  - Send and Sync
  - note that Mutex is always Sync
- Cell
  - But once we're on a single thread, why do we need to lock anything at all?
    What could go wrong?
  - Well, interior references into a Vec would be UB. But what if we forbid
    those?
  - "Interior" mutability is kind of a misnomer here, because there's no
    exterior.
- &Cell
  - Holy shit.
- UnsafeCell
- an unsound Cell
- unsoundness in clone_cell()


- "If I'm not using multiple threads, and I'm not taking interior pointers
  to things, why can't I mutate through a shared reference?"
- "Why is Rust so restrictive? Why not have something in between & and &mut?"
- this in between thing is called "interior mutability"
- atomics
  - work just like in C and C++
    - recommend Herb Sutter's Atomic Weapons
  - ...which is actually kind of surprising!
  - but atomics aren't generic
- you've also seen Arc<Mutex>>
  - note how it's letting you get &mut from &.
  - but this doesn't violate the fundamental &mut rule
- there's also RwLock, which is the same but with many writers
- there's also RefCell, which is like RwLock but single-threaded
  - demo Send and Sync
  - note that Arc isn't Send when its contents aren't Sync
  - rare because multithreading is the most common reason for aliasing
  - side note, Mutex is always Sync, RwLock is not
- these are all fundamentally similar. they're all about getting your hands on &mut
- but there's another interior mutability type that's really different: Cell
  - also rare, like RefCell
- set() through a shared reference
- get() with Copy types
- swap/replace/take() or into_inner() with non-Copy types
  - why not clone()? wait until the end...
- so wait, isn't this a crappier RefCell with more restrictions?
  - well, for one thing it has no space overhead, #[repr(transparent)]. but more importantly...
  - from_mut and as_slice_of_cells
  - compare to slice::from_mut and array::from_mut.
  - also note that GhostCell exists
- &Cell<T> is a third pointer type, in between & and &mut
  - aliasing and mutability are allowed together!
  - multithreading is forbidden
  - interior references are mostly forbidden, pointing into a Vec or enum would be unsafe
  - missing features:
    - Cell projection. The standard library supports projecting into slices but
      not structs or tuples. https://crates.io/crates/cell-project.
    - Method receivers. https://github.com/rust-lang/rust/issues/44874
- But Cell is just some library struct? How does Rust know it's so special?
  - UnsafeCell
  - aliasing with Cell: https://godbolt.org/z/88sv37YbG
  - MyCell: https://godbolt.org/z/cjMTT3xa9
  - MyCell: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=410558ec9808aad986706b575fb8654b
- circular Cells and an unsound clone function
  - the fundamental problem: we can't know what .clone() is doing.
  - https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=fde5ad24988b4342d77563a1507ed439
  - This fails Miri.

- http://smallcultfollowing.com/babysteps/blog/2012/11/18/imagine-never-hearing-the-phrase-aliasable/
- https://github.com/rust-lang/rust/blob/c9d4ad07c4c166d655f11862e03c10100dcb704b/doc/tutorial-borrowed-ptr.md
- https://medium.com/@GolDDranks/things-rust-doesnt-let-you-do-draft-f596a3c740a5
- https://users.rust-lang.org/t/why-does-cell-require-copy-instead-of-clone/5769/3
