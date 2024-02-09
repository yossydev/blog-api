## Blog API

This project provides a RESTful API for managing blog articles, intended for use on https://yossy.dev/.

### Features

- Like functionality

### Technology Stack

- Rust: Programming language
- actix-web : Rust web framework
- Diesel: Rust ORM (Object-Relational Mapping) library
- SQLite: Database

## Installation and Usage

1. Clone the repository:

```
$ git clone git@github.com:yossydev/blog-api.git
```

2. Navigate to the project directory:

```
$ cd blog-api
```

3. Run the application

```
$ make up
```

4. Api request

```
$ curl -X POST -H "Content-Type: application/json" -d '{"increment": 30}' http://localhost:8080/like/${slug}
```

## API Endpoints

- /likes: Like functionality

## License

This project is licensed under the MIT License. See the LICENSE file for details.
