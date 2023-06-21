use std::{env, fs, io::Write, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("timestamp.txt");

    let mut f = fs::File::create(dest_path).unwrap();
    // write!(f, r#""{}""#, time::OffsetDateTime::now_utc()).unwrap();
    write!(f, r#""{}""#, chrono::Utc::now().to_rfc3339()).unwrap();
}
