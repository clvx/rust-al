pub async fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    // This test will not compile because `add` is an async function
    #[test]
    fn it_does_not_compile() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    */

    // This test will compile and run
    #[tokio::test]
    async fn it_works() {
        let result = add(2, 2).await;
        assert_eq!(result, 4);
    }

    // This test will compile and run because we are using tokio runtime
    #[test]
    fn will_compile() {
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(add(3, 2));
        assert_eq!(result, 5);
    }
}
