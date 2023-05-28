# Rust with Data

A barebones and minimal implementation of a Create-Read API connected to a database through another container instance.

---

## Endpoints

- `/` : a basic "*hello*" GET endpoint.
- `/users/{name}`: a basic GET endpoint. See schema *User*.
- `/users` : a basic POST endpoint. See schema *User*.

## Schemas

```json
User {
    "name": "",  // string
    "age": 0,    // integer (u8)
}
```
