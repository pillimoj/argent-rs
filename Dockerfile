FROM rust:slim as build

# install dependencies
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y libssl-dev pkg-config

# create a new empty shell project
RUN USER=root cargo new --bin argent
WORKDIR /argent
# update the cargo registry which can be slow
RUN echo 'time = "0"' >> Cargo.toml
RUN cargo update

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY ./migrations ./migrations

# build for release
RUN rm ./target/release/deps/argent*
RUN cargo build --release

# our final base
FROM debian:stable-slim

# ssl certs
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates && rm -rf /var/lib/apt/lists/*


# copy the build artifact from the build stage
COPY --from=build /argent/target/release/argent .

# set the startup command to run your binary
CMD ROCKET_PORT=$PORT ./argent
