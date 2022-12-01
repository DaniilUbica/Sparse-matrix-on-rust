use queues::*;
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

struct Element{
  value: i16,
  row: usize,
  column: usize,
}

struct Matrix{
    columns: usize,
    rows: usize,
    matrix: Vec<Vec<i16>>,
    elements: Vec<Element>,
}

impl Element {
  fn new(row: usize, column: usize, value: i16) -> Element {
    Element {
      row: row,
      column: column,
      value: value,
    }
  }
}

impl Matrix {
    fn new(r: usize, c: usize) -> Matrix {
      Matrix{
        rows: r,
        columns: c,
        matrix: vec![vec![0; c]; r],
        elements: vec![],
      }
    }

    fn print(&self){
    for i in 0..self.rows{
      for j in 0..self.columns{
        print!("{} ", self.matrix[i][j]);
      }
      println!();
    }
  }

  fn set_matrix(&mut self) {
    self.matrix = vec![vec![0; self.columns]; self.rows];

    if self.elements.len() > 0{
      for i in 0..self.elements.len() {
        if self.elements[i].row < self.rows && self.elements[i].column < self.columns {
          self.matrix[self.elements[i].row][self.elements[i].column] = self.elements[i].value;
        }
      }
    }
  }

  fn read_elements(&mut self, arr: Vec<Element>) {
    while self.elements.len() > 0 {
      self.elements.pop().unwrap();
    }

    for i in 0..arr.len() {
      let element = Element{row: arr[i].row, column: arr[i].column, value: arr[i].value};
      self.elements.push(element);
    }
  }

  fn swap_elements(&mut self, number: i16) {
    let mut bigger: Queue<&Element> = Queue::new();
    let mut less: Queue<&Element> = Queue::new();
    let mut arr: Vec<Element> = vec![];

    for i in 0..self.elements.len() {
      match &self.elements[i].value.cmp(&number) {
        Ordering::Less => less.add(&self.elements[i]),
        Ordering::Greater => bigger.add(&self.elements[i]),
        Ordering::Equal => less.add(&self.elements[i]),
      };
    }

    for i in 0..self.rows {
      for j in 0..self.columns {
        if &self.matrix[i][j] != &0 {
          if bigger.size() > 0 && less.size() >= 1 {
            let element = Element{row: i, column:j, value: bigger.peek().unwrap().value};
            arr.push(element);
            bigger.remove().ok().expect("Error in removing");
          }
          else if less.size() > 0 {
            let element = Element{row: i, column:j, value: less.peek().unwrap().value};
            arr.push(element);
            less.remove().ok().expect("Error in removing");
          }
        }
      }
    }
    self.read_elements(arr);
    self.set_matrix();
  }
}

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
  
  generate_file(100, "file.txt".to_string());

  let mut m: Matrix = Matrix::new(3, 3);
  let q = parse_file(&mut m, "file.txt".to_string());

  m.read_elements(q);
  m.set_matrix();
  let start = Instant::now();
  m.swap_elements(30);
  let time = start.elapsed();
  println!("Time elapsed in expensive_function() is: {:?}", time);
  //m.print();  
  println!("{}", std::mem::size_of::<Element>());
}