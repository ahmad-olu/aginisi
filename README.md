# 🔧 Aginisi — Fast JSON-Backed Mock API Server

**Aginisi** is a developer-first Rust application for quickly spinning up a local backend server with zero setup. Ideal for frontend developers and prototypers who want to:
- Mock out APIs fast.
- Store tables as editable `.json` files.
- Run CRUD and filter operations over HTTP.
- Choose your port and path.
- Preview real-time update support (coming soon with Socket.IO).

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

## 🧪 Example Usage
```http
POST /users
Content-Type: application/json

{
  "name": "Alice",
  "email": "alice@example.com"
}
```
### 📥 Create (POST)
### 📤 Read (GET)
### ✏️ Update (PUT)
### ❌ Delete (DELETE)



## 🗃️ JSON Table Format
Each .json file should contain an array of objects, for example:
```json
[
  { "id": 1, "name": "Alice" },
  { "id": 2, "name": "Bob" }
]
```


## 📡 Coming Soon
- 🔄 Real-time change broadcasting with Socket.IO
- 🔐 Authentication middleware
- 📊 Schema validation & mock data generators
- 📁 Swagger/OpenAPI generation

<!-- 🤝 Contributing
PRs and issues are welcome! File bugs or request features on the GitHub Issues page. -->
