# 🔐 rs-locker

A simple experimental CLI tool written in Rust to lock and unlock directories using a password-based system. Ideal for basic personal data storage in a custom `vault-like` file format.

> ⚠️ **Warning:** This project is experimental and **not secure** for storing sensitive or important data at the moment.

---

## ✨ Features

* 📦 **Create a locked archive** from any directory
* 🔓 **Unlock and extract** the archive with the correct password
* 🗂️ Stores the entire folder structure and file contents
* 🧪 Uses a custom JSON-based data format internally

---

## 🛠️ How It Works

### Locking a directory

```bash
locker create <SRC_PATH> <PASSWORD>
```

* Reads the folder and all of its contents from `<SRC_PATH>`
* Serializes the structure and content into a custom data format (stored as JSON)
* Saves it to a `.locked` file

### Unlocking a directory

```bash
locker read <LOCKED_FILE_SRC_PATH> <PASSWORD>
```

* Parses the JSON file
* Validates the password
* Recreates the original directory structure and files if the password matches

---

## 🚧 Limitations

* **Not secure**: Currently uses plain JSON and simple password matching
* No encryption or obfuscation (yet)
* Meant for experimentation and hobby use only

---

## 🔮 Roadmap

* [ ] Add basic encoding/decoding using password
* [ ] Improve security for storage and retrieval
* [ ] Create a GUI version
* [ ] Add more CLI features (e.g., list contents, delete, rename)
* [ ] Consider encryption for secure storage

> If the project gets **10+ stars**, I plan to implement these improvements!

---

## 🤝 Contributing

Contributions are welcome!
If you're interested in improving the project or adding features, feel free to:

* Fork the repo
* Create a branch
* Submit a pull request

---

## 📦 Installation

You’ll need [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
cargo install --path .
```

Or clone and run directly:

```bash
git clone https://github.com/mahikoishor/rs-locker.git
cd rs-locker
cargo run -- create c:/my-folder mypassword
```

---

## 📄 License

This project is released under the MIT License.

---

## 💬 Feedback

Have ideas, suggestions, or found a bug?
Open an issue or start a discussion!

---

Would you like me to generate a logo or badge for this project?
