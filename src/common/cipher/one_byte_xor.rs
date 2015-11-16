use std::slice;

use common::{err, ascii, charfreq, util};


pub struct Guess {
    pub plain:      Vec<u8>,
    pub key:        u8,
    pub distance:   f32
}

pub fn guess_key(cipher: &Vec<u8>, opt_options: Option<&GuessOptions>) -> Result<Guess, err::Error> {
    let mut guess_dist = Vec::new();
    let default_options;
    let options: &GuessOptions = match opt_options {
        Some(v) => v,
        None    => {
            default_options = GuessOptions::new();
            &default_options
            }
    };

    let evec = vec![0f32];
    let mut weight_it = evec.iter();
    if options.weights_available() {
        weight_it = options.weight_iter().unwrap();
    }

    for key in options.keys() {
        let dist: f32;
        if options.weights_available() {
            dist = try!(options.weighted_distance(&xor(&cipher, key), *(weight_it.next().unwrap())));
        } else {
            dist = try!(options.distance(&xor(&cipher, key)));
        }
            
        guess_dist.push(dist);
    }

    let best_key_idx = util::min_index(&guess_dist).unwrap();
    let best_key = try!(options.idx_to_key(best_key_idx));

    Ok(Guess {
        plain:      xor(&cipher, best_key),
        key:        best_key,
        distance:   guess_dist[best_key_idx]
    })
}

pub type FnDistance = fn (&str) -> Result<f32, err::Error>;
pub type FnWeight = fn(f32, f32) -> f32;


pub struct GuessOptions<'a> {
    candidates:     Option<&'a Vec<u8>>,
    weights:        Option<&'a Vec<f32>>,
    start:          Option<u8>,
    end:            Option<u8>,
    distance_fn:    FnDistance,
    weight_fn:      FnWeight 
}

impl<'a> GuessOptions<'a> {
    pub fn new() -> GuessOptions<'a> {
        fn default_weight_fn(d: f32, w: f32) -> f32 {
            d * w
        }

        GuessOptions {
            candidates:     None,
            weights:        None,
            start:          None,
            end:            None,
            distance_fn:    charfreq::distance_from_base,
            weight_fn:      default_weight_fn
        }
    }

    pub fn set_candidates(&mut self, candidates: &'a Vec<u8>, weights: &'a Vec<f32>) -> Result<(), err::Error> {
        ctry!(self.start.is_some(), "candidates are redundant, range is already set");
        ctry!(weights.len() > 0 && (weights.len() != candidates.len()), "each candidate must have weight, if weights are provided");

        self.candidates = Some(candidates);
        self.weights= Some(weights);
        Ok(())
    }

    pub fn set_range(&mut self, start: u8, end: u8) -> Result<(), err::Error> {
        ctry!(start >= end, "invalid range, start value must be less than end value");
        ctry!(self.candidates.is_some(), "range is redundant, candidates are already set");

        self.start = Some(start);
        self.end = Some(end);
        Ok(())
    }

    pub fn set_distance_fn(&mut self, distance_fn: FnDistance) {
        self.distance_fn = distance_fn;
    }

    pub fn set_weight_fn(&mut self, weight_fn: FnWeight) {
        self.weight_fn = weight_fn; 
    }

    fn keys(&self) -> Vec<u8> {
        if self.start.is_some() {
            return (self.start.unwrap() .. self.end.unwrap() + 1).map(|v| v).collect();
        } else if self.candidates.is_some() {
            return self.candidates.unwrap().clone();
        } else {
            let mut r: Vec<u8> = (0..255).map(|v| v).collect();
            r.push(255);
            return r;
        }
    }

    fn weight_iter(&self) -> Option<slice::Iter<'a, f32>> {
        match self.weights {
            Some(v) => Some(v.iter()),
            None    => None
        }
    }

    fn weights_available(&self) -> bool {
        match self.weights {
            Some(v) => v.len() > 0,
            None    => false
        }
    }

    fn idx_to_key(&self, idx: usize) -> Result<u8, err::Error> {
        if self.start.is_some() {
            ctry!(idx as u8 >= self.end.unwrap() - self.start.unwrap(), "index out of range");
            return Ok(self.start.unwrap() + idx as u8);
        } else if self.candidates.is_some() {
            return Ok(self.candidates.unwrap()[idx]);
        } else {
            return Ok(idx as u8);
        }
    }

    fn distance(&self, input: &Vec<u8>) -> Result<f32, err::Error> {
        (self.distance_fn)(rts!(&input).as_ref())
    }

    fn weighted_distance(&self, input: &Vec<u8>, weight: f32) -> Result<f32, err::Error> {
        Ok((self.weight_fn)(try!(self.distance(&input)), weight))
    }
}

pub fn xor(input: &Vec<u8>, key: u8) -> Vec<u8> {
    let output: Vec<u8> = input.iter().map(|b| *b ^ key).collect();
    output
}

