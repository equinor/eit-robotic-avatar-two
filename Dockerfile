FROM rust:1.64 AS builder 

WORKDIR /app
COPY . .

#Install Cargo tools
RUN chmod +x tools/build-dependency.sh
RUN tools/build-dependency.sh

#Build webapp
RUN trunk build --release 

#Build the binaries
RUN cargo build --release --bin server

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/server .

USER 101

EXPOSE 3000
CMD ["/app/server"]