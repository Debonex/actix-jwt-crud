# Actix JWT CRUD

A demo repo for JWT authorization and simple CRUD with `Actix-web@4`, `sea-orm`, `redis` and `mysql`.

## Preparation for run

1. Create a database in mysql, prepare an usable redis server.
2. Prepare `.env` file in project's root directory, set your database url and redis url, see `.env.example`.
3. Database migration, this will create tables in your database. `cargo run -p migration -- fresh`
4. Generate secret for encoding and decoding JWT. `head -c16 /dev/urandom > ./api/.secret` (You can also use the sample secret by rename `.secret.example` to `.secret`)
5. Run. `cargo run -p api`

## API

A postman workspace for quick test. [https://www.postman.com/martian-robot-631196/workspace/actix-jwt-crud](https://www.postman.com/martian-robot-631196/workspace/actix-jwt-crud)

#### `GET /`

Health checker

- Response: `200 OK`

```text
Hello, Actix
```

#### `POST /user/register`

User register

- Request body

```json
{
    "name": "user1",
    "password": "password"
}
```

- Response: `200 OK`

```text
1
```

#### `POST /user/login`

User login

- Request body

```json
{
    "name": "user1",
    "password": "password"
}
```

- Response: `200 OK`

```json
{
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxLCJpYXQiOjE2ODA3NzQxNDYsImV4cCI6MTY4MTM3ODk0Nn0.ivj_KEJSbOictAa6OGMXBYMrejWfZJeZJijclGev-WQ",
    "info": {
        "name": "user1",
        "count": 1
    }
}
```

#### `POST /user/logout`

User logout

- Request Header

Authorization: bearer \<token\>

- Response: `204 No Content`

#### `GET /user/info`

Get user info

- Request Header

Authorization: bearer \<token\>

- Response: `200 OK`

```json
{
    "name": "user1",
    "count": 1
}
```

#### `POST /user/add`

Add count of user by one.

- Request Header

Authorization: bearer \<token\>

- Response: `200 OK`
```json
{
    "name": "user1",
    "count": 2
}
```