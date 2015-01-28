fn main() {
  use std::thread::Thread;
  use std::sync::mpsc;

  let (tx_to_main, rx_in_main): (mpsc::Sender<u64>, mpsc::Receiver<u64>) = mpsc::channel();
  let (tx_to_thread, rx_in_thread): (mpsc::Sender<u64>, mpsc::Receiver<u64>) = mpsc::channel();

  Thread::spawn(move || {
    loop {
      let rxed_val = rx_in_thread.recv().unwrap();
      if rxed_val > 1000 {break;}
      println!("rxed_val inside looped thread {}", rxed_val);
      let result = add_one(rxed_val);
      tx_to_main.send(result);
    }
  });

  //uncomment this block to compare speeds
  /* 
  Thread::spawn(move || {
    let mut x: u64 = 1;
    loop {
      if x > 1000 {break;}
      println!("counter {}", x);
      x = x + 1;
    }
  });
  */

  let initial_val = 1u64;
  println!("initial_val outside loop {}", initial_val);
  tx_to_thread.send(initial_val);

  loop {
    let rxed_val = rx_in_main.recv().unwrap();
    println!("rxed_val in main loop {}", rxed_val);
    let result = add_one(rxed_val);
    if result > 1000 { break; }
    tx_to_thread.send(result);
  }
  tx_to_thread.send(1001);
  println!("fin");


}

fn add_one(num: u64) -> u64 {
  return num + 1
}

