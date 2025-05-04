pub fn show_message(text: &str) {
    println!("DISPLAY: {} ", text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_message() {
        show_message("Test Message");
    }
}
