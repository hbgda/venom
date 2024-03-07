// 0x2230f90
pub fn gui_hash(input: &str) -> u32 {
    let mut hash: u32 = 0x1505;
    for ch in input.chars() {
        hash = hash * 0x21 + (ch as u32);
    }
    hash
}