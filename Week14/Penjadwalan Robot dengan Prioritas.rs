use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug)]
struct Task {
    priority: usize,
    description: String,
}

impl Task {
    fn new(priority: usize, description: &str) -> Self {
        Task {
            priority,
            description: description.to_string(),
        }
    }
}

fn main() {
    // Antrean tugas robot dengan prioritas
    let mut task_queue = BinaryHeap::new();

    // Tambahkan tugas ke antrean
    task_queue.push(Reverse(Task::new(3, "Ambil sampel udara")));
    task_queue.push(Reverse(Task::new(1, "Matikan alat sensor")));
    task_queue.push(Reverse(Task::new(2, "Isi ulang daya")));

    println!("Mulai menyelesaikan tugas berdasarkan prioritas:");

    while let Some(Reverse(task)) = task_queue.pop() {
        println!("Menyelesaikan tugas: {} dengan prioritas {}", task.description, task.priority);
    }
}
