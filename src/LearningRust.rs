use std::io;

fn get_float() -> f64 {
  println!("Enter decimal number...");

  let mut entry = String::new();

  io::stdin().read_line(&mut entry).expect("Enter a number.");

  let entry: f64 = entry
    .trim()
    .parse()
    .expect("Please type in a valid number.");  
  
  entry
}

fn build_vector(size: u32) -> Vec<f64> {
  let mut my_numbers: Vec<f64> = Vec::new();
  
  let mut i = 0;

  while i < size {
    my_numbers.push(get_float());
    i = i + 1;
  }
  my_numbers
}

fn get_sum(vector: &Vec<f64>) -> f64 {
  let mut total: f64 = 0.0;

  for x in vector {
    total += x;
  }
  total
}

fn get_mean(vector: &Vec<f64>) -> f64 {
  let z = get_sum(vector) as f64;
  let q = vector.len() as f64;

  z / q
}

fn sort(vector: &mut Vec<f64>) {
  for i in 0..vector.len() {
    for j in 0..vector.len() - i - 1 {
      if vector[j + 1] < vector[j] {
        vector.swap(j, j + 1);
      }
    }
  }
}

fn get_median(mut vector: &mut Vec<f64>) -> f64 {
  sort(&mut vector);

  let second_mid = vector.len() / 2;
  let first_mid = second_mid - 1;
  let average = (vector[second_mid] + vector[first_mid]) / 2.0;

  if vector.len() % 2 == 1 {
    vector[second_mid]
  }
  else {
  average
  }
}

fn get_mode(mut vector: &mut Vec<f64>) -> f64 {
  sort (&mut vector);
  
  let mut max = 0;

  // Note that it will default to first element if none are the most frequent
  let mut max_elem = vector[0];

  for i in 0.. vector.len() {
    let mut count = 0;
    for j in 0..vector.len() {
        if vector[i] == vector[j] {
            count = count + 1;
        }
    }
    if count > max {
      max = count;
      max_elem = vector[i];
      }
  }

  max_elem
}

fn main() {
    println!("Hello, user!");
    println!("How many elements do you want in your vector? (cannot be negative)");

  let mut elements = String::new();

  io::stdin().read_line(&mut elements).expect("Enter a number.");

  let elements: u32 = elements
    .trim()
    .parse()
    .expect("Please type in a valid number.");  

  let mut victor = build_vector(elements);

  println!("Mean: {}", get_mean(&victor));

  println!("Median: {}", get_median(&mut victor));

  println!("Mode: {}", get_mode(&mut victor));

  for x in victor {
    println!("{x}")
  }
}