# Setting up Tauri backend tests

If you have a Tauri function such as

```rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
```

then you can define Rust tests in the same file in the usual way with

```rust
#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn test_greet_name() {
        let result = greet("Test");
        assert_eq!(result, "Hello, Test! You've been greeted from Rust!");
    }
}
```

Note that you have to import `greet` from the parent module.
