use std::thread;
use std::time::Duration;

fn main() {
    let mut counter = 0;
    let incrementer = || {
        println!("{}", counter);
        counter += 1
    };
    ticker(incrementer);
}

fn ticker<F>(mut function: F)
where
    F: FnMut(),
{
    loop {
        function();
        thread::sleep(Duration::from_secs(1));
    }
}
