mod bench_sync_write;
mod bench_async_fsync;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "sync_write" => bench_sync_write::run_bench_sync_write(&args.as_slice()[2..]),
        "async_fsync" => bench_async_fsync::run_bench_async_fsync(&args.as_slice()[2..]),
        _ => panic!("{} argument invalid", args[1]),
    }
}
