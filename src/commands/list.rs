#[derive(Parser, Debug)]
pub struct ListCommand {}

impl ListCommand {
    pub async fn run(self) -> Result<()> {
        println!("Listing all installed versions");
        ok(())
    }
}
