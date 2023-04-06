# CalDavFast

### What is CalDav? (By ChatGPT)

CalDav is a protocol designed to allow users to access and manage calendar data on a remote server. It is an extension of the WebDAV (Web Distributed Authoring and Versioning) protocol, which allows for the creation and modification of web-based resources, including calendars.

CalDav enables users to retrieve calendar data, schedule meetings, and set reminders from their personal calendars on different devices, such as smartphones, laptops, or tablets. It uses the iCalendar format, a standard for exchanging calendar and scheduling information, to represent calendar data.

CalDav is supported by various calendar applications, including Apple iCal, Mozilla Thunderbird, and Microsoft Outlook, as well as open-source software, such as DAViCal and Baikal. It is also compatible with Google Calendar, which allows users to synchronize their Google calendar with third-party CalDav clients.

Overall, CalDav simplifies the process of accessing and sharing calendar data across multiple devices and applications.

- RFC 4918 - HTTP Extensions for Web Distributed Authoring and Versioning (WebDAV): This RFC defines the WebDAV protocol, which is an extension to the HTTP/1.1 protocol that allows clients to perform remote web content authoring operations. WebDAV enables clients to create, modify, and delete content on a remote web server, as well as lock and unlock resources to prevent editing conflicts. It also includes support for versioning, collections, and properties, making it a useful protocol for distributed authoring and versioning of web resources.

- RFC 4791 - CalDAV: An Extension to WebDAV for Distributed Collaboration: This RFC defines the CalDAV protocol, which extends the WebDAV protocol to enable calendaring and scheduling functionalities.

- RFC 5545 - iCalendar: This RFC defines the iCalendar format, which is used for exchanging calendar and scheduling information.

### Why Rust? (By ChatGPT)

Rust is a programming language that has gained popularity in recent years due to its focus on performance, safety, and reliability. These characteristics make it well-suited for developing network applications, such as CalDav servers.

First, Rust's memory safety features, such as its ownership and borrowing system, prevent common memory-related errors such as null pointer dereferencing and buffer overflows, which are common sources of security vulnerabilities. This makes Rust a good choice for developing secure network applications like CalDav servers.

Second, Rust's concurrency model and lightweight threading support allow for efficient handling of multiple client connections, making it well-suited for developing scalable network applications.

Third, Rust has a low-level control over system resources, allowing for fine-grained optimization of performance. This is especially important for network applications that need to handle large amounts of data and high levels of concurrency.

Finally, Rust has a growing ecosystem of libraries and tools, including the hyper HTTP library, which provides a high-performance foundation for developing web applications like CalDav servers.

Overall, Rust's performance, safety, and reliability features make it an attractive choice for developing high-performance network applications like CalDav servers.


### Why MongoDB? (By ChatGPT)

MongoDB is a NoSQL database that is particularly well-suited for applications that require high scalability, flexibility, and availability. One of the key advantages of MongoDB is its ability to handle large volumes of unstructured data. This makes it an ideal choice for a CalDAV server, which typically involves storing and managing large amounts of calendar data.

Here are a few reasons why MongoDB is a good choice for a CalDAV server:

- Flexibility: MongoDB's document-oriented data model allows for flexible and dynamic data structures that can easily handle changes in the schema. This makes it easy to store complex calendar data, such as recurring events, exceptions, and attachments.

- Scalability: MongoDB is designed to scale horizontally across multiple servers, which means that it can handle large amounts of data and traffic. This is particularly important for a CalDAV server, which needs to handle a large number of concurrent users.

- Availability: MongoDB is designed to be highly available, with built-in replication and automatic failover. This ensures that your CalDAV server is always up and running, even in the event of a hardware failure or network outage.

- Performance: MongoDB's indexing and query optimization features allow for fast and efficient data access, which is important for a CalDAV server that needs to serve up calendar data in real-time.

Overall, MongoDB is a powerful and flexible database that is well-suited for the demands of a CalDAV server. Its ability to handle large amounts of unstructured data, scale horizontally, and provide high availability and performance make it an ideal choice for this type of application.

## Development

- Install Rust https://www.rust-lang.org/learn/get-started
- `cp .env.example .env` and change the values if needed.
- Install pre-commit hook:
  1. `pip install pre-commit`
  2. `pre-commit install`
- Run `docker compose up` to start the dependencies like database. 
(Can use `docker compose start` after the first time).
- Run `cargo run` to start the server.
- Run `cargo-watch -x run` to start the server in watch mode.


- Run `cargo test` to run the tests.
- Run `cargo fmt` to format the code.
- Run `cargo clippy` to check for linting errors.
- Run `cargo doc --open` to generate the documentation.
