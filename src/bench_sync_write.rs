use std::{fs::File, io::Write, sync::atomic::AtomicUsize, time::{Duration, Instant}};
use std::vec;
use rand::Rng;

const FILE_NAME: &str = "test_write";
static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn benchmark_ops(data: Vec<u8>, file: String, seconds: u32) {
    let mut file = File::create(&file).unwrap();
    let timer = Instant::now();
    while timer.elapsed() < Duration::from_secs(seconds as u64) {
        file.write(data.as_slice()).unwrap();
        file.sync_data().unwrap();
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }
}

pub fn run_bench_sync_write(args: &[String]) {
    println!("{:?}", args);
    if args.len() < 4 {
        panic!("argument: thread num, data size, file directory, run seconds");
    }
    let thread_num = args[0].parse::<u32>().unwrap();
    let data_size = args[1].parse::<usize>().unwrap();
    let mut data = Vec::with_capacity(data_size);
    for _ in 0..data_size {
        let random: u32 = rand::thread_rng().gen();
        data.push((random % 26) as u8 + 'A' as u8);
    }
    let file_directory = args[2].clone();
    let seconds = args[3].parse::<u32>().unwrap();
    let mut handlers = vec![];
    for i in 0..thread_num {
        let file = file_directory.clone() + "/" + FILE_NAME + "_" + &i.to_string();
        let tmp_data = data.clone();
        handlers.push(std::thread::spawn(move || benchmark_ops(tmp_data, file, seconds)));
    }
    let timer = Instant::now();
    let mut trigger = 10;
    let mut previous_count = 0;
    loop {
        let elapsed = timer.elapsed();
        if elapsed >= Duration::from_secs(trigger) {
            let count = COUNTER.load(std::sync::atomic::Ordering::Acquire);
            println!("[{}s] count {}", trigger, count - previous_count);
            trigger += 10;
            previous_count = count;  
        }
        if elapsed >= Duration::from_secs(seconds as u64) {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let count = COUNTER.load(std::sync::atomic::Ordering::Acquire);
    println!("test end {}s total count {}", seconds, count);
    for h in handlers {
        h.join().unwrap();
    }
}