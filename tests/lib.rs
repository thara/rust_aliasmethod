extern crate aliasmethod;
extern crate rand;

#[cfg(test)]
mod tests {
    use aliasmethod::AliasTable;

    #[test]
    fn it_works() {
        let weights: Vec<f64> = vec![30.0, 70.0];
        let alias_table = AliasTable::new(&weights);
        match alias_table {
            Ok(v) => {
                let r = v.random();
                assert!(r <= weights.len() - 1);
            }
            Err(e) => {
                assert!(false, "error : {}", e);
            }
        }
    }
}
