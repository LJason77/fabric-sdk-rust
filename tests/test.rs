#[cfg(test)]
mod tests {
    use fabric::core::config::Config;

    #[test]
    fn it_works() {
        Config::from_file("aaa");
    }
}
