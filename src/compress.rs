// -> [ 0aaaaaaa, 0bbbbbbb, 0ccccccc, 0ddddddd, 0eeeeeee, 0fffffff, 0ggggggg, 0hhhhhhh, 0iiiiiii ]
//                                     ---====
//                           ,----------'  |
//                          ---           ====
// <- [ baaaaaaa, ccbbbbbb, dddccccc, eeeedddd, fffffeee, ggggggff, hhhhhhhg, 0iiiiiii ]
pub fn shrink(s: &str) -> Vec<u8> {
    let strippers = [
        (0b01111111, 0b01000000), // (a, b)
        (0b00111111, 0b01100000), // (b, c)
        (0b00011111, 0b01110000), // (c, d)
        (0b00001111, 0b01111000), // (d, e)
        (0b00000111, 0b01111100), // (e, f)
        (0b00000011, 0b01111110), // (f, g)
        (0b00000001, 0b01111111), // (g, h)
    ];
    let padding = 0b00000000;
    let mut bytes: Vec<u8> = s.bytes().collect();
    bytes.push(padding);

    bytes
        .windows(2)
        .enumerate()
        .map(|(i, pair)| (i % 8 + 1, pair[0], pair[1])) // i = number of bits we need to move
        .filter(|(i, _, _)| *i != 8 as usize) // The 8th one is the one we skip
        .map(|(from_byte2, byte1, byte2)| {
            if !byte1.is_ascii() {
                panic!("String must be ascii");
            }
            let (left, right) = strippers[from_byte2 - 1];
            let byte1_stripped = byte1 & left; // take only the parts we need of each
            let byte2_stripped = (byte2 & right) << 1;
            byte1_stripped | byte2_stripped // combine bytes
        })
        .collect()
}
