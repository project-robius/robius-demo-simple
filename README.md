# auth_test

This is a simple Makepad app that tests the [`robius-authentication`](https://github.com/project-robius/robius-authentication) crate.

## Building and running

### Local build set up
Currently this expects local copies of two other sibling directories, `makepad` and `robius-authentication`, with a directory structure like so:
```
projects
├── auth_test
├── makepad
└── robius-authentication
```

The `makepad` project should be checked out to the [latest commit of branch `rik`](https://github.com/makepad/makepad/tree/rik),
while the `robius-authentication` project should be checked out to [latest commit of branch `cargo_makepad_build`](https://github.com/project-robius/robius-authentication/tree/cargo_makepad_build).

In the near future we will specify direct git dependencies once the rate of change slows.

### Compiling and running
On native desktop, simply run:
```sh
cargo run
```

To build for Android, do the following:
```sh
cargo run --manifest-path ../makepad/tools/cargo_makepad/Cargo.toml --release -- android run -p auth_test --release
```

Once `auth_test` is running, you should see an `Authenticate` button -- click that to bring up the platform-native authentication prompt.
