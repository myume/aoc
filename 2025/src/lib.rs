#[macro_export]
macro_rules! read_input {
    ($filename:expr) => {{
        let filename = format!("./inputs/{}.txt", $filename);
        let file = File::open(filename).expect("Input doesn't exist");
        BufReader::new(file)
    }};
}
