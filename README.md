# Learn Rust, Rust is awesome.

Pure Actix BE API for CRUD

## local server
start up local server:
```
cargo run
```

## curl command
curl full list:
```
curl http://localhost:8080/
```

curl items for a todo:
```
curl http://localhost:8080/todos/1/items 
```

create an item:
```
curl -X POST -H "Content-Type: application/json" -d '{"title": "List 3" }' http://localhost:8080/todos

curl http://localhost:8080/todos
```

check an item
```
 curl -X PUT http://localhost:8080/todos/1/items/2 -s
```
## database setup
for data base:
make sure docker is run on mac, spin up docker first:
```
docker-compose up -d
```

Then create a database:
```
psql -h 127.0.0.1 -p 5432 -U actix actix < database.sql
```
Query relations:
```
todo % psql -h 127.0.0.1 -p 5432 -U actix actix  
```

restart postgresql:
```
 brew services restart postgresql
```

stop postgresql:
```
 brew services stop postgresql
```
