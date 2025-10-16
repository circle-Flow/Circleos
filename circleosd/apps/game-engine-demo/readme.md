# 🎮 CircleOSD Game Engine Demo

A small text-based demo app showcasing how **CircleOSD** applications can be packaged and run.

This is a “game engine” stub that simply moves a green dot (`🟢`) around using **WASD** keys.
It demonstrates:
- How apps can run standalone inside CircleOSD’s app framework.
- How terminal rendering and input work through `crossterm`.

---

## 🧱 File Structure
apps/
└── game-engine-demo/
├── Cargo.toml
└── src/
└── main.rs

yaml


---

## 🚀 Run
```bash
cd apps/game-engine-demo
cargo run
Then control the green dot using:

Key	Action
W	Up
A	Left
S	Down
D	Right
Q	Quit

🧩 Integration with CircleOSD
In the future, this app can be managed as a plugin or registered system app, for example:

toml

[[service]]
name = "game-engine-demo"
cmd = ["./apps/game-engine-demo/target/release/game-engine-demo"]
restart = "Manual"
yaml


---

## ⚙️ `etc/` Configuration Layer

This directory holds all static configuration files for the system.
