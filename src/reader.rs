pub type Source = ();

pub fn reader_source(filename: &str) -> Source {
    println!("Reading source code...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!((), ());
    }
}
