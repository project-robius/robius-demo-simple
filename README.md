# robius-demo

This is a simple Makepad app that demonstrates various Robius platform abstraction crates:
* [`robius-authentication`](https://github.com/project-robius/robius)
* [`robius-open`](https://github.com/project-robius/robius)
* [`robius-location`](https://github.com/project-robius/robius)

## Building and running

### Native desktop builds
On native desktop, simply run:
```sh
cargo run
```

> [!IMPORTANT]
> Certain platform APIs/services require a fully codesigned application bundle on certain platforms.
>
> For example, location services do not work on macOS unless the application has been bundled and packaged.
> See [how we do this for Robrix](https://github.com/project-robius/robrix/blob/fc8bedf767caab88709b6efb31719e6817eb2f39/README.md#packaging-robrix-for-distribution-on-desktop-platforms), a similar Makepad + Robius app.


### Cross-compiling for Android
To build for Android, you need to first install the `cargo makepad` build tool,
and then use it to installe the Android SDK + NDK.
```sh
cargo install --force --locked --git https://github.com/makepad/makepad.git --branch dev cargo-makepad
```
```sh
cargo makepad android install-toolchain
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
