# ApiResponse Library

## Overview

The `ApiResponse` library is designed to simplify the creation of HTTP responses for a Rocket-based RESTful API. It provides a set of helper functions to construct consistent and type-safe API responses with various HTTP status codes.

## Features

- Comprehensive support for HTTP status codes.
- Type-safe JSON responses for success and error cases.
- Easy integration with Rocket's `Responder` trait.
- Optional empty responses for status codes without a body.

## Installation

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
rocket = "0.5.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Usage

### Basic Setup

Define your API endpoints and use the provided helper functions to generate responses. Here’s an example:

```rust
use rocket::{get, launch, routes};
use rocket::serde::json::Json;
use serde::Serialize;

use rocket_response::{ok, not_found, ApiResponse};

#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
}

#[derive(Serialize)]
struct ErrMessage {
    error: String,
}

#[get("/hello")]
fn hello() -> ApiResponse<User, ErrMessage> {
    ok(User {
        name: "Alice".to_string(),
        age: 30,
    })
}

#[get("/not_found")]
fn not_found_example() -> ApiResponse<User, ErrMessage> {
    not_found(ErrMessage {
        error: "User not found".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, not_found_example])
}
```

## Helper Functions

The library provides a variety of helper functions corresponding to common HTTP status codes. Each function returns an `ApiResponse` enum:

### Success Responses

| Function Name     | HTTP Status Code | Description                      |
| ----------------- | ---------------- | -------------------------------- |
| `ok`              | 200              | Standard success response.       |
| `created`         | 201              | Resource successfully created.   |
| `accepted`        | 202              | Request accepted for processing. |
| `partial_content` | 206              | Partial resource returned.       |

### Empty Responses

| Function Name   | HTTP Status Code | Description                       |
| --------------- | ---------------- | --------------------------------- |
| `no_content`    | 204              | No content in the response body.  |
| `reset_content` | 205              | Request requires client to reset. |
| `not_modified`  | 304              | Resource not modified.            |

### Error Responses

| Function Name           | HTTP Status Code | Description                      |
| ----------------------- | ---------------- | -------------------------------- |
| `bad_request`           | 400              | Malformed or invalid request.    |
| `unauthorized`          | 401              | Authentication required.         |
| `forbidden`             | 403              | Request forbidden by the server. |
| `not_found`             | 404              | Resource not found.              |
| `internal_server_error` | 500              | Internal server error.           |

For a full list of helper functions, see the source code.

## ApiResponse Enum

The `ApiResponse` enum represents the different types of responses:

```rust
pub enum ApiResponse<T, E> {
    Ok(Status, Json<T>),
    Err(Status, Json<E>),
    Empty(Status),
}
```

## Customization

You can extend the library by adding your own helper functions for custom status codes or response types. For example:

```rust
/// 418 I'm a Teapot
pub fn teapot<T, E>(data: E) -> ApiResponse<T, E>
where
    T: Serialize,
{
    ApiResponse::Err(Status::ImATeapot, Json(data))
}
```

## License

This project is licensed under the [MIT License](LICENSE).
