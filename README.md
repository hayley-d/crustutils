# 🦀 crustutils
```
 __________________________
< Hello fellow Rustaceans! >
 --------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
```

**crustutils** = core + Rust = **crust** + utils. 
</br>
</br>
This project is a Rust library reimplementing essential Unix coreutils.</br>
Why stick with old GNU tools when you can have:  
- **fearless concurrency**
- **memory safety**
- **zero-cost abstractions**  
- **a cute crab mascot**

---

### **How It’s Built**

Instead of a collection of separate binaries, `crustutils` is a **single Rust crate**  
that exposes all utilities under one namespace.

You can:
-  Import it as a library (`use crustutils::ls;`) and call utils from other Rust projects.
-  Use the CLI if you want direct terminal commands. 

Want a system toolbox fully written in Rust? This is it.

---

### **Coreutils to Implement**

| Utility  | Description                    | Status         |
|----------|--------------------------------|----------------|
| `ls`     | List directory contents        | 🚧 In progress |
| `cat`    | Concatenate + print files      | 🕑 Planned     |
| `rm`     | Remove files/directories       | 🕑 Planned     |
| `touch`  | Create/update files            | 🕑 Planned     |
| `mkdir`  | Make directories               | 🕑 Planned     |
| `cp`     | Copy files/directories         | 🕑 Planned     |
| `mv`     | Move/rename files              | 🕑 Planned     |
| `echo`   | Print arguments to stdout      | 🕑 Planned     |
| `pwd`    | Print working directory        | 🕑 Planned     |
| `md5sum` | Generate the MD5 hash          | 🕑 Planned     |

---

### **How to Run**

1. Clone the repo:
```bash
git clone https://github.com/yourusername/crustutils.git
cd crustutils
```

2. Build the library
```bash
cargo build
```

3. Run examples and tests
```bash
cargo test
```
---

### Why Am I Doing This?
Because:
- Rust.
- Learning system-level programming is cool.
- Everything needs a rust rewrite.
- Ferris looks cooler than a penguin.

---

