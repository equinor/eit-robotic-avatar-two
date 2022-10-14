FROM rust:1.64 AS builder 

WORKDIR /app
COPY . .

#Install Cargo tools
RUN chmod +x .tools/build-dependency.sh
RUN .tools/build-dependency.sh

#Build the binaries
RUN cargo build --release --bin server

#Build webapp
RUN trunk build --release 

FROM nginxinc/nginx-unprivileged:1.21.1
WORKDIR /app
COPY --from=builder /app/client/dist ./www
COPY --from=builder /app/target/release/server .
COPY ./client/proxy ./proxy
COPY ./client/proxy/server.conf /etc/nginx/conf.d/default.conf

USER 0
RUN chown -R nginx /etc/nginx/conf.d \
    && chown -R nginx /app \
    && chmod +x ./proxy/init_app.sh

USER 101
# Note that nginx use port 8080 by default in nginx-unprivileged
EXPOSE 8080
CMD /bin/sh -c "./proxy/init_app.sh"