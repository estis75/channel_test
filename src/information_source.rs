extern crate rand;
use rand::Rng;

pub struct InformationSource {
  length: usize,
  prob_array: Vec<f64>
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
    
    InformationSource{
      length,
      prob_array: prob_array
    }
  }

  fn set_probs(&mut self, prob_array: &Vec<f64>) {
    let sum = prob_array.iter().fold(0.0, |sum, l| sum+l);
    assert!(1.0-0.005 <= sum && sum <= 1.0+0.005, "sum of probability must be 1.0 (with few errors); sum: {}", sum);

    self.prob_array = prob_array.clone();
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
}