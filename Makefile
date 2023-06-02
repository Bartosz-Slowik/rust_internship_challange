
create-db:
	psql -c "CREATE DATABASE rust_challenge;"
	psql -c "CREATE USER demo;"
	psql -c "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";"
