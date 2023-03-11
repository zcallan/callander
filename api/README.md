# Callander API

## Running

```bash
$ cargo run
```

Or for watching:

```bash
$ cargo install cargo-watch
$ cargo watch -q -c -w src/ -x run
```

## Migrations

```bash
$ diesel migration generate <name>
$ diesel migration run
$ diesel migration redo
```

## Deployment

This was a great resource: https://www.codefeetime.com/post/docker-config-for-actix-web-diesel-and-postgres/

```bash
$ fly launch --local-only
```
