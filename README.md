# pearson_hash_rust
- matching instead of 'unwrap' for panic management in production

```rust
/// Calculates the Pearson hash of a given input slice.
///
/// # Arguments
///
/// * `input_u8_int`: A slice of bytes representing the input data.
///
/// # Returns
///
/// * `Result<u8, std::io::Error>`: The calculated Pearson hash as a `u8` value,
///   or an `Error` if there was a problem converting the input data.
///
/// # Example
///
/// ```rust
/// use pearson_hash::pearson_hash_base;
///
/// let input = "hello world".as_bytes();
/// let hash = pearson_hash_base(input).unwrap();
/// assert_eq!(hash, 124);
/// ```
///
/// # References
///
/// * [Pearson Hashing on Wikipedia](https://en.wikipedia.org/wiki/Pearson_hashing)
/// * [Pearson Hashing in Rust](https://hashing.mojoauth.com/pearson-hashing-in-rust/)
fn pearson_hash_base(input_u8_int: &[u8]) -> Result<u8, std::io::Error> {
    // based on: https://en.wikipedia.org/wiki/Pearson_hashing
    // based on: https://hashing.mojoauth.com/pearson-hashing-in-rust/
    println!(
        "pearson_hash_base(): Input data: {:?}",
        input_u8_int
    );

    let mut p_hash_table = [0u8; 256];
    for p_iter_item in 0..256 {
        // Explicit pattern matching
        let u8_value: u8 = (p_iter_item % 256).try_into().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "usize value too large for u8")
        })?;
        p_hash_table[p_iter_item] = (p_iter_item as u8).wrapping_add(p_iter_item as u8);
    }

    let mut pearson_hash_output: u8 = 0;
    for p_iter_byte in input_u8_int {
        pearson_hash_output = p_hash_table[(pearson_hash_output ^ p_iter_byte) as usize];
        println!(
            "pearson_hash_base(): current_byte={:?}, pearson_hash_output={:?}",
            p_iter_byte,
            pearson_hash_output
        );
    }

    println!(
        "pearson_hash_base(): Final pearson_hash_output={:?}",
        pearson_hash_output
    );
    Ok(pearson_hash_output)
}


```
