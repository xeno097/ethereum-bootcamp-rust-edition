use sha3::Digest;

// 1:Find Favourite Color
const COLORS: [&str; 6] = ["red", "green", "blue", "yellow", "pink", "orange"];

fn find_color(hash: &str) -> Option<&str> {
    let mut hasher = sha3::Keccak256::new();

    COLORS.iter().copied().find(|color| {
        hasher.update(color);

        let hashed_color = hasher.finalize_reset();

        format!("{:x}", hashed_color) == hash
    })
}

#[cfg(test)]
mod tests {
    use super::{find_color, COLORS};
    use sha3::Digest;

    #[test]
    fn finds_color_given_hash() {
        let mut hasher = sha3::Keccak256::new();

        COLORS.iter().copied().for_each(|color| {
            // Arrange
            hasher.update(color);

            let hashed_color = hasher.finalize_reset();
            let hex_hashed_color = format!("{:x}", hashed_color);

            // Act
            let found_color = find_color(&hex_hashed_color).unwrap();

            // Assert
            assert_eq!(color, found_color);
        })
    }
}
