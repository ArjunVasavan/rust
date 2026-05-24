// In normal Rust (std):
//   - you have heap, threads, file system, println!
//   - runs on Linux/Windows/Mac
//
// In embedded Rust (no_std):
//   - no OS, no heap by default, no println!
//   - runs directly on microcontroller hardware
//   - you use core:: instead of std::
//
// This is how every embedded Rust file starts:

#![no_std]   // don't use standard library
#![no_main]  // don't use standard main

// instead of println! you use:
//   - UART to send text to serial monitor
//   - or semihosting for debug output

// this file won't compile alone - just for reading
fn main() {}
