mod bench_sync_write;
mod bench_async_fsync;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    match args[0].as_str() {
        "sync_write" => bench_sync_write::run_bench_sync_write(&args.as_slice()[1..]),
        "async_fsync" => bench_async_fsync::run_bench_async_fsync(&args.as_slice()[1..]),
        _ => panic!("{} argument invalid", args[0]),
    }
}
