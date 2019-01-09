## Before you run tests

Be sure all microservices are up and running.

## Run all tests

```
docker-compose run --rm functional-tests cargo test -- --test-threads=1
```

## Run single test

```
docker-compose run --rm functional-tests cargo test -- --test-threads=1 <test-name>
```

## Update schema

Example for local tests
Open link http://localhost:8000/
Copy `graphql/queries/introspection/introspection_query.graphql` in web page.
Response data copy from web page in file `graphql/schema.json`.