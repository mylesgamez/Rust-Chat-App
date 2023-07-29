# RustChatApp

RustChatApp is a simple multi-threaded chat server-client application implemented in Rust using TCP/IP sockets.

## Features
* Concurrent handling of multiple client connections using Rust threads.
* Reading, writing and echoing back messages to each client.
* Graceful handling of incoming and outgoing messages.

## Requirements
- Rust (latest stable version)

## Usage
Firstly, clone this repository and navigate into it:

```
git clone https://github.com/<your_username>/RustChatApp.git
cd RustChatApp
```

The application consists of two parts - the server and the client.

## Running the Server
Navigate to the chat_server directory and run the server using cargo:
```
cargo run
```

The server will start listening for incoming connections on localhost:8080.

## Running the Client
In a separate terminal window, navigate to the chat_client directory and run the server using cargo:
```
cargo run
```

The client will connect to the server and you can start typing messages. The client will echo back the messages from the server.

## License
MIT



