use ::std::time::Duration;

fn main() {
    trpl::run(async {
        trpl::spawn_task(async {
            for i in 1..5 {
                println!("Hello form {i} first loop");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        })
    });
}
