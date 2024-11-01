# pearson_hash_rust
A comparison of implimentations
```rust
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
fn pearson_hash7(input_u8_int: &[u8]) -> Result<u8, std::io::Error> {
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



use std::collections::HashMap;

// Creates a permutation table (no shuffling needed)
fn create_permutation_table() -> HashMap<u8, u8> {
    let mut table: HashMap<u8, u8> = HashMap::new();
    for i in 0..255 {
        table.insert(i, i); // Simple identity mapping
    }
    table
}

// Computes the Pearson hash of a message using the permutation table.
fn pearson_hash1(message: &str, table: &HashMap<u8, u8>) -> u8 {
    let mut p_hash: u8 = 0;
    for chunk in message.bytes() {
        p_hash = table[&(p_hash ^ chunk)];
    }
    p_hash
}


// Define the permutation table as a const array
const T: [u8; 256] = [
    // You should fill this with a permutation of 0-255
    // Here's an example permutation (you might want to use a different one)
    98,  6, 85,150, 36, 23,112,164,135,207,169,  5, 26, 64,165,219, //  1
    61, 20, 68, 89,130, 63, 52,102, 24,229,132,245, 80,216,195,115, //  2
    90,168,156,203,177,120,  2,190,188,  7,100,185,174,243,162, 10, //  3
    237, 18,253,225,  8,208,172,244,255,126,101, 79,145,235,228,121, //  4
    123,251, 67,250,161,  0,107, 97,241,111,181, 82,249, 33, 69, 55, //  5
    59,153, 29,  9,213,167, 84, 93, 30, 46, 94,  75,151,114, 73,222, //  6
    197, 96,210, 45, 16,227,248,202, 51,152,252,125, 81,206,215,186, //  7
    39,158,178,187,131,136,  1, 49, 50, 17,141, 91, 47,129,  60,99, //  8
    154, 35, 86,171,105, 34, 38,200,147, 58, 77,118,173,246, 76,254, //  9
    133,232,196,144,198,124, 53,  4,108, 74,223,234,134,230,157,139, // 10
    189,205,199,128,176, 19,211,236,127,192,231, 70,233, 88,146, 44, // 11
    183,201, 22, 83, 13,214,116,109,159, 32, 95,226,140,220, 57, 12, // 12
    221, 31,209,182,143, 92,149,184,148, 62,113, 65, 37, 27,106,166, // 13
    3, 14,204, 72, 21, 41, 56, 66, 28,193, 40,217, 25, 54,179,117, // 14
    238, 87,240,155,180,170,242,212,191,163, 78,218,137,194,175,110, // 15
    43,119,224, 71,122,142, 42,160,104, 48,247,103,15, 11,138,239  // 16
];

pub fn pearson_hash3(input: &[u8]) -> u8 {
    let mut hash: u8 = 0;
    
    for &byte in input {
        hash = T[(hash ^ byte) as usize];
    }
    
    hash
}


// Function to generate the permutation table at compile time
const fn generate_permutation_table1() -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        table[i as usize] = i as u8;
        i += 1;
    }
    table
}

// Use the generated table as a constant
const PERMUTATION_TABLE1: [u8; 256] = generate_permutation_table1();

pub fn pearson_hash4(input: &[u8]) -> u8 {
    let mut hash: u8 = 0;
    
    for &byte in input {
        hash = PERMUTATION_TABLE1[(hash ^ byte) as usize];
    }
    
    hash
}



/// Implementation of the Pearson hashing algorithm
/// 
/// This is a non-cryptographic hash function that produces an 8-bit hash value.
/// It's useful for:
/// - Hash tables
/// - Data integrity checks
/// - Fast execution on 8-bit processors
/// 
/// Features:
/// - Simple implementation
/// - Fast execution
/// - No simple class of inputs that produce collisions
/// - Two strings differing by one character never collide
/// 
/// Reference: Pearson, Peter K. (1990). "Fast Hashing of Variable-Length Text Strings"



/// Computes the Pearson hash of the input bytes
/// 
/// # Arguments
/// 
/// * `input` - A slice of bytes to hash
/// 
/// # Returns
/// 
/// * An 8-bit hash value as u8
/// 
/// # Example
/// 
/// ```
/// let text = "Hello, World is the first onasei!";
/// let hash = pearson_hash(text.as_bytes());
/// println!("Hash: {}", hash);
/// ```
pub fn pearson_hash5(input: &[u8]) -> u8 {
    // Initialize hash to 0
    let mut hash: u8 = 0;
    
    // For each byte in the input
    for &byte in input {
        // XOR the current byte with the hash, use result as index into permutation table
        hash = PERMUTATION_TABLE[(hash ^ byte) as usize];
    }
    
    hash
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_basic_hash() {
        let result = pearson_hash5(b"Hello, World is the first onasei!");
        assert_ne!(result, 0); // Basic sanity check
    }

    #[test]
    fn test_different_inputs() {
        // Two different inputs should (likely) have different hashes
        let hash1 = pearson_hash5(b"test1");
        let hash2 = pearson_hash5(b"test2");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_empty_input() {
        let result = pearson_hash5(b"");
        assert_eq!(result, PERMUTATION_TABLE[0]); // Empty input should return first table value
    }
}


/// Implementation of the Pearson hashing algorithm
/// 
/// This is a non-cryptographic hash function that produces an 8-bit hash value.
/// It's useful for:
/// - Hash tables
/// - Data integrity checks
/// - Fast execution on 8-bit processors
/// 
/// Features:
/// - Simple implementation
/// - Fast execution
/// - No simple class of inputs that produce collisions
/// - Two strings differing by one character never collide
/// 
/// Reference: Pearson, Peter K. (1990). "Fast Hashing of Variable-Length Text Strings"

// Generate a permutation table using a non-linear transformation
// This is done at compile time using const fn
const fn generate_permutation_table() -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        // Non-linear transformation: multiply by prime number 167 and add 13
        // Then mask with 0xFF to keep it within u8 range
        table[i as usize] = ((i * 167 + 13) & 0xFF) as u8;
        i += 1;
    }
    table
}

// The permutation table is computed once at compile time
const PERMUTATION_TABLE: [u8; 256] = generate_permutation_table();

/// Computes the Pearson hash of the input bytes
/// 
/// # Arguments
/// 
/// * `input` - A slice of bytes to hash
/// 
/// # Returns
/// 
/// * An 8-bit hash value as u8
/// 
/// # Example
/// 
/// ```
/// let text = "Hello, World is the first onasei!";
/// let hash = pearson_hash6(text.as_bytes());
/// println!("Hash: {}", hash);
/// ```
pub fn pearson_hash6(input: &[u8]) -> u8 {
    // Initialize hash to 0
    let mut hash: u8 = 0;
    
    // For each byte in the input
    for &byte in input {
        // XOR the current byte with the hash, use result as index into permutation table
        hash = PERMUTATION_TABLE[(hash ^ byte) as usize];
    }
    
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_hash() {
        let result = pearson_hash6(b"Hello, World is the first onasei!");
        assert_ne!(result, 0); // Basic sanity check
    }

    #[test]
    fn test_different_inputs() {
        // Two different inputs should (likely) have different hashes
        let hash1 = pearson_hash6(b"test1");
        let hash2 = pearson_hash6(b"test2");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_empty_input() {
        let result = pearson_hash6(b"");
        assert_eq!(result, PERMUTATION_TABLE[0]); // Empty input should return first table value
    }
}


fn main() -> Result<(), io::Error> {

    let input = "Hello, World is the first onasei".as_bytes();
    let hash = pearson_hash7(input)?;
    println!("\nbroken Pearson  hash of '{:?}': {}", input, hash);

    let message = "Hello, World is the first onasei";
    let permutation_table = create_permutation_table();

    // println!("\npermutation_table '{:?}", permutation_table);

    let hash = pearson_hash1(message, &permutation_table);
    println!("pearson_hash1 Hash of '{}': {}", message, hash);

    let text = "Hello, World is the first onasei";
    let hash = pearson_hash3(text.as_bytes());
    println!("pearson_hash3 Hash of '{}' is: {}", text, hash);

    let text = "Hello, World is the first onasei";
    let hash = pearson_hash4(text.as_bytes());
    println!("pearson_hash4 Hash of '{}' is: {}", text, hash);

    // Example usage
    let text = "Hello, World is the first onasei";
    let hash = pearson_hash5(text.as_bytes());
    println!("pearson_hash5 Hash of '{}' is: {}", text, hash);

    // Example usage
    let text = "Hello, World is the first onasei";
    let hash = pearson_hash6(text.as_bytes());
    println!("pearson_hash6 Hash of '{}' is: {}", text, hash);

    Ok(())
}

```
