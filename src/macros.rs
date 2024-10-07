#[macro_export]
macro_rules! parseerr {
    ($line:expr,$short:expr ,$err_msg:expr, $reason:expr) => {
        eprintln!("\x1b[31mParse error at line {}\x1b[0m", $line);
        eprintln!("{}",$short);
        eprintln!("{}", $err_msg);
        eprintln!("Forced termination: {}", $reason);
        std::process::exit(1);
    };
}
