# Build with rust nightly
FROM rustlang/rust:nightly as chef

ENV SQLX_OFFLINE=true

WORKDIR /app

# Install cargo-chef so we can use it for caching
# I tried using the docker image, but I couldn't get it working with
# the +nightly below.
RUN apt update && apt install lld clang -y && cargo install cargo-chef

FROM chef as planner
COPY . .

# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo +nightly chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin minimail

# Use the slim nightly rust version as our runtime.
FROM rustlang/rust:nightly-slim as runtime
WORKDIR /app
COPY /config config
COPY --from=builder /app/target/release/minimail minimail

ENTRYPOINT ["./minimail"]
