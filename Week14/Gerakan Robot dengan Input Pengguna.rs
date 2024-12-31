use std::io;

fn main() {
    let mut robot_position = (0, 0); // Posisi awal robot

    loop {
        println!("Posisi robot saat ini: {:?}", robot_position);
        println!("Masukkan perintah untuk menggerakkan robot (w: atas, s: bawah, a: kiri, d: kanan, q: keluar):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");
        let command = input.trim();

        match command {
            "w" => robot_position.0 = robot_position.0.saturating_sub(1), // Gerak ke atas
            "s" => robot_position.0 = robot_position.0.saturating_add(1), // Gerak ke bawah
            "a" => robot_position.1 = robot_position.1.saturating_sub(1), // Gerak ke kiri
            "d" => robot_position.1 = robot_position.1.saturating_add(1), // Gerak ke kanan
            "q" => {
                println!("Keluar dari program.");
                break;
            }
            _ => println!("Perintah tidak dikenali. Gunakan w, s, a, d, atau q."),
        }

        // Batasan area agar robot tetap dalam grid 5x5
        robot_position.0 = robot_position.0.clamp(0, 4);
        robot_position.1 = robot_position.1.clamp(0, 4);
    }
}
