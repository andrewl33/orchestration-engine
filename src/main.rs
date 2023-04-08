use std::thread;
use work::WorkError;


mod work {
    include!("work.rs");
}

fn main() {
    println!("Hello, world!");
}

// SimpleWork is a unit of work that implements the Work trait
// execute waits 5 seconds and returns ok
struct SimpleWork {
    id: u64,
    status: work::WorkStatus,
}

impl work::Work for SimpleWork {
    fn execute(&self) -> Result<(), WorkError> {
        println!("SimpleWork {} executing", self.id);
        thread::sleep(std::time::Duration::from_secs(2));
        println!("SimpleWork {} complete", self.id);
        Ok(())
    }

    fn status(&self) -> work::WorkStatus {
        self.status
    }
}

impl std::fmt::Debug for SimpleWork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimpleWork {{ id: {} }}", self.id)
    }
}