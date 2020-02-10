use std::thread;
//use brings names into scope. We’re going to start using the thread module from the standard library, and so we need to use it.

struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is done eating.", self.name);
        thread::sleep_ms(1000);
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Baruch Spinoza"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Friedrich Nietzsche"),
        Philosopher::new("Michel Foucault"),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
          thread::spawn(move || {
              p.eat();
              //Here’s where the concurrency happens. The thread::spawn function takes a closure as an argument and executes that closure in a new thread. This closure needs an extra annotation, move, to indicate that the closure is going to take ownership of the values it’s capturing. Primarily, the p variable of the map function.
          })
      }).collect();

      for h in handles {
          h.join().unwrap();
          //At the end of main(), we loop through the handles and call join() on them, which blocks execution until the thread has completed execution. This ensures that the threads complete their work before the program exits.
    }
}
