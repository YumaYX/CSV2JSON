fn main() {
    if let Err(e) = csv2json::get_args().and_then(csv2json::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
