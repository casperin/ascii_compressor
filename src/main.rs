mod compress;
mod decompress;

fn main() {
    // Just ghetto benching the code
    let input = include_str!("./tests/test_string.txt");
    for _ in 0..1_000_000 {
        let small = compress::shrink(&input);
        let _output = decompress::grow(&small);
    }
}
