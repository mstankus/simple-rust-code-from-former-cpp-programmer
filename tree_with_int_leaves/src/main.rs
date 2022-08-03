// Mark Stankus (c) Wed Aug  3 14:52:47 PDT 2022

// There can be "extra commas" in the output via lisp_print with 
// this implementation. Making it simple.

enum Tree {
  Leaf(i32),
  Branch(Vec<Tree>),
}

fn print_n_spaces(n : usize) {
  for _ in 0..n {
    print!(" ");
  }
}

impl Tree {
  fn lisp_print(&self) {
    match self {
      Tree::Leaf(a) => { 
        print!("{},",a) 
      }
      Tree::Branch(e) => { 
        print!("(");
        for i in 0..e.len() {
          e[i].lisp_print();
        }
        print!(")");
      }
    }
  }
  fn indent_print(&self,n : usize) {
    match self {
      Tree::Leaf(a) => { 
        print_n_spaces(n);
        println!("{}",a);
      }
      Tree::Branch(e) => { 
        print_n_spaces(n);
        println!("(");
        for i in 0..e.len() {
          e[i].indent_print(n+2);
        }
        print_n_spaces(n);
        println!(")");
      }
    }
  }
}

fn main() {
  let x : Tree = Tree::Leaf(5);
  let y : Tree = Tree::Leaf(17);
  let z : Tree = Tree::Leaf(-3);
  let w : Tree = Tree::Branch(vec![x,y,z]);
  println!("You should see (5,17,-3,) just below.");
  w.lisp_print();
  println!("\n");

  println!("And now simply indentated!");
  let t : Tree = Tree::Leaf(5);
  let u : Tree = Tree::Leaf(17);
  let v : Tree = Tree::Branch(vec![t,u]);
  let w : Tree = Tree::Leaf(-3);
  let x : Tree = Tree::Branch(vec![v,w]);
  x.indent_print(0);
  println!("\n");

  println!("And now with extra 10 spaces of indentation!");
  x.indent_print(10);
}
