use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use queues::*;

mod sparse_matrix;

use sparse_matrix::sparse_matrix::*;

fn parse_file(matrix: &mut Matrix, file_name: String) -> Vec<Element> {

    let mut file = File::open(file_name).expect("file not found!");

    let mut ret: Queue<i16> = queue![];
    let mut buffer = String::new();

    let mut data: Vec<Element> = vec![];

    let mut row: usize;
    let mut column: usize;
    let mut value: i16;

    file.read_to_string(&mut buffer).ok().expect("Error in reading string");
    
    let mut info: Vec<&str> = buffer.split(' ').collect();
    
    info.pop();

    matrix.rows = info[0].trim().parse().ok().expect("Error in converting!");
    matrix.columns = info[1].trim().parse().ok().expect("Error in converting!");

    for i in 2..info.len() {
      ret.add(info[i].trim().parse().ok().expect("Error in converting!")).ok().expect("Error in add");
    }

    while ret.size() > 1 {
      row = ret.peek().unwrap() as usize;
      ret.remove().ok().expect("Error in removing");
      column = ret.peek().unwrap() as usize;
      ret.remove().ok().expect("Error in removing");
      value = ret.peek().unwrap();
      ret.remove().ok().expect("Error in removing");
      let temp = Element::new(row, column, value);
      data.push(temp);
    }

    data
}

fn generate_file (size: i32, file_name: String) {
  let mut file =  File::create(file_name).ok().expect("Error in creating file!");
  let rows: i32 = rand::thread_rng().gen_range(1, 10000);
  let columns: i32 = rand::thread_rng().gen_range(1, 10000);
  let mut s = size * 3;
  write!(file, "{} ", rows).ok().expect("Error in writing");
  write!(file, "{} ", columns).ok().expect("Error in writing");
  while s > 0 {
    s -= 1;
    let num: i32 = rand::thread_rng().gen_range(1, 1000);
    write!(file, "{} ", num).ok().expect("Error in writing");
  }
}

fn main() {
  
  generate_file(3, "file.txt".to_string());

  let mut m: Matrix = Matrix::new(3, 3);
  let q = parse_file(&mut m, "file.txt".to_string());

  m.read_elements(q);
  m.swap_elements(30);
  m.print();  
}