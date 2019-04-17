use std::string::FromUtf8Error;

// -> [ baaaaaaa, ccbbbbbb, dddccccc, eeeedddd, fffffeee, ggggggff, hhhhhhhg,           0iiiiiii ]
//                          ---           ====
//                           '----------.  |
//                                     ---====
// <- [ 0aaaaaaa, 0bbbbbbb, 0ccccccc, 0ddddddd, 0eeeeeee, 0fffffff, 0ggggggg, 0hhhhhhh, 0iiiiiii ]
pub fn grow(bytes: &Vec<u8>) -> Result<String, FromUtf8Error> {
    let mut out = Vec::new();

    for i in 0..bytes.len() {
        let byte1 = if i == 0 { 0b0 } else { bytes[i - 1] };
        let byte2 = bytes[i];
        let from_byte1 = i % 7;

        let next_byte = expand(from_byte1, byte1, byte2);
        out.push(next_byte);

        if from_byte1 == 6 {
            let next_byte = expand(7, byte2, 0b0);
            if next_byte != 0b0 {
                out.push(next_byte);
            }
        }
    }

    String::from_utf8(out)
}

fn expand(from_byte1: usize, byte1: u8, byte2: u8) -> u8 {
    let (left, right) = [
        (0b00000000, 0b01111111), // 0 and 7 (a)
        (0b10000000, 0b00111111), // 1 and 6 (b)
        (0b11000000, 0b00011111), // 2 and 5 (c)
        (0b11100000, 0b00001111), // 3 and 4 (d)
        (0b11110000, 0b00000111), // 4 and 3 (e)
        (0b11111000, 0b00000011), // 5 and 2 (f)
        (0b11111100, 0b00000001), // 6 and 1 (g)
        (0b11111110, 0b00000000), // 7 and 0 (h)
    ][from_byte1];
    let byte1_stripped = (byte1 & left) >> 1; // dddccccc -> 0ddd0000
    let byte2_stripped = byte2 & right; // eeeedddd -> 0000dddd
    byte1_stripped | byte2_stripped // 0ddddddd
}
