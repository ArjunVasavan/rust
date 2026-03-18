# Learning Rust

A personal repository documenting my journey learning Rust,
approached from an Embedded Systems / Linux Systems Engineering background.

---

## What is Rust?

Rust is a systems programming language designed for performance and memory
safety. Unlike C, Rust guarantees memory safety at compile time — no null
pointer dereferences, no buffer overflows, no use-after-free — without
needing a garbage collector.

It gives you the same low-level control as C but with a compiler that
catches entire classes of bugs before your program ever runs.

```
C / C++      →  Full control, but memory bugs are your problem
Rust         →  Full control, and the compiler catches memory bugs for you
Python/Java  →  Memory safe, but garbage collected — not suitable for embedded
```

---

## Why Rust for Embedded Systems?

Embedded software has traditionally been written in C because:
- No garbage collector (deterministic timing)
- Direct hardware access
- Minimal runtime overhead

Rust satisfies all of these — and adds:

- **Memory safety without GC** — ownership and borrowing rules enforced
  at compile time. No malloc/free mistakes.
- **No undefined behaviour** — C's biggest source of bugs is undefined
  behaviour. Rust eliminates most of it by design.
- **Zero-cost abstractions** — high-level code compiles down to the same
  machine code as hand-written C. You pay nothing at runtime.
- **Fearless concurrency** — the borrow checker prevents data races at
  compile time. Critical for RTOS and multi-threaded embedded systems.
- **Growing embedded ecosystem** — `embedded-hal`, `RTIC` (Real-Time
  Interrupt-driven Concurrency), `probe-rs` for flashing/debugging.
  Rust runs on ARM Cortex-M, RISC-V, ESP32, and more.

Industry adoption is accelerating:
- Linux kernel now accepts Rust for driver development (since kernel 6.1)
- Google, Microsoft, and Amazon are moving safety-critical code to Rust
- AUTOSAR (automotive embedded standard) is evaluating Rust

---

## How Rust Handles Memory (vs C)

In C, you manage memory manually:
```c
char *buf = malloc(64);   // you allocate
strcpy(buf, "hello");
free(buf);                // you must remember to free
// if you forget → memory leak
// if you free twice → undefined behaviour
// if you use after free → security vulnerability
```

In Rust, the compiler handles this through ownership:
```rust
let buf = String::from("hello");  // buf owns this memory
// when buf goes out of scope, memory is automatically freed
// no malloc, no free, no leaks, no use-after-free
```

The three rules Rust enforces at compile time:
1. Every value has exactly one owner
2. When the owner goes out of scope, the value is dropped (freed)
3. You can borrow a value (reference it) but the compiler ensures
   no dangling references ever exist

---

## Repo Structure

```
rust-learning/
├── basics/               # Variables, functions, control flow,
│                         # ownership, borrowing, references
├── primitive_datatypes/  # integers, floats, bool, char,
│                         # type casting, overflow behaviour
├── compound_datatypes/   # tuples, arrays, slices, structs, enums
└── README.md
```

---

## Key Concepts Covered

| Concept              | C equivalent          | Rust approach                        |
|----------------------|-----------------------|--------------------------------------|
| Memory management    | malloc / free         | Ownership — automatic, compile-time  |
| Null pointers        | int *p = NULL         | Option<T> — no null, no crashes      |
| Error handling       | return -1 / errno     | Result<T, E> — explicit, no ignoring |
| Concurrency safety   | your responsibility   | Borrow checker prevents data races   |
| Buffer bounds        | your responsibility   | Checked at compile + runtime         |
| Unsigned overflow    | undefined behaviour   | Wraps in debug, panics in release    |

---

## Background

I come from an Embedded Linux / C systems programming background:
- Strong in C — Linux system calls, IPC, TCP/UDP sockets, /proc filesystem
- Experience with ESP32, ESP8266, LoRa, RFID, Arduino platforms
- Coursework in RTOS, Microprocessors, Digital Electronics

Learning Rust to add memory-safe systems programming to my skillset,
with a focus on eventually applying it to embedded targets (ARM Cortex-M,
ESP32) and Linux kernel modules.

---

## Resources I'm Using

- [The Rust Book](https://doc.rust-lang.org/book/) — official, free, excellent
- [Rustlings](https://github.com/rust-lang/rustlings) — small exercises
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/) — for bare-metal targets

---

## Progress

- [x] Basics — variables, functions, control flow
- [x] Primitive datatypes
- [x] Compound datatypes
- [ ] Ownership and borrowing (deep dive)
- [ ] Structs and enums
- [ ] Error handling — Result and Option
- [ ] Traits and generics
- [ ] Closures and iterators
- [ ] Concurrency — threads, channels
- [ ] Embedded Rust — bare-metal on ARM Cortex-M

---

*Part of my ongoing learning as an Embedded Systems Engineer.*
*Main profile: [github.com/ArjunVasavan](https://github.com/ArjunVasavan)*
