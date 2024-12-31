use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::time::Duration;
use rand::Rng; // Untuk simulasi probabilistik

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
    sensor_uncertainty: f64, // Ketidakpastian data sensor
}

impl Robot {
    fn new(position: (i32, i32), goal: (i32, i32), sensor_uncertainty: f64) -> Self {
        Robot { position, goal, sensor_uncertainty }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::ObstacleDetected => {
                println!("Obstacle detected! Adjusting path probabilistically...");
                self.avoid_obstacle();
            }
            Event::GoalChanged => {
                println!("Goal changed to: {:?}", self.goal);
            }
            Event::Idle => {
                println!("Robot is idle at position: {:?}", self.position);
            }
        }
    }

    fn avoid_obstacle(&mut self) {
        let mut rng = rand::thread_rng();
        let direction = rng.gen_range(0..4);

        match direction {
            0 => self.position.0 += 1, // Gerak ke bawah
            1 => self.position.0 -= 1, // Gerak ke atas
            2 => self.position.1 += 1, // Gerak ke kanan
            _ => self.position.1 -= 1, // Gerak ke kiri
        }

        println!("New position after avoiding obstacle: {:?}", self.position);
    }

    fn move_towards_goal(&mut self) {
        let mut rng = rand::thread_rng();

        if self.position != self.goal {
            // Simulasikan ketidakpastian sensor
            let noise: f64 = rng.gen_range(-self.sensor_uncertainty..self.sensor_uncertainty);

            if self.position.0 < self.goal.0 {
                self.position.0 = ((self.position.0 as f64 + 1.0 + noise).round()) as i32;
            } else if self.position.0 > self.goal.0 {
                self.position.0 = ((self.position.0 as f64 - 1.0 + noise).round()) as i32;
            }

            if self.position.1 < self.goal.1 {
                self.position.1 = ((self.position.1 as f64 + 1.0 + noise).round()) as i32;
            } else if self.position.1 > self.goal.1 {
                self.position.1 = ((self.position.1 as f64 - 1.0 + noise).round()) as i32;
            }

            println!("Moving towards goal: {:?}, current position: {:?}", self.goal, self.position);
        } else {
            println!("Robot has reached the goal at position: {:?}", self.position);
        }
    }
}

fn main() {
    let robot = Arc::new(Mutex::new(Robot::new((0, 0), (5, 5), 0.2)));
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
