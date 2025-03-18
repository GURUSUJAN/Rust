
# 📝 Rust To-Do App  
A simple, efficient **Command-Line To-Do List Application** written in **Rust**. This application allows users to **add, remove, update, and list tasks** using a **persistent JSON-based storage system**.  

---

## 🚀 Features  
✅ Add tasks with a title and description  
✅ List all tasks in a structured format  
✅ Mark tasks as completed or delete them  
✅ Persistent storage using **JSON**  
✅ Simple and lightweight CLI interface  

---

## 📌 Installation & Setup  
### 1️⃣ Clone the Repository  
```bash
git clone https://github.com/GURUSUJAN/Rust.git
cd Rust/todo
```

2️⃣ Build the Project
```bash
cargo build
```

3️⃣ Run the To-Do App
```bash
cargo run
```

---


💻 Usage

Here are some sample commands and their outputs:
```bash
➕ Add a Task
```

cargo run add "Learn Rust" "Complete Rust book by next week"

🟢 Output:
```bash
Task Added: "Learn Rust" - Complete Rust book by next week
```

📋 List Tasks
```bash
cargo run list
```

🟢 Output:
```bash
1️⃣ Learn Rust - Complete Rust book by next week [❌ Not Completed]
2️⃣ Build Rust CLI App - Work on To-Do CLI project [✅ Completed]
```

✅ Mark a Task as Completed
```bash
cargo run complete 1
```

🟢 Output:
```bash
Task "Learn Rust" marked as Completed ✅
```

🗑 Delete a Task
```bash
cargo run delete 2
```

🟢 Output:
```bash
Task "Build Rust CLI App" deleted successfully 🗑
```

---

🛠 Technologies Used
	•	Rust 🦀 (Main language)
	•	Serde (JSON serialization)
	•	Clap (Command-line argument parsing)
	•	File I/O (Persistent task storage)

---

📌 Contributing

Contributions are welcome! If you’d like to improve this project:
1.	Fork the repository
2.	Create a feature branch (git checkout -b feature-name)
3.	Commit changes (git commit -m "Added new feature")
4.	Push to your fork (git push origin feature-name)
5.	Open a Pull Request

---

🔗 Connect with Me

👤 Gurusujan Reddy Madem
🔗 [LinkedIn](https://www.linkedin.com/in/gurusujan/)

---

📜 License

📄 This project is MIT Licensed – feel free to use and modify it.

---

🌟 Star this repository if you found it useful! ⭐

---
