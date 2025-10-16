use anyhow::Result;

pub async fn run(action: &str) -> Result<()> {
    match action {
        "status" => {
            println!("circlectl system status: (stub) All systems nominal");
        }
        _ => {
            println!("unknown system action: {}", action);
        }
    }
    Ok(())
}
