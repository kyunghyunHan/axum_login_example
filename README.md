# axum_login_example
- In this example, axum, diesel, and postgresSQL are used to log in and sign up.

## Axum

## postgresSQL

## Diesel

```
rustup update stable.

```

```toml
diesel = { version = "2.1.0", features = ["postgres"] }
dotenvy = "0.15"
```

```
cargo install diesel_cli --no-default-features --features postgres

```

```
DATABASE_URL=postgres://username:password@localhost/diesel_demo

```

```
diesel setup

```

```
diesel migration generate "migration name"

```
- up.sql
```sql
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
)
```


- down.sql
```sql
DROP TABLE posts

```

```
diesel migration run
```