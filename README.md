# robius-demo

This is a simple Makepad app that demonstrates various Robius platform abstraction crates:
* [`robius-authentication`](https://github.com/project-robius/robius-authentication)
* [`robius-open`](https://github.com/project-robius/robius-open)

## Building and running

### Native desktop builds
On native desktop, simply run:
```sh
cargo run
```

### Cross-compiling for Android
To build for Android, you need to first install the `cargo makepad` build tool,
and then use it to installe the Android SDK + NDK.
```sh
cargo install --force --locked --git https://github.com/makepad/makepad.git --branch dev cargo-makepad
```
```sh
cargo makepad android install-toolchain --full-ndk
```
> You only need to run the above commands once on your build machine.

-------------------------------------------------------------------------------

Then, to build and run this `robius-demo-simple` app, do the following:
```sh
cargo makepad android run -p robius-demo-simple --release
```

Note that the above `android run` command will look for a running Android emulator or a physically-connected Android device.


### Interactive demo/test functionality

Once the demo app is running, you should see a simple view with several labeled buttons. Click whichever buttons that corresponds to the functionality you wish to test out.
