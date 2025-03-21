# axum-rs-api
Basic CRUD functionality in Rust+Axum, API design, and error handling.

## Installing Rust

### 1. Install Rust using rustup

Rust uses `rustup`, a toolchain manager, for installation.

**For Windows**

*   Download the Rust installer: [https://win.rustup.rs/](https://win.rustup.rs/)
*   Run the installer (`rustup-init.exe`) and follow the instructions.
*   Restart your terminal after installation.

**For macOS & Linux**

*   Run the following command in the terminal:

    ```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

*   Follow the on-screen instructions.

### 2. Verify Installation

After installation, check if Rust is installed correctly:

```

rustc --version

```

You should see output like:

```

rustc 1.x.x (yyyy-mm-dd)

```

## Setup & Run Project

*   Clone Repository 

    ```
    git clone https://github.com/jalindarht/axum-rs-api.git
    cd axum-rs-api
    ```

*   Build binaries
   
    ```
    cargo build
    ```

*   Create required tables in Redshift

    Open Redshift query editor and execute create table queries from 20250320_create_tasks_table.sql

*   Create .env file

    Get database connection details from Redshift.
    Refer .env.example to create .env file and update variable values

*   Run with 
  
    ```
    cargo run
    ```
*   Refer postman collection from AXUM-RS-API.postman_collection.json