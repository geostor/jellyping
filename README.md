# jellyping

Find Jellyfin servers on your local network.

## Build

Requires a Rust toolchain ([rustup.rs](https://rustup.rs)).

```sh
cargo build --release
```

## Run

```sh
cargo run --release
# or, after building:
./target/release/jellyping
```

To install it on your `PATH`:

```sh
cargo install --path .
jellyping
```

### Example output

```json
[
  {
    "Address": "http://192.168.1.50:8096",
    "Id": "a1b2c3d4e5f6...",
    "Name": "living-room",
    "EndpointAddress": null,
    "version": "10.10.3",
    "productName": "Jellyfin Server",
    "os": "Linux",
    "setupComplete": true
  }
]
```

### Exit codes

- `0`: at least one server was found
- `1`: no servers found, or the broadcast couldn't be sent
