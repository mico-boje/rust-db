### Testing some rust sql functionality


### Steps:
1. Start PostgreSQL database via docker
``` bash
./utils/start_db.sh
```
2. Populate the database
``` bash
cargo run --bin populate_db
``` 
