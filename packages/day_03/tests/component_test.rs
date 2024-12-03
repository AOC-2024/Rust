use day_03::result_uncorrupt_file;

#[test]
fn it_should_uncorrupt_file() {
    assert_eq!(result_uncorrupt_file("tests/resources/puzzle.txt"), 161);
}