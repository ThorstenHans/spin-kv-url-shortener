# Spin key-value URL shortener

This repository contains a simple URL shortener that is build using Fermyon Spin. Data is persisted using built-in key-value store (SQLite) provided by Spin.

The app responds to three different kind of HTTP requests

- `POST /create` - for creating a new short-link
- `GET /<short>` - for opening a previously created short-link
- `GET /all` - for retrieving all known short-links

When you want to create a new short-link, you have to pass the original URL as JSON payload:

```json
{
    "url": "https://www.thorsten-hans.com"
}
```

For further details, consult my corresponding blog post ["Let Spin lift your key-value store"](https://www.thorsten-hans.com/let-spin-lift-your-key-value-store/)

## How to run the sample

To run this sample on your machine, you must have Rust and the `wasm32-wasi` installed (`rustup target add wasm32-wasi`). Additionally, you must have Spin CLI installed on your machine. You can find detailed installation instructions for Spin [here](https://developer.fermyon.com/spin/install).


