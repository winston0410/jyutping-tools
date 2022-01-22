use actix_web::http::header::LastModified;
use std::time::{Duration, SystemTime};
use std::{env, fs, io::Write};

fn main() {
    let outdir = "/tmp";
    let outfile = format!("{}/timestamp.txt", outdir);

    let current_time = SystemTime::now();
    let build_time = LastModified(current_time.into());

    let mut fh = fs::File::create(&outfile).unwrap();
    write!(fh, r#""{}""#, build_time).ok();
}
