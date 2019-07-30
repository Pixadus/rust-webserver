This is a very basic webserver made in Rust, designed to serve up all content in a static HTML directory to a LAN address.

## SETUP

1. Clone the git repository.
2. (OPTIONAL) Change the webserver's port from `8080` in `src/main.rs` to a port of your choosing. Note that reserved ports, such as port 80 requires root privileges.
3. Making sure you have a working Rust and Cargo installation, compile the binary with 'cargo build'.
4. You now have a working webserver! Run it with `cargo run`. It will serve all content in the `html/` directory.
