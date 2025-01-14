# rust-graphql-xml

A simple Rust application demonstrating:

1. A **GraphQL** API (using [async-graphql](https://github.com/async-graphql/async-graphql) and [warp](https://github.com/seanmonstar/warp)).  
2. An **XML** endpoint (using [Yaserde](https://docs.rs/yaserde/latest/yaserde/) for serialization/deserialization).

Both the GraphQL and XML endpoints read from and write to the same `data.xml` file, ensuring consistency between the two interfaces.

## Features

- **GraphQL**:  
  - Query and Mutation to read/write data stored in `data.xml`.  
  - In-browser Playground at `/playground`.  
- **XML**:  
  - `GET /xml` returns the contents of `data.xml` as XML.  
  - `POST /xml` consumes XML payload, updates `data.xml`, and responds with newly written data as XML.  
- **Shared Structs**:  
  - `MyInputData` (`#[derive(InputObject)]`) for incoming GraphQL input and for consuming XML.  
  - `MyOutputData` (`#[derive(SimpleObject)]`) for GraphQL response and for serving XML.  

## Project Structure

```bash
.
├── Cargo.toml
├── src
│   ├── lib.rs            # Shared data types + read/write helpers
│   ├── graphql_schema.rs # GraphQL Query & Mutation + route builders
│   ├── xml_schema.rs     # XML GET/POST endpoints + route builders
│   └── main.rs           # Main entry point (combines all routes)
└── data.xml              # Created or updated at runtime
```

1. **lib.rs**  
   - Contains our shared structs (`MyInputData` and `MyOutputData`) and helpers to read/write `data.xml`.  
2. **graphql_schema.rs**  
   - Defines `QueryRoot`, `MutationRoot`, and sets up a Warp route for `/graphql` and `/playground`.  
3. **xml_schema.rs**  
   - Defines Warp routes for `GET /xml` and `POST /xml`, handling XML with Yaserde.  
4. **main.rs**  
   - Ties everything together, building the GraphQL schema and combining the Warp filters for both GraphQL and XML endpoints.

## Prerequisites

- **Rust** (stable) installed. To check, run `rustc --version`.
- **Cargo** (comes with Rust) to build and run.

## Getting Started

1. **Clone the repository**:

   ```bash
   git clone https://github.com/<your-username>/rust-graphql-xml.git
   cd rust-graphql-xml
   ```

2. **Build and run**:

   ```bash
   cargo run
   ```

   The server will start on `127.0.0.1:8000`.

3. **Explore the endpoints**:
   - **GraphQL Playground**:  
     - [http://localhost:8000/playground](http://localhost:8000/playground)  
     - A web UI to run GraphQL queries and mutations.  
   - **GraphQL Endpoint**:  
     - [http://localhost:8000/graphql](http://localhost:8000/graphql)  
     - Accepts GraphQL queries/mutations over `POST` (and `GET` with query strings).  
   - **XML Endpoint**:  
     - `POST /xml` (with `Content-Type: application/xml`) to create/overwrite data in `data.xml`.  
     - `GET /xml` to retrieve the current contents of `data.xml` as XML.

## Usage Examples

### 1. GraphQL

- **Mutation**: Write data to `data.xml`
  ```graphql
  mutation {
    writeData(data: { id: 1, name: "Alice" })
  }
  ```

- **Query**: Read data from `data.xml`
  ```graphql
  query {
    readData {
      id
      name
    }
  }
  ```

### 2. XML

- **POST**:  
  ```bash
  curl -X POST \
       -H "Content-Type: application/xml" \
       -d '<?xml version="1.0"?>
            <MyData>
              <id>99</id>
              <name>Bananas</name>
            </MyData>' \
       http://localhost:8000/xml
  ```
  - Creates/updates `data.xml` and returns XML for `<MyData><id>99</id><name>Bananas</name></MyData>`.

- **GET**:
  ```bash
  curl http://localhost:8000/xml
  ```
  - Outputs the current `data.xml` content as XML.
