# sbc-rs

A library for decoding the SBC (Sub Band Codec) audio format used primary by bluetooth.

This library internally uses Google's [libsbc](https://github.com/google/libsbc/) compiled to Rust using C2Rust and therefore uses a lot of `unsafe`. I hope to gradually replace the `unsafe` code with safe Rust code in the future.

The test cases are the official SBC test cases from the Bluetooth Website.


