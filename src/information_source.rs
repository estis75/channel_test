extern crate rand;
use rand::Rng;
use std::collections::HashMap;

pub struct InformationSource {
  length: usize,
  prob_array: Vec<f64>,
  code_array: Vec<Vec<u8>>,
  source_array: Vec<String>,
  encoder: HashMap<String, usize>,
  decoder: HashMap<String, usize>
}

impl InformationSource {
  pub fn new(length: usize) -> InformationSource {
    let mut rng = rand::thread_rng();
    let mut prob_array = Vec::with_capacity(length);
    let mut sum = 0.0;
    for _ in 0..length {
      let r: f64 = rng.gen_range(0.0f64..(1.0f64-sum));
      prob_array.push(r);
      sum += r;
    }

    let mut encoder = HashMap::new();
    let mut decoder = HashMap::new();
    let mut source_array = Vec::new();

    let code = (0..length).map(|num| {
      let mut e = num as u128;
      let mut v = Vec::new();
      while e > (1<<8) {
        v.push((e%(1<<8)) as u8);
        e >>= 8;
      }
      v.push(e as u8);
      let src: String = v.iter().rev().map(|&e| char::from_digit(e as u32, 10).unwrap()).collect();
      source_array.push(src.clone());
      encoder.insert(src.clone(), num);
      decoder.insert(src, num);
      return v.iter().rev().map(|&e| e).collect::<Vec<u8>>();
    }).collect::<Vec<Vec<u8>>>();

    InformationSource{
      length,
      prob_array: prob_array,
      code_array: code,
      source_array: source_array,
      encoder: encoder,
      decoder: decoder
    }
  }

  pub fn set_probs(&mut self, prob_array: &Vec<f64>) {
    let sum = prob_array.iter().fold(0.0, |sum, l| sum+l);
    assert!(1.0-0.005 <= sum && sum <= 1.0+0.005, "sum of probability must be 1.0 (with few errors); sum: {}", sum);

    self.prob_array = prob_array.clone();
  }

  pub fn set_codes(&mut self, code_array: &Vec<Vec<u8>>) {
    let mut code = code_array.iter().map(
      |v| v.iter().fold(String::from(""), 
          |sum, &x| sum + &char::from_digit(x as u32, 10).unwrap().to_string()
        )
      ).collect::<Vec<String>>();
    code.sort();
    code.dedup();
    assert_eq!(code.len(), code_array.len(), "duplicate code is not allowed");

    self.code_array = code_array.clone();

    let mut decoder = HashMap::new();
    for (i, code) in code_array.iter().enumerate() {
      let code: String = code.iter().map(|&e| char::from_digit(e as u32, 10).unwrap()).collect();
      decoder.insert(code, i);
    }
    self.decoder = decoder;
  }
  
  pub fn set_source(&mut self, src_array: &Vec<String>) {
    let mut src = src_array.clone();
    src.sort();
    src.dedup();
    assert_eq!(src.len(), src_array.len(), "duplicate src is not allowed");

    self.source_array = src_array.clone();

    let mut encoder = HashMap::new();
    for (i, src) in src_array.iter().enumerate() {
      encoder.insert(src.clone(), i);
    }
    self.encoder = encoder;
  }
}

impl std::fmt::Debug for InformationSource {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let output: String = format!("\
      InformationSource{{\n\
      \tlength: {:?},\n\
      \tsource_array: {:?},\n\
      \tcode_array: {:?},\n\
      \tprob_array: {:?},\n\
      }}
    ", &self.length, &self.source_array, &self.code_array, &self.prob_array);
    f.pad_integral(true, "", &output)
  }
}

#[cfg(test)]
mod test {
  use super::InformationSource;
  #[test]
  fn test_set_information_source_length() {
    let length = 10;
    let v = InformationSource::new(length);
    assert_eq!(v.length, length);
    let sum = v.prob_array.iter().fold(0.0, |sum, l| sum+l);
    assert!(1.0-0.05 <= sum && sum <= 1.0+0.05 );
    assert_eq!(v.prob_array.len(), length);
    assert_eq!(v.code_array.len(), length);
  }

  #[test] 
  fn test_set_probs() {
    let length = 10;
    let mut v = InformationSource::new(length);
    let probs = vec![0.4,0.2,0.1,0.1,0.1,0.02,0.02,0.02,0.02,0.02];
    v.set_probs(&probs);
    assert_eq!(v.prob_array, probs);
  }
  
  #[test]
  #[should_panic]
  fn test_failcase_set_probs() {
    let length = 10;
    let mut v = InformationSource::new(length);
    let probs = vec![0.4,0.2,0.1,0.1,0.1,0.01,0.01,0.01,0.01,0.01];
    v.set_probs(&probs);
    assert_eq!(v.prob_array, probs);
  }

  #[test] 
  fn test_set_codes() {
    let length = 10;
    let mut v = InformationSource::new(length);
    let code = [[0,0], [0,1], [1,0], [1,1]].to_vec().iter().map(|v| v.to_vec()).collect::<Vec<Vec<u8>>>();
    v.set_codes(&code);
    assert_eq!(v.code_array, code);
  }
  
  #[test]
  #[should_panic]
  fn test_failcase_set_codes() {
    let length = 10;
    let mut v = InformationSource::new(length);
    let code = [[0,0].to_vec(), [1,1].to_vec(), [1,0].to_vec(), [1,1].to_vec()].to_vec();
    v.set_codes(&code);
    assert_eq!(v.code_array, code);
  }

  #[test]
  fn test_encoder_decoder() {
    let srclen = 4;
    let mut v = InformationSource::new(srclen);
    let code = [[1,1],[1,0],[0,1],[0,0]].to_vec().iter().map(|e| e.to_vec()).collect::<Vec<Vec<u8>>>();
    let source = ['a'.to_string(), 'b'.to_string(), 'c'.to_string(), 'd'.to_string()].to_vec();
    v.set_codes(&code);
    v.set_probs(&[0.4,0.3,0.0,0.3].to_vec());
    v.set_source(&source);
    for (i, cd) in code.iter().enumerate() {
      let cd: String = cd.iter().map(|&e| char::from_digit(e as u32, 10).unwrap()).collect();
      assert_eq!(v.decoder.get(&cd), Some(&i));
    }
    assert_eq!(v.decoder.len(), srclen);
    for (i, src) in source.iter().enumerate() {
      assert_eq!(v.encoder.get(src), Some(&i));
    }
    assert_eq!(v.encoder.len(), srclen);
  }
}