uniffi::setup_scaffolding!();

#[uniffi::export]
fn say_hi() -> String {
    "Hello from Rust on Android!".to_string()
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
