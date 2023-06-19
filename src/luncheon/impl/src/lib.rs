declare_log_buffer!(name = INFO, capacity = 1000);
declare_log_buffer!(name = ERROR, capacity = 1000);

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
