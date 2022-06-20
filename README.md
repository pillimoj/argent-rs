# Argent backend built with Rust

## Run it

### Database needs to run

`docker compose up -d db`

### Run debug version

`cargo run`

### Run release buld

`make rundocker`
The container runs similarly to the way it runs in production with the a few minor differences (CORS, auth cookie settings)

## Deploying

`make deploy` will build a container ready for release

## TODO

- migrations
- code structure
- improve error handling
