#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Lets hack {}!", name)
}
