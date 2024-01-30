# Learn Actix

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
