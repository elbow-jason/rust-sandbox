fn main() {
  use std::thread::Thread;
  use std::sync::mpsc;

  // set up channels for comms to and from main from thread
  let (tx_to_main, rx_in_main): (mpsc::Sender<u64>, mpsc::Receiver<u64>) = mpsc::channel();
  let (tx_to_thread, rx_in_thread): (mpsc::Sender<u64>, mpsc::Receiver<u64>) = mpsc::channel();

  // spawn thread with loop and channels for tx and rx to and from fnmain
  Thread::spawn(move || {
    loop {
      let rxed_val = rx_in_thread.recv().unwrap();
      if rxed_val == 0 { break;}
      println!("rxed_val inside looped thread {}", rxed_val);
      let result = add_one(rxed_val);
      tx_to_main.send(result);
    }
    println!("exiting spawned thread");
    tx_to_main.send(0);
  });

  //uncomment this block to compare speeds of simple addition vs thread csp
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

  // initialize to 1
  let initial_val = 1u64;
  println!("initial_val outside loop {}", initial_val);
  tx_to_thread.send(initial_val);

  // main's loop
  loop {
    let rxed_val = rx_in_main.recv().unwrap();
    println!("rxed_val in main loop {}", rxed_val);
    let result = add_one(rxed_val);
    if result > 10000 { break; }
    tx_to_thread.send(result);
  }

  // stop and clean up thread
  tx_to_thread.send(0);
  let exit_code = rx_in_main.recv().unwrap();
  println!("thread exited with code {}", exit_code);
  println!("fin!!!");


}

fn add_one(num: u64) -> u64 {
  return num + 1
}

