use std::string::FromUtf8Error;

fn main() {
    let s = include_str!("./test_string.txt");
    for _ in 0..1_000_000 {
        let shrunk_string = shrink(s).unwrap();
        let _ = grow(shrunk_string);
    }
}

fn shrink(s: &str) -> Result<Vec<u8>, String> {
    let bytes: Vec<u8> = s.bytes().collect();
    let mut out = Vec::new();

    for i in 0..bytes.len() {
        let byte1 = bytes[i];
        if !byte1.is_ascii() {
            return Err("Please only use with ascii strings".to_string());
        }
        if i % 8 == 7 {
            continue;
        }
        let byte2 = if i == bytes.len() - 1 {
            0b00000000
        } else {
            bytes[i + 1]
        };
        let shrunk_byte = shrink_2_bytes(i % 8, byte1, byte2);
        out.push(shrunk_byte);
    }

    Ok(out)
}

fn shrink_2_bytes(index: usize, byte1: u8, byte2: u8) -> u8 {
    if index > 6 {
        panic!("Index should not be above 6");
    }
    let left_shift = (index + 1) as u8;
    let right_shift = 6 - index;
    let compare = [
        0b01000000, 0b01100000, 0b01110000, 0b01111000, 0b01111100, 0b01111110, 0b01111111,
    ];
    (byte1 << left_shift) + ((byte2 & compare[index]) >> right_shift)
}

fn grow(bytes: Vec<u8>) -> Result<String, FromUtf8Error> {
    let mut out = Vec::new();

    for chunk in bytes.chunks(7) {
        let grown_byte = grow_2_bytes(0, 0b10000000, chunk[0]);
        out.push(grown_byte);

        for i in 0..chunk.len() - 1 {
            let byte1 = chunk[i];
            let byte2 = chunk[i + 1];
            let grown_byte = grow_2_bytes(i + 1, byte1, byte2);
            out.push(grown_byte);
        }

        if chunk.len() == 7 {
            let grown_byte = grow_2_bytes(7, chunk[6], 0b0);
            out.push(grown_byte);
        }
    }

    String::from_utf8(out)
}

fn grow_2_bytes(index: usize, byte1: u8, byte2: u8) -> u8 {
    if index > 7 {
        panic!("Index should not be above 7");
    }
    let mappers = [
        (0b00000000, 0b11111110),
        (0b00000001, 0b11111100),
        (0b00000011, 0b11111000),
        (0b00000111, 0b11110000),
        (0b00001111, 0b11100000),
        (0b00011111, 0b11000000),
        (0b00111111, 0b10000000),
        (0b01111111, 0b00000000),
    ];
    let (left_map, right_map) = mappers[index];
    if index == 7 {
        return byte1 & left_map;
    }
    let left_shift = (7 - index) as u8;
    let right_shift = (index + 1) as u8;
    let left_side = (byte1 & left_map) << left_shift;
    let right_side = (byte2 & right_map) >> right_shift;
    left_side | right_side
}
