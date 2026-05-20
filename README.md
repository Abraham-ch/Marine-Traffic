# Marine Traffic

Realtime maritime tracking project built with Rust.

Currently focused on:

- ingesting AIS vessel data
- learning async Rust
- realtime event processing
- geospatial systems

## Workspace

```txt
apps/
├── ingest-service/
└── backend/
```

For running the ingest-service just use:

```shell
cargo run -p ingest-service
```

Tech Stack:

- Rust
- Tokio
- Axum
- Diesel
- PostgreSQL
- WebSockets
