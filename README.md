# egui-bind
Library for storing and showing key and poitner binds

# Example
```rust
// Foreword: You can find this example in `examples/bind.rs`

// Order matters, If you were to put this if case
// after the bind was shown, then it would trigger `self.cout += 1`
// on the same frame user assigned a new bind, which may not be the 
// desired behavior. But you can mitigate this by using the return
// value of a `Bind::show` as shown below with `println!`.
if self.bind.pressed(ui.input()) {
    self.count += 1;
}

let assigned = Bind::new("_test", &mut self.bind).show(ui);

// Here it checks if the bind was pressed but not assigned on the same frame.
if !assigned && self.bind.pressed(ui.input()) {
    println!("I was pressed");
}
```
