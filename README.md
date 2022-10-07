# Ethereum transactions viewer app

This repository contains an example full-stack Rust application. Where both the backend and frontend are written in pure Rust code. This project aims to show how a full-stack Rust application can be built and also the advantage of doing so. Namely that data structures code can be shared between all the components.The project also shows how to retrieve data from the Etherscan API.

The project consists of the following components:

- The backend uses the warp crate to create a REST API.
- The frontend uses the Yew framework.
- There is also a common libary component that defines the data structures.

The tutorial explaining how the project is built can be found on my blog: [Ethereum transactions viewer app: Rust how to](https://tms-dev-blog.com/ethereum-transaction-viewer-rust-app) soon.

## Building and Running the project

First, to fully use this project an API Key is needed. The tutorial on my website explains where to get it and how to use it with the project. To build and run the project cargo and [trunk](https://trunkrs.dev/) are required.

To run the backend run the following command from the `backend` directory:

`cargo run`

To run the frontend run the following command from the `frontend` directory:

`trunk serve`
