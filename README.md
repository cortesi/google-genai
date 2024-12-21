# genai-rs

A Rust client for the Google Generative Language API (PaLM2/Gemini).

## Features

- Stream and non-stream content generation
- Full type safety with Rust datatypes
- Simple async API

See the `examples` directory for usage examples.

This crate currently supports just enough fo the API to be used in
[tenx](https://github.com/cortesi/tenx).

The datatypes in this crate is a transliteration of the [Google Go
SDK](https://github.com/googleapis/go-genai/) into Rust datatypes. 
