// This might give you a warning like this
// warning: variable does not need to be mutable
//  --> /tmp/23CMfFF3Ry/main.rs:5:10
//   |
// 5 |      let mut x = 19;
//   |          ----^
//   |          |
//   |          help: remove this `mut`
//   |
//   = note: `#[warn(unused_mut)]` on by default

// warning: 1 warning emitted

fn main() {
     let mut x = 19;
     println!("{x}");
}
