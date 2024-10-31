use std::io;


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

// /// Calculates the Pearson hash of a given input slice.
// /// 
// /// # Arguments
// /// 
// /// * `input_u8_int`: A slice of bytes representing the input data.
// /// 
// /// # Returns
// /// 
// /// * `Result<u8, std::io::Error>`:  The calculated Pearson hash as a `u8` value, 
// ///   or an `Error` if there was a problem converting the input data.
// /// 
// /// # Example
// /// 
// /// ```rust
// /// use pearson_hash::pearson_hash_base;
// /// 
// /// let input = "hello world".as_bytes();
// /// let hash = pearson_hash_base(input).unwrap();
// /// assert_eq!(hash, 124);
// /// ```
// /// 
// /// # References
// /// 
// /// * [Pearson Hashing on Wikipedia](https://en.wikipedia.org/wiki/Pearson_hashing)
// /// * [Pearson Hashing in Rust](https://hashing.mojoauth.com/pearson-hashing-in-rust/)
// fn pearson_hash_base(input_u8_int: &[u8]) -> Result<u8, std::io::Error> {
//     // Create a lookup table for Pearson hashing
//     let mut p_hash_table = [0u8; 256];
//     // Initialize the lookup table 
//     for p_iter_item in 0..256 {
//         p_hash_table[p_iter_item] = (p_iter_item as u8).wrapping_add(p_iter_item as u8);
//     }

//     // Initialize the hash output
//     let mut pearson_hash_output: u8 = 0;

//     // Iterate over each byte in the input data
//     for p_iter_byte in input_u8_int {
//         // Calculate the hash output based on the current byte and lookup table
//         pearson_hash_output = p_hash_table[(pearson_hash_output ^ p_iter_byte) as usize]; 
//     }

//     // Return the calculated hash output
//     Ok(pearson_hash_output)
// }



/// Calculates the Pearson hash of a given input slice, handling longer inputs by splitting
/// into chunks and applying the base hash function.
///
/// # Arguments
///
/// * `input`: A slice of bytes representing the input data.
///
/// # Returns
///
/// * `Result<u8, std::io::Error>`: The calculated Pearson hash as a `u8` value,
///   or an `Error` if there was a problem calculating the hash.
fn pearson_hash_wrapper2(input: &[u8]) -> Result<u8, std::io::Error> {
    // Initialize the hash output
    let mut pearson_hash_output: u8 = 0;

    // Split the input into chunks of size 256
    for chunk in input.chunks(256) {
        // Calculate the base hash of the chunk
        let chunk_hash = pearson_hash_base(chunk)?;
        // Combine the chunk hash with the overall output (XOR)
        pearson_hash_output ^= chunk_hash;
    }

    // Return the final hash
    Ok(pearson_hash_output)
}


/// Calculates the Pearson hash of a given input slice, handling longer inputs by splitting
/// into chunks and applying the base hash function.
///
/// # Arguments
///
/// * `input`: A slice of bytes representing the input data.
///
/// # Returns
///
/// * `Result<u8, std::io::Error>`: The calculated Pearson hash as a `u8` value,
///   or an `Error` if there was a problem calculating the hash.
fn pearson_hash_wrapper(input: &[u8]) -> Result<u8, std::io::Error> {
    // Initialize the hash output
    let mut pearson_hash_output: u8 = 0;

    // Split the input into chunks of size 256
    for chunk in input.chunks(256) {
        // Calculate the base hash of the chunk
        let chunk_hash = pearson_hash_base(chunk)?;
        // Combine the chunk hash with the overall output (sum modulo 256)
        pearson_hash_output = pearson_hash_output.wrapping_add(chunk_hash);
    }

    // Return the final hash
    Ok(pearson_hash_output)
}

fn main() -> Result<(), io::Error> {

    let input = "to test the wrapper function".as_bytes();
    let hash = pearson_hash_wrapper(input)?;
    println!("Pearson 1 hash of '{:?}': {}", input, hash);

    let input = "This is a longer string to test the wrapper function".as_bytes();
    let hash = pearson_hash_wrapper(input)?;
    println!("Pearson 1 hash of '{:?}': {}", input, hash);

    let input = "hello world".as_bytes();
    let hash = pearson_hash_wrapper2(input)?;
    println!("\nPearson 2 hash of '{:?}': {}", input, hash);

    let input = "hello hello world".as_bytes();
    let hash = pearson_hash_wrapper2(input)?;
    println!("\nPearson 2 hash of '{:?}': {}", input, hash);

    Ok(())
}
