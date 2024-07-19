Source code for [my personal homepage](https://harryagustian.xyz/)

Written in Rust, built to Web Assembly using Dioxus and styled with Tailwind CSS

# Development

## Pre-requisites
- [Rust](https://rustup.rs/)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.5/getting_started#dioxus-cli)
- [Node](https://nodejs.org/en) (optional: [nvm](https://github.com/nvm-sh/nvm) and [pnpm](https://pnpm.io/))
- [Tailwind CLI](https://tailwindcss.com/docs/installation)

```
# install dependency
pnpm install

# you will need 2 separate terminals
# rebuild css
pnpx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch

# rebuild website
dx serve --hot-reload
```
