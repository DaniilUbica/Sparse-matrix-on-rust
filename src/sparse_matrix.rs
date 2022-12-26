pub mod sparse_matrix {

    use std::cmp::Ordering;
    use queues::*;
    
      pub struct Element{
        value: i16,
        row: usize,
        column: usize,
      }
    
      pub struct Matrix{
          pub columns: usize,
          pub rows: usize,
          matrix: Vec<Vec<i16>>,
          elements: Vec<Element>,
      }
    
      impl Element {
        pub fn new(row: usize, column: usize, value: i16) -> Element {
          Element {
            row: row,
            column: column,
            value: value,
          }
        }
      }
    
      impl Matrix {
          pub fn new(r: usize, c: usize) -> Matrix {
            Matrix{
              rows: r,
              columns: c,
              matrix: vec![vec![0; c]; r],
              elements: vec![],
            }
          }
    
          pub fn print(&self){
          for i in 0..self.rows{
            for j in 0..self.columns{
              print!("{} ", self.matrix[i][j]);
            }
            println!();
          }
        }
    
        pub fn read_elements(&mut self, arr: &Vec<Element>) {
          while self.elements.len() > 0 {
            self.elements.pop().unwrap();
          }
    
          for i in 0..arr.len() {
            let element = Element{row: arr[i].row, column: arr[i].column, value: arr[i].value};
            self.elements.push(element);
          }
          self.update_matrix();
        }
        
        pub fn displace_elements(&mut self) {
          for i in 0..self.elements.len() {
            if self.elements[i].column == self.columns - 1 && self.elements[i].row == self.rows - 1 {
                self.elements[i].column = 0;
                self.elements[i].row = 0;
                continue;
            }
            if self.elements[i].column == self.columns - 1 {
              self.elements[i].column = 0;
              self.elements[i].row += 1;
              continue;
            }
            if self.elements[i].column < self.columns - 1 {
              self.elements[i].column += 1;
              continue;
            }
          }
        }

        pub fn swap_elements(&mut self, number: i16) {
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
          self.read_elements(&arr);
          self.update_matrix();
        }

        fn update_matrix(&mut self) {
          self.matrix = vec![vec![0; self.columns]; self.rows];
    
          if self.elements.len() > 0{
            for i in 0..self.elements.len() {
              if self.elements[i].row < self.rows && self.elements[i].column < self.columns {
                self.matrix[self.elements[i].row][self.elements[i].column] = self.elements[i].value;
              }
            }
          }
        }

      }
    
    }