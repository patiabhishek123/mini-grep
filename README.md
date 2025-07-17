🦀 Mini-Grep in Rust
A simple implementation of the classic grep command-line tool, written in Rust.
This project was built to practice file I/O, pattern matching, command-line parsing, and Rust’s unique ownership and error handling concepts.

📂 Project Overview
What is this?
mini-grep lets you search for lines containing a given query string within a text file — just like the Unix grep tool.

🚀 Features
✅ Search for a query string in a file
✅ Supports case-sensitive and case-insensitive searches
✅ Command-line arguments for query, filename, and optional case-insensitive flag
✅ Safe and robust error handling

🧩 How to Run
1️⃣ Clone the repo

bash
Copy
Edit
git clone https://github.com/your-username/mini-grep.git
cd mini-grep
2️⃣ Build the project

bash
Copy
Edit
cargo build --release
3️⃣ Run the program

bash
Copy
Edit
cargo run -- <query> <filename>
Example:

bash
Copy
Edit
cargo run -- to-do poem.txt
To run a case-insensitive search, set the IGNORE_CASE environment variable:

bash
Copy
Edit
IGNORE_CASE=1 cargo run -- to-do poem.txt
📚 What I Learned
✨ Reading and writing files in Rust
✨ Parsing command-line arguments
✨ Pattern matching & iterators
✨ Ownership and borrowing
✨ Writing clean, modular code
✨ Error handling with Result and unwrap_or_else

🗂️ Project Structure
bash
Copy
Edit
.
├── src
│   ├── main.rs   # Program entry point
│   ├── lib.rs    # Core search logic
├── poem.txt      # Example text file
├── Cargo.toml    # Project config
🛠️ Build & Test
To run tests:

bash
Copy
Edit
cargo test
🤝 Contributing
This project is a learning exercise, but PRs, suggestions, and improvements are always welcome!

✨ Acknowledgements
Inspired by The Rust Programming Language (a.k.a. The Book).

Happy coding! 🚀✨
Feel free to fork, experiment, and share your feedback!  
