use comp_macro::comp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mscc = vec![1,5,2,5];
        let result = comp![x * x for x in mscc if x != 5].collect::<Vec<_>>();
        assert_eq!(result, vec![1, 4]);
    }
}
