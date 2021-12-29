FROM rust

RUN cargo install diesel_cli --no-default-features --features mysql

RUN cargo install cargo-watch

WORKDIR /usr/src/app

EXPOSE 8000

VOLUME ["/usr/local/cargo"]