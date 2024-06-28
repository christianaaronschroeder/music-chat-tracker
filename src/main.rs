use message_exporter::add as me_add;
use playlist_builder::add as pb_add;

fn main() {
    println!("2 + 2 = {}", me_add(2, 2));
    println!("2 + 2 = {}", pb_add(2, 2));
}
