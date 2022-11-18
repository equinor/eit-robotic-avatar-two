FROM rust:1.64 AS frontend 

#Copy over src
WORKDIR /app
COPY . .

#Tools to build front-end
RUN chmod +x tools/build-dependency.sh
RUN tools/build-dependency.sh

#Build webapp
RUN trunk build


FROM rust:1.64 AS server 

#Tools to build server
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

#Copy over src
WORKDIR /app
COPY . .
COPY --from=frontend /app/server/static /app/server/static

#Build the server
RUN cargo build --target x86_64-unknown-linux-musl --bin server


FROM scratch
WORKDIR /app
COPY --from=server /app/target/x86_64-unknown-linux-musl/debug/server .

USER 101:101

EXPOSE 3000
CMD ["/app/server"]
