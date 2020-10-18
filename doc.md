# Clock-in/Clock-out Architecture
Document specifying how the frontend and backend of the application are architectured. Find the repo here: https://github.com/zachnicoll/clock-in-clock-out
## Backend
The backend source code is found in the `backend` folder in the root directory of the repo. The API server is written in Rust and utilises the Rocket web framework for handling requests [https://rocket.rs/], and Diesel ORM for querying the database [http://diesel.rs/].

The entry point for the API is found in `src/main.rs` and notated by `rocket::ignite()`. Each endpoint is 'mounted' to the server via certain routes defined by the `.mount("/api/routename", routes![route1, route2])` function calls, where `route1` (for example) is a function defining a route.

All route endpoints are defined within the `src/Routes` directory in their respective files. For instance, all endpoints related to users are defined inside `src/Routes/api_users.rs`. You can tell a function is an endpoint because it's notated by a HTTP request decorator, like:
```rust
#[post("/login", format = "json", data = "<credentials>")]
pub fn login(conn: DbConn, credentials: Json<PostUser>) -> ApiResponse {...}
```
If an enpoint requires a connection to the database, it needs to require a parameter with type `DbConn` as seen above. If the endpoint should be secured by requirement of JWT token, then include a parameter with type `JWT`. For example:
```rust
/*
    Route:      /api/users/<id>
    Method:     GET
    Info:       Gets a user based on <id> (Uuid) param
    Authorized: True
*/
#[get("/<id>")]
pub fn get_user(conn: DbConn, _jwt: JWT, id: UuidParam) -> ApiResponse
```

## Frontend
The frontend is built with React and is still super prototypey, so there isn't a very solid architecture or file structure yet.