extern crate rand;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};


#[derive(Debug)]
pub struct AliasTable {
    len: i32,
    prob: Vec<f32>,
    alias: Vec<usize>,
}


impl AliasTable {

    pub fn new(weights: &Vec<f32>) -> Result<AliasTable, &'static str> {
        let n = weights.len() as i32;

        let sum = weights.iter().fold(0.0, |acc, x| acc + x);
        if sum == 0.0 {
            return Err("sum of weights is 0.");
        }

        let mut prob = weights.iter().map(|w| w * (n as f32) / sum).collect::<Vec<f32>>();
        let mut h = 0;
        let mut l = n - 1;
        let mut hl: Vec<usize> = vec![0; n as usize];

        for (i, p) in prob.iter().enumerate() {
            if *p < 1.0 {
                hl[l as usize] = i;
                l -= 1;
            }
            if 1.0 < *p {
                hl[h as usize] = i;
                h += 1;
            }
        }

        let mut a: Vec<usize> = vec![0; n as usize];

        while h != 0 && l != n - 1 {
            let j = hl[(l + 1) as usize];
            let k = hl[(h - 1) as usize];

            if 1.0 < prob[j] {
                panic!("MUST: {} <= 1", prob[j]);
            }
            if prob[k] < 1.0 {
                panic!("MUST: 1 <= {}", prob[k]);
            }

            a[j] = k;
            prob[k] -= 1.0 - prob[j];   // - residual weight
            l += 1;
            if prob[k] < 1.0 {
                hl[l as usize] = k;
                l -= 1;
                h -= 1;
            }
        }

        Ok(AliasTable {
            len: n,
            prob: prob,
            alias: a,
        })
    }

    pub fn random(&self) -> usize {
        let mut rng = rand::thread_rng();
        let u = rng.gen::<f32>();
        let range = Range::new(0, self.len);
        let n = range.ind_sample(&mut rng) as usize;

        if u <= self.prob[n] { n } else { self.alias[n] }
    }
}


#[test]
fn it_works() {
    let weights: Vec<f32> = vec![30.0, 70.0];
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
