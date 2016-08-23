extern crate aliasmethod;
extern crate rand;

#[cfg(test)]
mod tests {
    use aliasmethod::AliasTable;
    use std::collections::HashMap;

    struct TestParam {weights: Vec<f64>, rates: Vec<f64>}

    #[test]
    fn test_probability() {
        let params: [TestParam; 6] = [
            TestParam {weights: vec![10.0, 15.0],       rates:vec![40.0, 60.0]},
            TestParam {weights: vec![20.0, 30.0],       rates:vec![40.0, 60.0]},
            TestParam {weights: vec![20.0, 5.0],        rates:vec![80.0, 20.0]},
            TestParam {weights: vec![25.0],             rates:vec![100.0]},
            TestParam {weights: vec![1.0, 99.0],        rates:vec![1.0, 99.0]},
            TestParam {weights: vec![1.0, 1.0, 8.0],    rates:vec![10.0, 10.0, 80.0]},
        ];

        for (param_no, &TestParam{ref weights, ref rates}) in params.into_iter().enumerate() {
            let alias_table = AliasTable::new(&weights);
            match alias_table {
                Err(e) => {
                    assert!(false, "error : {}", e);
                }
                Ok(actual) => {
                    let sample: i64 = 100000;

                    let mut results: HashMap<usize, i64> = HashMap::new();

                    for _ in 0..sample {
                        let n = actual.random();
                        let count = results.entry(n).or_insert(0);
                        *count += 1;
                    }

                    let fs = sample as f64;

                    for (n, rate) in rates.into_iter().enumerate() {
                        match results.get(&n) {
                            None => {
                                assert!(false, "Not fond '{}' in results", n);
                            }
                            Some(count) => {
                                let p: f64 = rate / 100.0;
                                let q: f64 = 1.0 - p;

                                let expected = fs * p;
                                // 3.89 = inverse of normal distribution function with alpha=0.9999
                                let delta = 3.89 * (fs * p * q).sqrt();

                                let count = *count as f64;
                                let ok = expected - delta <= count && count <= expected + delta;
                                assert!(ok, format!(
                                        "The probability is out of by interval estimation. param_no={} num={} weight={} actual={}, expected={}, delta={}",
                                        param_no, n, weights[n as usize], count, expected, delta));
                            }
                        }
                    }
                }
            }
        }
    }
}
