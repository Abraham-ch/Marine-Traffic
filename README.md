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
├── api/
└── packages/database
```

For running the ingest-service just use:

```shell
cargo run -p ingest-service
```

Also now to allow the income data to allow the future api, we'll need a database so run:

```shell
docker compose up -d
```

then ``diesel setup`` from ``packages/database``

Lastly to generate the ``schema.rs`` just use:

```shell
diesel migration run
```
