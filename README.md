# Orbis Kernel Framework

Orbis Kernel Framework (OKF) is a Rust crate for developing an application that run inside a PS4 kernel.

## Develop a kernel application

Before start you need to install `x86_64-unknown-none` target:

```sh
rustup target add x86_64-unknown-none
```

Keep in mind that any static item that need to be dropped **will live forever until the PS4 is restarted** because Rust will not drop it. This is not an issue on the user-mode application due to the kernel will free all of used resources after the process is terminated.

### Create an executable project

```sh
cargo new hello_world
```

### Set output target

Create `.cargo/config.toml` in the directory that was created by the above command and add the following content to it:

```toml
[build]
target = "x86_64-unknown-none"
rustflags = ["--cfg", "fw=\"FW\""]
```

Replace `FW` with the firmware version you want to run on (e.g. `1100` for 11.00). You need to add additional dependency to `Cargo.toml` on the root of the project depend on the firmware you choose here (e.g. `okf-1100` for 11.00):

```toml
[target.'cfg(fw = "1100")'.dependencies]
okf-1100 = { version = "0.1.0", features = ["panic-handler"] }
```

If no public crate available for your firmware you need to build one. See one of available crate as a reference.

## License

MIT
