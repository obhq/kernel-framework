# Orbis Kernel Framework

Orbis Kernel Framework (OKF) is a Rust crate for developing an application that run inside a PS4 kernel.

## Develop a kernel application

Keep in mind that any static item that need to be dropped **will live forever until the PS4 is restarted** because Rust will not drop it. This is not an issue on the user-mode application due to the kernel will free all of used resources after the process is terminated.

## License

MIT
