//module to handle endianness


pub fn little(input: u32) -> Vec<u8> {
    (0 .. 8).map(|i| (input >> (i * 4) & 0xF) as u8).collect()
}
