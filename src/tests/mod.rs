/*
a: 0b01100001
b: 0b01100010
c: 0b01100011
d: 0b01100100
e: 0b01100101
f: 0b01100110
g: 0b01100111
h: 0b01101000
i: 0b01101001
j: 0b01101010
k: 0b01101011
*/

#[cfg(test)]
#[test]
fn shrink_simple() {
    for b in "abcdefghijk".bytes() {
        println!("\"{:#10b}\"", b);
    }

    // 0b01100001 0b01100010 0b01100011
    let input = "abc";
    let output = super::shrink(input);
    let expected: [u8; 3] = [0b11100001, 0b11100010, 0b00000011];

    for (i, b) in output.iter().enumerate() {
        assert_eq!(*b, expected[i]);
    }
}

#[test]
fn shrink_7() {
    let output = super::shrink("abcdefg");
    let expected: [u8; 7] = [
        0b11100001, // b1, a7
        0b11100010, // c2, b6
        0b11000011, // d3, c5
        0b11000100, // e4, d4
        0b11001101, // f5, e3
        0b11001110, // g6, f2
        0b00000001, // 07, g1
    ];

    for (i, b) in output.iter().enumerate() {
        assert_eq!(*b, expected[i]);
    }
}

#[test]
fn shrink_8() {
    let output = super::shrink("abcdefgh");
    let expected: [u8; 7] = [
        0b11100001, // b1, a7
        0b11100010, // c2, b6
        0b11000011, // d3, c5
        0b11000100, // e4, d4
        0b11001101, // f5, e3
        0b11001110, // g6, f2
        0b11010001, // h7, g1
    ];

    for (i, b) in output.iter().enumerate() {
        assert_eq!(*b, expected[i]);
    }
}

#[test]
fn shrink_9() {
    let output = super::shrink("abcdefghi");
    let expected: [u8; 8] = [
        0b11100001, // b1, a7
        0b11100010, // c2, b6
        0b11000011, // d3, c5
        0b11000100, // e4, d4
        0b11001101, // f5, e3
        0b11001110, // g6, f2
        0b11010001, // h7, g1
        0b01101001, // 01, i7
    ];

    for (i, b) in output.iter().enumerate() {
        assert_eq!(*b, expected[i]);
    }
}

#[test]
#[should_panic]
fn shrink_panics() {
    super::shrink("søbæst");
}

#[test]
fn shrink_grow_various_strings() {
    let inputs = &[
        include_str!("./test_string.txt"),
        "",
        "0",
        "abcdefg",
        "abcdefgh",
        "abcdefghi",
        "012345678901234567890123",
        "{\"foo\": [42], \"bar\": true, \"moz\": \"hi\"}",
    ];

    for input in inputs {
        let compressed = super::shrink(input);
        let output = super::grow(&compressed).unwrap();
        assert_eq!(*input, output);
    }
}
