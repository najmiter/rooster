# Rust Axum Web Service

A simple web service built with Rust and the Axum framework.

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Cargo (Rust's package manager, installed with Rust)

## Getting Started

### Installation

1.  **Clone the repository (if applicable):**

    ```bash
    git clone https://github.com/najmiter/rooster
    cd rust-app
    ```

2.  **Build the project:**
    ```bash
    cargo build
    ```

### Running the Application

1.  **Run the development server:**
    ```bash
    cargo run
    ```
    The application will start on `http://127.0.0.1:3000`.

## API Endpoints

### `GET /`

- **Description:** Displays a welcome message and basic API usage information.
- **Response:** HTML content.

### `GET /users/:id`

- **Description:** Retrieves data for a specific user. This endpoint requires authentication.
- **Path Parameters:**
  - `id` (string, required): The username of the user.
- **Query Parameters:**
  - `page` (integer, optional, default: `1`): The page number for pagination.
  - `per_page` (integer, optional, default: `30`): The number of items per page.
- **Authentication:**
  - The username in the path (`:id`) is used for authentication. Currently, only the username `najmiter` is authorized. because yes.
- **Responses:**
  - `200 OK`:
    ```json
    {
      "user_id": "najmiter",
      "page": 1,
      "per_page": 30,
      "items": []
    }
    ```
  - `401 Unauthorized`: If the user ID is not `najmiter`.
    ```json
    {
      "error": "Unauthorized access"
    }
    ```
  - `404 Not Found`: If the user ID is empty in the path (though this scenario is handled before authentication logic in the current setup).
    ```json
    {
      "error": "User not found"
    }
    ```

## Authentication

Authentication is handled via a middleware that checks the `id` path parameter.

- If the `id` is `najmiter`, the request is allowed.
- Otherwise, a `401 Unauthorized` error is returned.

## Technologies Used

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Tokio](https://tokio.rs/) - Asynchronous runtime
- [Serde](https://serde.rs/) - Serialization/deserialization framework
- [anyhow](https://github.com/dtolnay/anyhow) - Flexible error handling
