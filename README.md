# piston-examples [![Build Status](https://travis-ci.org/PistonDevelopers/piston-examples.svg?branch=master)](https://travis-ci.org/PistonDevelopers/piston-examples)

A collection of examples using the Piston game engine

[How to contribute](https://github.com/PistonDevelopers/piston/blob/master/CONTRIBUTING.md)

## How to build & run examples

To build the examples, you need Rust and Cargo installed.

1. Install Rustlang for example through [rustup](https://rustup.rs/)
2. Invoke `cargo run --example <NAME>` where the `NAME` are a file names from `examples` directory
2.1. Optionally navigate to `examples/<directory>` and invoke `cargo run` for more complex examples

## Troubleshooting

* [I get `ld: library not found for -lSDL2` error on OSX](https://github.com/PistonDevelopers/rust-empty/issues/175)

* I get "GL context creation failed" when running an example.

  It's likely your hardware or driver doesn't support PistonWindow's default OpenGl spec. Just change it to something
  you can support at the beginning of the example. See hello_world.rs for an example.
