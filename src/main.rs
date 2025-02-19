use std::sync::{
    atomic::{AtomicUsize, Ordering},
    mpsc, Arc,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    let (tx, rx) = mpsc::channel();
    let incrementer = move || {
        println!("Counter: {}", counter_clone.load(Ordering::Relaxed));
        counter_clone.fetch_add(1, Ordering::Relaxed);
    };
    let ticker_object = Ticker::new(tx);
    let handle = ticker_object.start(incrementer, rx);

    thread::sleep(Duration::from_secs(3));
    ticker_object.stop();
    handle.join().unwrap();
}

struct Ticker {
    statesender: mpsc::Sender<u8>,
}
impl Ticker {
    fn new(tx: mpsc::Sender<u8>) -> Self {
        Ticker {
            // handle: None,
            statesender: tx,
        }
    }
    fn start(&self, f: impl Fn() + Send + 'static, rx: mpsc::Receiver<u8>) -> JoinHandle<()> {
        thread::spawn(move || {
            ticker(f, rx);
        })
    }
    fn stop(&self) {
        self.statesender.send(1);
    }
}

fn ticker<F>(function: F, rcv: mpsc::Receiver<u8>)
where
    F: Fn() + Send + 'static, // Ensure `f` is thread-safe
{
    loop {
        if let Ok(state) = rcv.try_recv() {
            if state == 1 {
                break;
            }
        }

        function();
        thread::sleep(Duration::from_secs(1));
    }
}
