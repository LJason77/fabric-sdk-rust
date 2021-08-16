#[cfg(test)]
mod tests {
    use fabric::core::config::ConfigBackend;

    #[test]
    fn it_works() {
        ConfigBackend::from_file("aaa");
    }
}
