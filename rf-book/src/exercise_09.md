# Exercise 9 - Deployment

There are a few different ways to deploy a leptos site that runs a server, you 
could setup a virtual machine and then manually run the process. Or you could 
containerize it with a `Dockerfile` and deploy to any of the many, many services
 that'll run that for you. For this example I'm going to deploy to 
[fly.io](https://fly.io). 

We have some example Dockerfiles [here](https://book.leptos.dev/deployment/ssr.html#deploy-to-flyio) that can be edited, here is the Debian one:
```Dockerfile
# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bookworm as builder

# If you’re using stable, use this instead
# FROM rust:1.88-bookworm as builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install required tools
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends clang

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/leptos_start /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Run the server
CMD ["/app/leptos_start"]

```

```bash
curl -L https://fly.io/install.sh | sh
```
If you have issues, or for installing to other platforms see the full 
instructions [here](https://fly.io/docs/hands-on/install-flyctl/).

Then login to Fly.io
```bash
fly auth login
```

and manually launch your app using the command
```bash
fly launch
```

## Exercise
Deploy your app to Fly.io(or the provider of your choice)!
