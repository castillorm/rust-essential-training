# 🦀 Rust Essential Training — Learning Repository

This repository documents my journey through the **Rust Essential Training** course by [Barron Stone](https://www.linkedin.com/learning/) on LinkedIn Learning.  
It includes **notes, examples, and mini-projects** that explore Rust’s core concepts — from ownership and lifetimes to error handling, traits, and concurrency.

---

## 🎯 Goals
- Build a solid foundation in the **Rust programming language**
- Develop **clean, safe, and efficient** Rust code
- Share progress and connect with the **Rust developer community**

---

## 🧠 Topics Covered
- Variables, Data Types, and Control Flow  
- Ownership, Borrowing, and Lifetimes  
- Structs, Enums, and Traits  
- Error Handling (`Result`, `Option`, `panic!`)  
- Collections and Iterators  
- Concurrency and Async Programming  
- Modules, Crates, and Cargo Workflows  

---

## 🗂️ Repository Structure
```

/notes         → Markdown summaries from each module
/examples      → Code snippets and playground demos
/projects      → Mini-projects and exercises

````

---

## 🚀 How to Use
1. Clone the repository:
   ```bash
   git clone https://github.com/<your-username>/rust-essential-training.git
   cd rust-essential-training

2. Build and run examples:

   ```bash
   cargo run
   ```
3. Explore the `/notes` folder for concept summaries.

---
Perfect 👍 — here’s how to properly set up your **Rust learning projects** so you can use `cargo run` smoothly for each exercise or module.

---

## 🦀 Step-by-Step: Convert Your Folder into a Cargo Project

### 1️⃣ Go to your repo root

From wherever you are, move up to the main repo folder:

```bash
cd ~/path/to/rust-essential-training
```

---

### 2️⃣ Create a new Cargo project

Use the `--bin` flag (for an executable project):

```bash
cargo new _1_write_your_first_program --bin
```

This creates a new folder structure:

```
_1_write_your_first_program/
├── Cargo.toml
└── src/
    └── main.rs
```

> 💡 Tip: Avoid using periods or spaces in folder names (like `1._Write_Your_First_Program`) — Cargo prefers underscores.

---

### 3️⃣ Copy your code

Move your existing `main.rs` content (if you already wrote code) into the new project’s `src/main.rs` file:

```bash
cp _1._Write_Your_First_Program/main.rs _1_write_your_first_program/src/main.rs
```

---

### 4️⃣ Run it!

Now, navigate into the new project folder and run your code:

```bash
cd _1_write_your_first_program
cargo run
```

You should see your program’s output in the terminal. 🎉

---

### 5️⃣ (Optional) Update your GitHub repo

To include your new project folder:

```bash
git add _1_write_your_first_program
git commit -m "Added first Rust program project"
git push
```

---

### 🗂 Recommended Project Layout for Your Repo

As you progress through the LinkedIn Learning course, create one Cargo project per module:

```
rust-essential-training/
├── LICENSE
├── README.md
├── _1_write_your_first_program/
├── _2_variables_and_types/
├── _3_ownership_and_borrowing/
└── _4_collections_and_traits/
```

Each folder can be a standalone Cargo project (`cargo new folder_name --bin`), making it easy to build and run examples independently.

---

## 🧑‍💻 About the Author

**Roderick M. Castillo**
[LinkedIn](https://www.linkedin.com/in/roderick-castillo/) | [GitHub](https://github.com/<your-username>)
Cybersecurity Engineer · AI Enthusiast · Lifelong Learner

---

## 🪪 License

This project is licensed under the [MIT License](LICENSE).
Feel free to use and share the content with proper attribution.

---

### 🌟 Acknowledgment

Special thanks to **Barron Stone** and **LinkedIn Learning** for creating high-quality educational content that makes learning Rust accessible and engaging.
