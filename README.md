# MultiThreadedServer
## Installation
```
clone the project
install cargo
cargo run
```
## Introduction
This project creates a multi-threaded server with rust language, there are many methods to handle multiple requests from users and increase throughput like fork join model or single threaded asynchronous io model. This projects focuses on the concepts of communication between ports/sockets(TCP layer) and using threadpool and mutex(binary semaphore) to achieve the aim.
