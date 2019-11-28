mod fetch_json;
mod selection_sort;

fn main() {
    let json_array = fetch_json::create();
    match json_array {
        Ok(v) => println!("{:?}", selection_sort::sort(v)),
        Err(e) => println!("{:?}", e),
    }
    println!("Hello, world!");
}
