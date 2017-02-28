# piston-examples [![Build Status](https://travis-ci.org/PistonDevelopers/piston-examples.svg?branch=master)](https://travis-ci.org/PistonDevelopers/piston-examples)

A collection of examples using the Piston game engine

[How to contribute](https://github.com/PistonDevelopers/piston/blob/master/CONTRIBUTING.md)

## How to build & run examples

To build the examples, you need Rust and Cargo installed.

1. Install [Rust](https://www.rust-lang.org/en-US/)
2. In the Terminal window, navigate to the project directory of the example you want to build.
3. Type `cargo build`
4. Type `cargo run --bin hello_world`

## Troubleshooting

* [I get `ld: library not found for -lSDL2` error on OSX](https://github.com/PistonDevelopers/rust-empty/issues/175)

* I get "GL context creation failed" when running an example.

  It's likely your hardware or driver doesn't support PistonWindow's default OpenGl spec. Just change it to something
  you can support at the beginning of the example. See hello_world.rs for an example.
