/* Pre-calculates the binary table so it can be more efficient in execution.
 * If you need to calculate 92 bits, after this change you needed to do 736 iterations. Now 
 * you need to do 92 iterations.*/
const fn generate_crc32_table() -> [u32; 256]{
    let mut table = [0u32; 256];
    let mut index = 0;
    // Pre calculates the result of the CRC32 table.
    while index < 256{
        let mut crc = index as u32;
        let mut current_byte = 0;
        while current_byte < 8{
            // If the LSB is 1, shift and XOR it to the polynomial number. 
            if crc & 1 != 0{
                crc = (crc >> 1) ^ 0xEDB88320;
            // If the LSB is 0, shift it to the right.
            } else {
                crc >>= 1;
            }
            current_byte += 1;
        }
        // Replaces the current index to the position in the table 
        table[index] = crc;
        index += 1;
    }
    table
}

const CRC32_TABLE: [u32; 256] = generate_crc32_table();

// Matches the given data with the pre-calculated CRC32_TABLE.
pub fn calculate_crc32(data: &[u8]) -> u32{
    let mut crc: u32 = 0xFFFFFFFF;
    // Process each byte in the slice given 
    for &byte in data{
        let index = ((crc ^ byte as u32) & 0xFF) as usize;
        // matches the byte with the index in the table
        crc = (crc >> 8) ^ CRC32_TABLE[index];
    }
    // Returns the NOT value of the final crc (0x04C11DB7 polynomial)
    !crc
}
