### Testing some rust sql functionality


### Steps for populate_db and test_query_file:
1. Start PostgreSQL database via docker
``` bash
./utils/start_db.sh
```
2. Populate the database
``` bash
cargo run --bin populate_db
``` 
3. Set DATABASE_URL env variable
´´´ bash
export DATABASE_URL="postgres://postgres:localdb@localhost/postgres" 
´´´