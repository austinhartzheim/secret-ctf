FROM debian:stable-slim
WORKDIR /usr/src/app
EXPOSE 2323
EXPOSE 4000-4099/udp
COPY target/release/port-knocking ./
ENTRYPOINT ["./port-knocking"]
