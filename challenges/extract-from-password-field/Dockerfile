FROM debian:stable-slim
WORKDIR /usr/src/app
EXPOSE 8000
COPY src/Rocket.toml ./
COPY target/release/extract-from-password-field ./
CMD ["./extract-from-password-field"]
