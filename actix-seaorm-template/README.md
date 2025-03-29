# Web template for Rust, Actix, and SeaORM

## Setup Instructions
### First Time Setup
- `cargo install sea-orm-cli`
- `sea-orm-cli migrate init`

### Subsequent Setup
- Set .env file variables for PORT, ADDRESS, and DATABASE_URL

## Running The Application
- `cargo run`

## Creating new database migrations
- `sea-orm-cli migrate create_a_new_table` (replace "create_a_new_table" with the name of your migration)
