//! Implementation of [Walker's Alias method](https://en.wikipedia.org/wiki/Alias_method)
extern crate rand;

use std::fmt;
use self::rand::{thread_rng, ThreadRng, Rng};
use self::rand::distributions::{IndependentSample, Range};


pub struct AliasMethod<RNG: Rng> {
    rng: RNG
}


/// Creates a new AliasMethod using the ThreadRng
pub fn alias_method() -> AliasMethod<ThreadRng> {
    AliasMethod::new(thread_rng())
}


#[derive(Debug)]
pub struct AliasTable {
    len: i32,
    prob: Vec<f64>,
    alias: Vec<usize>,
}

#[derive(Debug)]
pub enum AliasMethodError {
    ZeroTotalWeights,
    Internal{ text: String },
}

impl fmt::Display for AliasMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AliasMethodError::ZeroTotalWeights => write!(f, "Total of weights is 0."),
            AliasMethodError::Internal{ref text} => write!(f, "Internal error: {}", text),
        }
    }
}

impl<RNG: Rng> AliasMethod<RNG> {

    /// Creates a new AliasMethod struct.
    pub fn new(rng: RNG) -> Self {
        AliasMethod { rng: rng }
    }

    /// Chooses a index.
    pub fn random(&mut self, alias_table: &AliasTable) -> usize {
        let u = self.rng.next_f64();
        let range = Range::new(0, alias_table.len);
        let n = range.ind_sample(&mut self.rng) as usize;

        if u <= alias_table.prob[n] {
            n
        } else {
            alias_table.alias[n]
        }
    }
}


/// Creates a new AliasTable struct.
pub fn new_alias_table(weights: &Vec<f64>) -> Result<AliasTable, AliasMethodError> {
    let n = weights.len() as i32;

    let sum = weights.iter().fold(0.0, |acc, x| acc + x);
    if sum == 0.0 {
        return Err(AliasMethodError::ZeroTotalWeights)
    }

    let mut prob = weights.iter().map(|w| w * (n as f64) / sum).collect::<Vec<f64>>();
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
            return Err(AliasMethodError::Internal{text: format!("MUST: {} <= 1", prob[j])})
        }
        if prob[k] < 1.0 {
            return Err(AliasMethodError::Internal{text: format!("MUST: 1 <= {}", prob[k])})
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


#[test]
fn test_new_alias_table() {
    let params = [
        vec![1.0, 1.0],
        vec![1.0, 1.0, 8.0],
    ];
    for sample_weights in params.into_iter() {
        let alias_table = new_alias_table(&sample_weights);
        match alias_table {
            Ok(AliasTable {prob, ..}) => {
                assert_eq!(prob.len(), sample_weights.len());
            }
            Err(e) => {
                assert!(false, "error : {}", e);
            }
        }
    }
}
