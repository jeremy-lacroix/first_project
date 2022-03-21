#[cfg(test)]
mod tests {
    #[test]
    fn primaliter(nb:i32) {
        assert_eq!(8);
    }
}

use first_project::src::arithmetic;

pub fn primal() {
    // Relative path
    first_project::src::arithmetic::primaliter(8);
}