# auth_test

This is a simple Makepad app that tests the [`robius-authentication`](https://github.com/project-robius/robius-authentication) crate.

## Building and running

### Local build set up
Currently this expects a local copy of the `makepad` project as a sibling directory, with a directory structure like so:
```
projects
├── makepad
└── auth_test
```

The `makepad` project should be checked out to the [latest commit of branch `rik`](https://github.com/makepad/makepad/tree/rik).

In the near future we will specify direct git dependencies on `makepad` once its build tool is more stable.

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
