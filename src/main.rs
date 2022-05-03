use std::fmt::Display;
use num::{Num, NumCast};
use rand::Rng;

struct Node<T: Num + NumCast + PartialOrd + Display + Copy> {
  value: T,
  left: Option<Box<Node::<T>>>,
  right: Option<Box<Node::<T>>>
}

impl<T: Num + NumCast + PartialOrd + Display + Copy> Node<T> {
  pub fn insert(&mut self, value: T) {
    if self.value < value {
      if self.left.is_some() {
        self.left.as_mut().unwrap().insert(value);
      }
      else {
        self.left = Some(Box::new(Node{value, left: None, right: None}));
      }
    }
    else {
      if self.right.is_some() {
        self.right.as_mut().unwrap().insert(value);
      }
      else {
        self.right = Some(Box::new(Node{value, left: None, right: None}));
      }
    }
  }

  pub fn has(&self, value: T) -> bool {
    if self.value == value {
      return true;
    }

    if self.value < value {
      if self.left.is_some() {
        return self.left.as_ref().unwrap().has(value);
      }
    }
    else {
      if self.right.is_some() {
        return self.right.as_ref().unwrap().has(value);
      }
    }

    return false;
  }
  
  pub fn inorder(&self, output: &mut Vec<T>) {
    if self.left.is_some() {
      self.left.as_ref().unwrap().inorder(output);
    }

    output.push(self.value);

    if self.right.is_some() {
      self.right.as_ref().unwrap().inorder(output);
    }
  }

  pub fn draw(&self, space: u16) {
    let count = 5;
    let space = space + count;
  
    if self.left.is_some() {
      self.left.as_ref().unwrap().draw(space);
    }

    println!();
    for _ in count..space {
      print!(" ");
    }
    print!("{} ", self.value);

    if self.right.is_some() {
      self.right.as_ref().unwrap().draw(space);
    }
  }

  pub fn height(&self) -> i32 {
    let lh = match &self.left {
      Some(n) => n.value,
      None => T::zero()
    };

    let rh = match &self.right {
      Some(n) => n.value,
      None => T::zero()
    };

    if lh >= rh {
      return lh.to_i32().unwrap() + 1;
    }
    else {
      return rh.to_i32().unwrap() + 1;
    }
  }
  
  pub fn is_balanced(&self) -> bool {
    let lh = match &self.left {
      Some(n) => (n.height(), n.is_balanced()),
      None => (0, true)
    };
    
    let rh = match &self.right {
      Some(n) => (n.height(), n.is_balanced()),
      None => (0, true)
    }; 
    
    return (lh.0-rh.0).abs() <= 1 && lh.1 && rh.1;
  }
}

fn main() {
  println!("Hello, world!");

  let mut rng = rand::thread_rng();

  let mut root = Node{value: 0.5, left: None, right: None};
  for _ in 0..10 {
    root.insert(rng.gen_range(0f64..1f64));
  }

  root.draw(0);
  println!();
  
  let mut vec: Vec<f64> = Vec::new();
  root.inorder(&mut vec);
  
  println!("{:?}", vec);
  println!("{}", root.is_balanced());
  println!("{}", root.has(0.5f64));
}
