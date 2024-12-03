use day_03::uncorrupt_file;

#[test]
fn it_should_uncorrupt_file() {
    assert_eq!(uncorrupt_file("tests/resources/puzzle.txt"), 161);
}