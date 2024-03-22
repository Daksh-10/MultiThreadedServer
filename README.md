# MultiThreadedServer
## Installation
1. Clone the project
2. Install cargo
3. In the folder, open terminal and run the program
```cargo run```
## Introduction
This project creates a multi-threaded server with rust language, there are many methods to handle multiple requests from users and increase throughput like fork join model or single threaded asynchronous io model. This projects focuses on the concepts of communication between ports/sockets(TCP layer) and using threadpool and mutex(binary semaphore) to achieve the aim.
