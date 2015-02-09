% Rust Sydney

![](logo-v1.png)

[<span style="font-size:80%">rust-lang.org</span>](http://rust-lang.org/)

[<span style="font-size:70%; font-weight: normal;">huonw.github.io/rust-sydney-feb15</span>](http://huonw.github.io/rust-sydney-feb15)


# Hi

I'm Huon Wilson, long-time Rust contributor: wrote a patch, got
hooked... Somehow now organising a meetup.

![](firstpatch.png)


# Preliminaries &ndash; Thanks

- Rod Howarth and Envoy Advanced Technologies (@EnvoyAT)
- Mozilla
- Steve Klabnik


# Preliminaries

Conduct of conduct,
[rust-lang.org/conduct.html](http://rust-lang.org/conduct.html). Summary:

> - We are committed to providing a friendly, safe and welcoming
>   environment for all.
> - Please be kind and courteous. There's no need to be mean or rude.
> - We will exclude you from interaction if you insult, demean or harass anyone.
>   That is not welcome behaviour.



# Background

Open source
([github.com/rust-lang/rust](http://github.com/rust-lang/rust))
programming language. Originally a personal project of Graydon Hoare,
adopted by Mozilla Research in 2009.

[1.0.0-alpha released](http://blog.rust-lang.org/2015/01/09/Rust-1.0-alpha.html)
a month ago, full stable release soon!

# Rust's Aims

A haiku:

> a systems language<br/>
> pursuing the trifecta<br/>
> safe, concurrent, fast

<div class="attribution"><a href="http://twitter.com/lindsey">@lindsey</a></div>

# "Systems language"

Control is critical, overhead minimal.

No garbage collection, no "runtime", use the OS directly where
possible... write the OS too.


<h1></h1>

<img src="embedded.jpg" />
<div class="attribution"><a href="http://www.reddit.com/r/rust/comments/21qogc/im_making_a_note_here_huge_embedded_success/">
farcaller on /r/rust</a></div>

# Safe

*Memory* safety is a priority, avoid GC with powerful static analysis:

- ownership,
- borrowing,
- lifetimes.

Compiler uses annotations on pointers (etc.) to ensure they're always valid.

# Safe &ndash; lifetimes

```rust
fn get_ten<'a>(map: &'a HashMap<i32, String>) -> &'a String {
    let val = &map[10];

    val
}
```

![](lifetimes.png)

# Concurrent

Rust provides tools to handle shared state and message passing
safely, e.g.

- `Arc<String>`, immutable [shared memory](http://doc.rust-lang.org/nightly/std/sync/struct.Arc.html)
- `Arc<Mutex<String>>`, [mutable](http://doc.rust-lang.org/nightly/std/sync/struct.Mutex.html) shared memory
- `channel::<String>()`, [message passing](http://doc.rust-lang.org/nightly/std/sync/mpsc/)

No data races: locking/atomicity enforced when required.

# Concurrent

Possible to implement a general `parallel_map` in safe code (in the
near future: [RFC 258](https://github.com/rust-lang/rfcs/pull/458)), e.g. usable like:

```rust
// Eight integers on the stack
let mut stack_buffer = [0; 8];

// add one to each, in parallel
parallel_map(stack_buffer.iter_mut(), |item| {
    // item is a pointer into the main thread's stack
    *item += 1
});

println!("{:?}", &stack_buffer[]); // [1, 1, ..., 1];
```

Lifetimes, and the `Send` & `Sync` traits allow the compiler to guarantee safety.


# Fast

Few runtime language features to stand in the way of the
compiler. Standard libraries abstractions try to optimise well.

<img src="benchmarksgame.png"/>
<div class="attribution"><a href="http://benchmarksgame.alioth.debian.org/u32/rust.php">
Benchmarks game, x86 one core</a></div>

# Fast

"Functional programming" iterators feel high-level:

```rust
pub fn sum(x: &[i32]) -> i32 {
    x.iter().fold(0, |a, b| a + *b)
}
```

Vectorised by LLVM! (16 `i32`s per iteration.)

```asm
.LBB0_4:
	vpaddd	-48(%rdx), %xmm0, %xmm0
	vpaddd	-32(%rdx), %xmm1, %xmm1
	vpaddd	-16(%rdx), %xmm2, %xmm2
	vpaddd	(%rdx), %xmm3, %xmm3
	addq	$64, %rdx
	addq	$-16, %rdi
	jne	.LBB0_4
```

# Projects

- Operating systems/bare metal (`#rust-osdev`): [zinc](http://zinc.rs)
- Games (`#rust-gamedev`): [piston](http://piston.rs), [snowmew](https://github.com/csherratt/snowmew), [hematite](https://github.com/pistondevelopers/hematite)
- Web servers/development (`#rust-webdev`): [iron](http://ironframework.io/), [nickel.rs](http://nickel.rs/)
- Web browser(s): [servo](https://github.com/servo/servo)
- Cryptography (`#rust-crypto`)
- Rust's compiler and standard library (`#rust-internals`)
