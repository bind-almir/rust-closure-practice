use std::collections::HashMap;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    run(None);
}

fn run(test: Option<String>) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    loop {
        let mut input = String::new();
        let mut is_test = false;
        match test {
            Some(ref arg) => {
                is_test = true;
                input = arg.clone()
            },
            None => {
                println!("Please enter a number: ");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
            }
        }
        let num = input.trim().parse::<u32>().unwrap();
        println!("{}", expensive_result.value(num));
        if num == 0 || is_test{
            break;
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};
    #[test]    
    fn test_cache() {
        let instant = Instant::now();
        let two_sec = Duration::from_secs(1);
        let fifty_ms = Duration::from_millis(50);
        run(Some("1".to_string()));
        assert!(instant.elapsed() >= two_sec);
        let instant = Instant::now();        
        run(Some("1".to_string()));
        assert!(instant.elapsed() < fifty_ms);
    }
}
