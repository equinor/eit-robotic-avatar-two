FROM rust:1.64 AS builder 

#Tools to build server
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

#Copy over src
WORKDIR /app
COPY . .

#Tools to build front-end
RUN chmod +x tools/build-dependency.sh
RUN tools/build-dependency.sh

#Build webapp
RUN trunk build --release

#Build the server
RUN cargo build --target x86_64-unknown-linux-musl --release --bin server


FROM scratch
WORKDIR /app
COPY --from=server /app/target/x86_64-unknown-linux-musl/release/server .

USER 101:101

EXPOSE 3000
CMD ["/app/server"]
