# variables, basic types, functions, comments, and control flow

---

> by default, variables are immutable

---

# memory safety
## when trying to access an unbound array index
> This is an example of Rust’s memory safety principles in action. In many
low-level languages, this kind of check is not done, and when you provide an
incorrect index, invalid memory can be accessed. Rust protects you against this
kind of error by immediately exiting instead of allowing the memory access and
continuing.
