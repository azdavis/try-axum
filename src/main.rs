fn main() {
  println!("starting up");
  let mut i = 0usize;
  loop {
    println!("{}", i);
    i += 1;
    std::thread::sleep(std::time::Duration::from_secs(1));
  }
}
