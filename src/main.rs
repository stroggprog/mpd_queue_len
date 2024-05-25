extern crate mpd;

use mpd::Client;

const HOST: &str = "romeo:6600";

fn secs_to_time(t: u64) -> String {
    let seconds = t % 60;
    let minutes = (t / 60) % 60;
    let hours = (t / 60) / 60;
    format!("{:>02}:{:>02}:{:>02}", hours, minutes, seconds)
}

fn main(){
    let mut conn = Client::connect(HOST).unwrap();
    let mut duration: u64 = 0; // u64 because that's what duration unwraps to
    let p = conn.queue().unwrap();
    for item in p {
        duration += item.duration.unwrap().as_secs();
    }
    println!("{}", secs_to_time(duration));
}
