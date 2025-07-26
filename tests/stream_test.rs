use rxx::Stream;

#[test]
pub fn should_map() {
    let value = Stream::<&str, String>::new(|input| input.to_string())
        .map(|input| input.as_bytes().to_vec())
        .map(|input| input.len());

    assert_eq!(value.run(&"testing 123"), 11);
}
