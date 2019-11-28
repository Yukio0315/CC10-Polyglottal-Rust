mod fetch_json;
mod selection_sort;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let json_array = fetch_json::create();
    match json_array {
        Ok(v) => {
            let mut val = v;
            let ref_val: &mut [i32] = &mut val;
            selection_sort::sort(ref_val);
            println!("{:?}", ref_val);
        }
        Err(e) => panic!("{}", e),
    }
    let end = start.elapsed();
    println!(
        "processing time {}.{:03}",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    )
}
