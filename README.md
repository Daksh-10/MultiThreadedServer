# MultiThreadedServer
## To run the project use the following command in terminal(cargo should be installed before running)
1. cargo run

This project creates a multi-threaded server with rust language, there are many methods to handle multiple requests from users and increase throughput like fork join model or single threaded asynchronous io model. This projects focuses on the concepts of communication between ports/sockets(TCP layer) and using threadpool and mutex(binary semaphore) to achieve the aim.
