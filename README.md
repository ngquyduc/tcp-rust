# CS3211 Assignment 3

### 0. Contributors

- Nguyen Quy Duc
- Tran Nhan Duc Anh

### 1. An outline and brief explanation of the TCP server

- **Framework**: The server is built on Tokio, using asynchronous Futures for handling client connections. This ensures efficient, non-blocking concurrency. About client connection handling, each client connection is managed asynchronously by the server, allowing for concurrent processing without blocking.
- **Assumptions:**

  - Result Size: All client results fit into a 64-bit unsigned integer.
  - Input Size: Initial and generated client seeds are within 64-bit unsigned integers.

- **Key Details:**

  - Concurrency Management: Asynchronous processing enables simultaneous handling of multiple clients.
  - Resource Management: Techniques like semaphore-based throttling prevent resource exhaustion by limiting the number of CPU intensive task to be maximum of 40

### 2. The concurrency paradigm and how the clients’ requests are being handled concurrently

- **Concurrency Paradigm:** The primary concurrency paradigm in our implementation is Tokio. Tokio is a asynchronous runtime for Rust designed for handling IO-heavy operations concurrently without blocking. It offers efficient non-blocking asynchronous execution, making it suitable for building high-performance servers.
- **Handling Clients' Requests Concurrently:** In our implementation, Tokio is utilized to handle each client's connection concurrently. When a new TCP connection is established, a new asynchronous connection handler is spawned in the Tokio runtime. This means that the server can handle multiple client connections simultaneously without blocking. The `await` keyword is used to poll asynchronous operations, allowing the server to efficiently manage multiple client requests without blocking.
- **CPU-Intensive Task Management:** To manage CPU-intensive tasks and prevent resource exhaustion, a counting semaphore is employed. The semaphore limits the total number of CPU-intensive tasks that can be executed concurrently (40). Each time a CPU-intensive task is allocated and awaited, the semaphore is acquired, ensuring that the server does not exceed the specified limit for concurrent CPU-bound operations.

### 3.1. Level of concurrency

- **Task level concurrency.**
- Each client connection is processed asynchronously, enabling the server to handle multiple client requests simultaneously without blocking. This asynchronous approach ensures that the server can efficiently manage incoming connections without waiting for each individual client to complete its request.
- Furthermore, both I/O and CPU-intensive tasks are executed asynchronously within the Tokio runtime, enabling concurrent execution of these tasks. This design maximizes the server's efficiency by utilizing available system resources effectively.

### 3.2. All cases when the concurrency level decreases

- **CPU-intensive task saturation:** If all CPU-intensive tasks are executed concurrently without any throttling mechanism, the concurrency level decreases. This happens because CPU-intensive tasks within Tokio do not inherently yield back resources for other tasks to execute. Consequently, if all CPU-intensive tasks are utilizing the available processing capacity, no additional tasks can be processed concurrently. This situation leads to a decrease in concurrency as the server becomes bottleneck by the maximum capacity of CPU-bound tasks it can handle simultaneously.
- **Resource limitations:** If system resources such as memory or network bandwidth become fully utilized, the concurrency level decreases. In such cases, the server may not be able to handle additional tasks concurrently due to resource constraints. For example, if the server exhausts available memory or network bandwidth, it may slow down or even halt processing new tasks until resources become available again. This also results in decreased concurrency as the server's ability to handle multiple tasks simultaneously is limited by resource availability.

### 4. Server running in parallel

- **Yes, the server can run tasks in parallel**. It leverages asynchronous processing within the Tokio runtime, enabling multiple tasks to be executed concurrently. This approach ensures that the server can efficiently handle client connections while simultaneously processing tasks without blocking. By utilizing asynchronous execution, the server optimizes resource utilization and responsiveness, enabling effective parallel execution of tasks.

### 5. If you have tried multiple implementations, explain the differences and your evolution to the current submission.

- In our initial exploration, we experimented with **Rayon's thread pool** for handling concurrent tasks. However, we encountered difficulties integrating it with the given server architecture (the code couldn't compile). Balancing thread pools alongside asynchronous operations is somewhat challenging and may introduce potential overhead.

- To simplify our implementation, we turned to **Tokio**. Tokio's asynchronous runtime, well-documented in lectures and have a lot of online tutorials, offered a more straightforward solution for our server. It provided a natural fit for handling client connections concurrently without the need for explicit thread pool management from developers. This change allowed us to achieve efficient concurrency while benefiting from the ease of implementation and clarity offered by Tokio's approach.
