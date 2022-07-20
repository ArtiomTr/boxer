pub fn parse_configuration(source_path: &str) {
    println!("The path to config is: {}", source_path);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
