## Before you run tests

Be shure all microservices are up and running.

## Run all tests

```
docker-compose run --rm functional-tests cargo test -- --test-threads=1
```

## Run single test

```
docker-compose run --rm functional-tests cargo test -- --test-threads=1 <test-name>
```
