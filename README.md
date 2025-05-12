# Buddytalk LiveKit Worker

A Rust worker service that connects to a [LiveKit](https://livekit.io/) room, subscribes to remote audio tracks, and logs received audio frames. This project is designed for real-time audio processing, monitoring, or integration with other systems using the LiveKit infrastructure.

## Features

- Connects to a LiveKit room using the [LiveKit Rust SDK](https://github.com/livekit/rust-sdks)
- Subscribes to remote audio tracks
- Logs received audio frames for further processing or debugging
- Asynchronous, event-driven architecture using [Tokio](https://tokio.rs/)
- Simple configuration via environment variables

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (edition 2021 or later)
- Access to a LiveKit server (cloud or self-hosted)

### Installation

Clone the repository and build the project:

```sh
cargo build --release
```

### Configuration

Create a `.env` file in the project root or set the following environment variables:

- `LIVEKIT_URL`   – The WebSocket URL of your LiveKit server (e.g., `wss://your-livekit-server`)
- `LIVEKIT_TOKEN` – An access token for authenticating with the LiveKit room

Example `.env`:

```
LIVEKIT_URL=wss://your-livekit-server
LIVEKIT_TOKEN=your_jwt_token
```

### Running

```sh
cargo run --release
```

The service will connect to the specified LiveKit room and begin logging audio frames as they are received.

## Usage Example

The core logic is in `src/main.rs`:

```rust
let url = env::var("LIVEKIT_URL").expect("LIVEKIT_URL must be set");
let token = env::var("LIVEKIT_TOKEN").expect("LIVEKIT_TOKEN must be set");

let (room, mut room_events) = Room::connect(&url, &token, RoomOptions::default()).await?;

// Listen for track subscriptions
while let Some(event) = room_events.recv().await {
    match event {
        RoomEvent::TrackSubscribed { participant, track, .. } => {
            // Handle audio tracks
        }
        _ => {}
    }
}
```

## Dependencies

- [livekit](https://crates.io/crates/livekit) – Real-time audio/video/data SDK
- [tokio](https://crates.io/crates/tokio) – Asynchronous runtime
- [dotenv](https://crates.io/crates/dotenv) – Loads environment variables from `.env`
- [tracing](https://crates.io/crates/tracing) – Structured logging
- [futures](https://crates.io/crates/futures) – Asynchronous programming utilities
- [log](https://crates.io/crates/log) – Logging facade

## References

- [LiveKit Rust SDK Documentation](https://docs.rs/livekit)
- [LiveKit Project](https://livekit.io/)
- [LiveKit Rust SDK GitHub](https://github.com/livekit/rust-sdks)

## License

This is a private repository. All rights reserved. Unauthorized use, distribution, or modification of this code is strictly prohibited.
