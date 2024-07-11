# convert tailwind css into a .css file
FROM node:22 AS node-builder

WORKDIR /opt/homepage

COPY package.json ./

RUN npm install

COPY tailwind.config.js input.css ./

RUN npx tailwindcss -i ./input.css -o ./assets/tailwind.css --minify

# build wasm
FROM rust:latest AS rust-builder

WORKDIR /opt/homepage

ENV BUILD_METHOD=docker

RUN rustup target add wasm32-unknown-unknown

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN cargo binstall trunk --no-confirm

COPY src ./src
COPY Cargo.toml Cargo.lock Dioxus.toml ./
COPY --from=node-builder /opt/homepage/assets/tailwind.css ./assets/

RUN trunk build --release

FROM nginx:1.27

COPY --from=rust-builder /opt/homepage/dist /usr/share/nginx/html/

COPY nginx.conf /etc/nginx/nginx.conf
