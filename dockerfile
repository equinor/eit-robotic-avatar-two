FROM rust:1.64 AS builder 

WORKDIR /app
COPY . .

#Install Cargo tools
RUN chmod +x .devcontainer/cargo-tools.sh
RUN .devcontainer/cargo-tools.sh

#Build the binaries
RUN cargo build --release 

#Build webapp
RUN trunk build

FROM nginxinc/nginx-unprivileged:1.21.1
WORKDIR /app
COPY --from=builder /app/web-client/dist ./www
COPY --from=builder /app/target/release/robot ./www/robots/robot
COPY --from=builder /app/target/release/server .
COPY ./web-client/proxy ./proxy
COPY ./web-client/proxy/server.conf /etc/nginx/conf.d/default.conf

USER 0
RUN chown -R nginx /etc/nginx/conf.d \
    && chown -R nginx /app \
    && chmod +x ./proxy/init_app.sh

USER 101
# Note that nginx use port 8080 by default in nginx-unprivileged
EXPOSE 8080
CMD /bin/sh -c "./proxy/init_app.sh"