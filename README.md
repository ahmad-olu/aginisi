# 🔧 Aginisi — Fast JSON-Backed Mock API Server

## overview
**Aginisi** is a developer-first Rust application for quickly spinning up a local backend server with zero setup. Ideal for frontend developers and prototypers who want to:
- Mock out APIs fast.
- Store tables as editable `.json` files.
- Run CRUD and filter operations over HTTP.
- Choose your port and path.
- Preview real-time update support (coming soon with Socket.IO).

---

## 🔧 Motivation
As a frontend developer who also builds backends—often using different languages for each—I’ve repeatedly encountered a common challenge: after building the backend, I find that integrating it with my frontend can be frustrating because the data shape or API responses weren’t fully anticipated during the frontend development. This mismatch slows me down, especially when rapidly prototyping MVPs or testing new product ideas.

To solve this, I created this tool to let me design and prototype REST APIs around the needs of my frontend first. It allows me to simulate backend behavior with flexible JSON-based data storage, CRUD operations, and filtering—so I can build and test my UI against realistic endpoints from day one. Once the frontend is solid, I can then implement the real backend with a clear understanding of the data structure and interactions needed.

This approach bridges the gap between frontend and backend during prototyping and speeds up development of MVPs, internal tools, and early-stage features.

---

## 🚀 Features

- 🧾 JSON-based table storage (stored in your root project directory)
- 🌐 Launches a RESTful HTTP server on your desired port
- ⚙️ Supports CRUD operations: Create, Read, Update, Delete
- 🔍 Simple filtering support via query params
- 🛠️ Easily extendable via modular architecture
- 🔌 Real-time data sync planned (via Socket.IO)

---

## 📂 Project Structure

```
├── aginisi/
│ ├── users.json
│ └── products.json
├── aginisi.toml # optional config
```

## TO Run

```bash
aginisi serve --port 5000
```
This will start a server on http://localhost:5000 and look for or create a aginisi/ folder with .json files (like users.json, posts.json, etc.).

---

## 🧪 Example Usage

### 📥 Create (POST)

```http
POST /users
Content-Type: application/json

{
  "data":{
    "name":"alex",
    "email":"alex@gmail.com"
  }
}
```

### 📤 Read (GET)
```http
GET /users
Content-Type: application/json

{}
```
### ✏️ Update (PATCH)
```http
PATCH /users/3
Content-Type: application/json

{
  "data":{
    "email":"alex1@gmail.com"
  }
}
```
### ❌ Delete (DELETE)
```http
DELETE /users/3
Content-Type: application/json

{}
```

## Auth

### 📥 Sign Up (POST)

```http
POST /auth/sign_up
content-type: application/x-www-form-urlencoded

{
    "name": "myNameIsMyName"
    "email": "email@email.com",
    "password": "verySecuredPassword"
}
```

### 📥 Sign In (POST)

```http
POST /auth/sign_in
content-type: application/x-www-form-urlencoded

{
    "email": "email@email.com",
    "password": "verySecuredPassword"
}
```

## File

### Upload
```http
POST /file/upload
content-type: multipart/form-data

{
    "key": "file",
    "value": "your image"
}
```

### Download
```http
GET /file/files/:image_name
content-type: application/octet-stream

{}
```

### 📤 Read (GET)(AUTH)(JWT)
```http
GET /users
Content-Type: application/json
Authorization: Bearer ...

{
  "filter": {
    "type": "Equals",
    "key": "email",
    "value": "jonah@gmail.com"
  }
}

```

### 📤 Read (GET)(AUTH)(JWT)
```http
GET /users
Content-Type: application/json
x-session: (Session id)


{
  "filter": {
    "type": "Or",
    "left": {
      "type": "Equals",
      "key": "id",
      "value": 1
    },
    "right": {
      "type": "Equals",
      "key": "id",
      "value": 3
    }
  }
}
```

## for configuration `aginisi_config.toml`

```toml
[overview]
name = "Aginisi"
version = 1

[config]
port = 3000
auth = "jwt" #"jwt" or "session" or con be empty for no auth

```

---

## 🗃️ JSON Table Format
Each .json file should contain an array of objects, for example:
```json
[
  { "id": 1, "name": "Alice" },
  { "id": 2, "name": "Bob" }
]
```
---

## stream with socket io

subscribe to either url or url/socket e.g (http://127.0.0.1:8090/) or (http://127.0.0.1:8090/socket)

1. broadcast after post request
    when you create a `POST` request every one subscribed to the `[path]-listener` where path is the table. for example to listen to changes on posts. subscribe to `posts-listener`. would see the result of response of the post request

2. specific stream with created table
    when you send a event to table name / path e.g posts table = [posts]. every one subscribed to [to-[table name]] e.g [to-posts] were posts is a table under aginisi.

---
## 📡 Coming Soon
- websocket
- 🔐 Authentication middleware
- 📊 Schema validation & mock data generators
- 📁 Swagger/OpenAPI generation

---
<!-- 🤝 Contributing
PRs and issues are welcome! File bugs or request features on the GitHub Issues page. -->
