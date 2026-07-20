use tokio::sync::broadcast;
fn main() {
    let (tx, mut rx1) = broadcast::channel::<u8>(10);
    // let rx2 = rx1.clone(); // Does this compile?
}
