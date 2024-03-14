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
while the `robius-authentication` project should be checked out to [this commit of branch `main`](https://github.com/project-robius/robius-authentication/commit/a5d7e6ac4ebf81e740ef09e22e3b69215fae6562).

In the near future we will specify direct git dependencies once the rate of change slows.

### Compiling and running

#### Native desktop builds
On native desktop, simply run:
```sh
cargo run
```

#### Cross-compiling for Android
To build for Android, you need to first install the Android SDK + NDK via the cargo makepad tool (you only need to run this once):
```sh
cargo run --manifest-path ../makepad/tools/cargo_makepad/Cargo.toml --release -- android install-toolchain --full-ndk
```

Then, to build and run this `auth_test` app, do the following:
```sh
cargo run --manifest-path ../makepad/tools/cargo_makepad/Cargo.toml --release -- android run -p auth_test --release
```

Note that the above `android run` command will look for a running Android emulator or a physically-connected Android device.


-----------------

Once `auth_test` is running, you should see an `Authenticate` button -- click that to bring up the platform-native authentication prompt.
