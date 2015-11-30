pub fn identity(x: i32) -> i32 {
    x
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, identity(4));
    }

}
