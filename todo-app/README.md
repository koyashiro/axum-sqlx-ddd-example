# todo-app

1. export environment variables `DATABASE_URL`

   ```sh
   export DATABASE_URL=postgres://postgres:password@localhost
   ```

2. run local servers

   ```sh
   docker compose up -d
   ```

3. create database and run migration (using [sqlx-cli](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli))

   ```sh
   sqlx create database
   sqlx migrate run
   ```

4. start server

   ```sh
   cargo run
   ```
