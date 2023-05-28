# Rust with Data

A barebones and minimal implementation of a Create-Read API connected to a database through another container instance.

---

## Endpoints

- `/` : a basic "*hello*" GET endpoint.
- `/users/{name}`: a basic GET endpoint. See schema *User*.
- `/users` : a basic POST endpoint that creates a new User. See schema *User*.

## Schemas

```ron
User {
    name: "",  // string
    age: 0,    // integer
}
```

## How To Use

1. Clone this repository locally using `git clone`.
2. Adjust the variables inside of `docker-compose.yml` as needed.
3. Run `docker-compose up`.
4. Interact with the endpoints either with a browser or with `curl`.
5. Read the code and modify accordingly as a learning experience.

## Feature Suggestions

If you would like to add your own twists to the code from your local machine, here are a few feature suggestions you could try implementing:

- Finish all CRUD endpoints.
  - This API is still missing the Update and Delete functionalities of a usual CRUD API. Implement them at your own pace and ensure that they still work during `docker-compose`.
- Use a more sophisticated schema.
- Add security features to the backend, such as Basic Auth and hidden environment variables.
- Optimize for performance after each implementation if necessary.
- Add a frontend of your own design!
