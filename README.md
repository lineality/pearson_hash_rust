# pearson_hash_rust


```rust
/// Calculates the Pearson hash of a given input slice.
fn pearson_hash_base(input_u8_int: &[u8]) -> Result<u8, std::io::Error> {  // Returns std::io::Error
    // based on: https://en.wikipedia.org/wiki/Pearson_hashing
    // based on: https://hashing.mojoauth.com/pearson-hashing-in-rust/
    let mut p_hash_table = [0u8; 256];
    for p_iter_item in 0..256 {
        // Explicit pattern matching
        let u8_value: u8 = (p_iter_item % 256).try_into().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "usize value too large for u8")
        })?;
        p_hash_table[p_iter_item] = (p_iter_item as u8).wrapping_sub(u8_value);
    }
    // for p_iter_item in 0..256 {
    //     p_hash_table[p_iter_item] = (p_iter_item as u8).wrapping_sub(p_iter_item % 256);
    // }
    
    let mut pearson_hash_output: u8 = 0;
    for p_iter_byte in input_u8_int {
        pearson_hash_output = p_hash_table[(pearson_hash_output ^ p_iter_byte) as usize];
    }
    Ok(pearson_hash_output)
}

```
