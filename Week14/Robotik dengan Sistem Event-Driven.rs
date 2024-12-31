use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::time::Duration;

#[derive(Debug)]
enum Event {
    ObstacleDetected,
    GoalChanged,
    Idle,
}

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    goal: (i32, i32),
}

impl Robot {
    fn new(position: (i32, i32), goal: (i32, i32)) -> Self {
        Robot { position, goal }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::ObstacleDetected => {
                println!("Obstacle detected! Adjusting path...");
                // Logic to adjust path goes here
            }
            Event::GoalChanged => {
                println!("Goal changed to: {:?}", self.goal);
                // Logic to recalculate path to new goal
            }
            Event::Idle => {
                println!("Robot is idle at position: {:?}", self.position);
            }
        }
    }

    fn move_towards_goal(&mut self) {
        if self.position != self.goal {
            if self.position.0 < self.goal.0 {
                self.position.0 += 1;
            } else if self.position.0 > self.goal.0 {
                self.position.0 -= 1;
            }

            if self.position.1 < self.goal.1 {
                self.position.1 += 1;
            } else if self.position.1 > self.goal.1 {
                self.position.1 -= 1;
            }

            println!("Moving towards goal: {:?}, current position: {:?}", self.goal, self.position);
        } else {
            println!("Robot has reached the goal at position: {:?}", self.position);
        }
    }
}

fn main() {
    let robot = Arc::new(Mutex::new(Robot::new((0, 0), (5, 5))));
    let (tx, rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();

    let robot_clone = Arc::clone(&robot);
    thread::spawn(move || {
        for event in rx {
            let mut robot = robot_clone.lock().unwrap();
            robot.handle_event(event);
        }
    });

    // Simulate events
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        tx.send(Event::ObstacleDetected).unwrap();

        thread::sleep(Duration::from_secs(2));
        tx.send(Event::GoalChanged).unwrap();
        {
            let mut robot = robot.lock().unwrap();
            robot.goal = (10, 10);
        }

        thread::sleep(Duration::from_secs(2));
        tx.send(Event::Idle).unwrap();
    });

    // Main loop to simulate robot movement
    loop {
        {
            let mut robot = robot.lock().unwrap();
            robot.move_towards_goal();
        }
        thread::sleep(Duration::from_secs(1));
    }
}
