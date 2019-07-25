# ![RealWorld Example App](logo.png)

> ### Seed codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld](https://github.com/gothinkster/realworld) spec and API.

### [Demo](https://github.com/gothinkster/realworld) &nbsp;&nbsp;&nbsp;&nbsp;[RealWorld](https://github.com/gothinkster/realworld)

This codebase was created to demonstrate a fully fledged fullstack application built with **Seed** including CRUD operations, authentication, routing, pagination, and more.

We've gone to great lengths to adhere to the **Seed** community styleguides & best practices.

For more information on how to this works with other frontends/backends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.

# How it works

> Describe the general architecture of your app here

# Getting started

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Update Rust: `$ rustup update`
3. Install WASM target: `$ rustup target add wasm32-unknown-unknown`
4. Install [cargo-make](https://sagiegurari.github.io/cargo-make/): `$ cargo install --force cargo-make`
5. Build project from its root: `$ cargo make build`
6. Start local server: `$ cargo make serve`
7. Open in you browser [localhost:8000](http://localhost:8000/)

# Contributing

1. Create issues and PRs - bugs, missing documentation, typos, unreadable code...
2. Squash commits, rebase on the current `master` and run `$ cargo make verify` (+ commit changes, if any) before creating PR.
