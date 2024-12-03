use day_03::result_uncorrupt_file;

fn main() {
    let result = result_uncorrupt_file("src/resources/puzzle.txt");

    // 160672468
    println!("Multiply result in uncorrupted file: {result}");
}
