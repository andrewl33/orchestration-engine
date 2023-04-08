// WorkItem is a base unit of work on the work engine
#[derive(Debug)]
pub struct WorkItem {
    id: u64,
    name: String,
    description: Option<String>,
    status: WorkItemStatus,
    work: Box<dyn Work>,
}

// WorkItemStatus is the status of a work item
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum WorkItemStatus {
    // WorkItemStatus::NotStarted is returned when the work item has not started
    NotStarted,
    // WorkItemStatus::InProgress is returned when the work item is in progress
    InProgress,
    // WorkItemStatus::Complete is returned when the work item is complete
    Complete,
    // WorkItemStatus::Error is returned when the work item has errored
    Error(WorkError),
}

// Work is a unit of work that can be performed
pub trait Work: std::fmt::Debug{
    // execute performs the work
    fn execute(&self) -> Result<(), WorkError>;
    // status returns the status of the work
    fn status(&self) -> WorkStatus;
}

// WorkStatus is the status of a work
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum WorkStatus {
    // WorkStatus::NotStarted is returned when the work has not started
    NotStarted,
    // WorkStatus::InProgress is returned when the work is in progress
    InProgress,
    // WorkStatus::Complete is returned when the work is complete
    Complete,
}

// WorkError is an error that can occur during work
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum WorkError {
    // WorkError::NotImplemented is returned when a work item is not implemented
    NotImplemented,
    // WorkError::Unknown is returned when an unknown error occurs
    Unknown,
    // WorkError::Unrecoverable is returned when an unrecoverable error occurs 
    Unrecoverable,
    // WorkError::Recoverable is returned when a recoverable error occurs
    Recoverable,
}

// WorkEngine is the engine that executes work and tracks the status of work items
pub struct WorkEngine {
    // work_items is a vector of active work items
    work_items: Vec<WorkItem>,
    // completed_work_items is a vector of completed work items
    completed_work_items: Vec<WorkItem>,
    // work_item_counter is a counter for work items
    work_item_counter: u64,
    // stop is a flag that indicates whether the work engine should stop
    stop: bool,
}

impl WorkEngine {
    // new creates a new work engine
    pub fn new() -> WorkEngine {
        WorkEngine {
            // work_items is a vector of active work items
            work_items: Vec::new(),
            // completed_work_items is a vector of completed work items
            completed_work_items: Vec::new(),
            // work_item_counter is a counter for work items
            work_item_counter: 0,
            stop: false,
        }
    }



    // run executes all work items in the work engine and cleans up completed work items usi
    pub fn run(&mut self) -> Result<(), WorkError> {
        while !self.stop {
            let ret = self.work_items.iter_mut().
            filter(|work_item| work_item.status == WorkItemStatus::NotStarted).
            try_for_each(|work_item| { 
                if work_item.work.execute().is_ok() {
                    work_item.status = WorkItemStatus::InProgress;
                    Ok(())
                } else {
                    Err(WorkError::Unknown)
                }
            });
        // short circuit for now
        if !ret.is_ok() {
            return ret;
        }

        // check on status of in progress work items
        let ret = self.work_items.iter_mut().
            filter(|work_item| work_item.status == WorkItemStatus::InProgress).
            try_for_each(|work_item| { 
                if work_item.work.status() == WorkStatus::Complete {
                    work_item.status = WorkItemStatus::Complete;   
                }
                Ok(())
            });

       // short circuit for now
        if !ret.is_ok() {
            return ret;
        }

        self.move_completed_work_items();
        }
        
        Ok(())
    }
    
    // stop stops the work engine
    pub fn stop(&mut self) {
        self.stop = true;
    }

    // print_work_items prints all work items in both the work items vec and the completed work items vec
    pub fn print_work_items(&self) {
        println!("Work Items:");
        for work_item in &self.work_items {
            println!("{:?}", work_item);
        }
        println!("Completed Work Items:");
        for work_item in &self.completed_work_items {
            println!("{:?}", work_item);
        }
    }

    // add creates a work item from a struct with the trait Work and adds it to the work engine and returns its id
    pub fn add<T: Work + 'static>(&mut self, name: String, description: Option<String>, work: T) -> u64 {
        self.add_work_item(WorkItem {
            id: self.work_item_counter,
            name,
            description,
            status: WorkItemStatus::NotStarted,
            work: Box::new(work),
        });
        self.work_item_counter += 1;
        self.work_item_counter - 1
    }
    

    // add_work_item adds a work item to the work engine
    fn add_work_item(&mut self, work_item: WorkItem) {
            self.work_items.push(work_item);
    }

    // move_completed_work_items appends completed work items to the completed work items vec and deletes them from the work items vec
    fn move_completed_work_items(&mut self) {
        for i in 0..self.work_items.len() {
            if self.work_items[i].status == WorkItemStatus::Complete {
                let work_item = self.work_items.remove(i);
                self.completed_work_items.push(work_item);
            }
        }
    }
}

