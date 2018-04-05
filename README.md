# badcache

## Introduction

A demo project to learn a bit about [Rocket](https://rocket.rs), a web framework for Rust.

This horrible, horrible piece of code mimics a super minimal [memcached](https://www.memcached.org) with a very limited set of operations and few features.It's really just to learn something. Please, never actually use this.

## Running

After cloning the repository, build and run the code:

```
cargo build
cargo run
```

The service is now running at [http://localhost:8000](http://localhost:8000). All operations are invoked by GET requests to other URLs:

| Verb | Path               | Description                                     |
|------|--------------------|-------------------------------------------------|
| GET  | /add/_key_/_value_ | Adds the specified key-value pair               |
| GET  | /remove/_key_      | Removes the key-value pair at the specified key |
| GET  | /get/_key_         | Retrieves the value at _key_, or "null"         |

## License

Released under the [MIT License](http://rnelson.mit-license.org).
