use rand::Rng;

// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#create-random-passwords-from-a-set-of-user-defined-characters
// can also directly use - https://docs.rs/rand/latest/rand/
pub fn generate_code(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            0123456789";
    let mut rng = rand::rng();

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    password
}
