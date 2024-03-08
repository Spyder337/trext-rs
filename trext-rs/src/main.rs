use common::{App, Program};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = App::new();
    let mut p = Program::new(a);
    p.run().await
}
