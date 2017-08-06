FROM debian:stable-slim
WORKDIR /usr/src/app
EXPOSE 2323
EXPOSE 4000/udp
COPY target/release/port-knocking ./
ENTRYPOINT ["./port-knocking"]
