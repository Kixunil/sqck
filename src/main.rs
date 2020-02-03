fn main() {
    use sequoia_openpgp::parse::Parse;
    use sequoia_openpgp::serialize::Serialize;

    let mut args = std::env::args();
    let program_name = args.next().expect("Not even program name given");
    let fingerprint = args.next().unwrap_or_else(|| {
        eprintln!("Usage: {} FINGERPRINT", program_name);
        std::process::exit(2);
    });
    let fingerprint = sequoia_openpgp::Fingerprint::from_hex(&fingerprint)
        .unwrap_or_else(|err| {
            eprintln!("Failed to parse fingerprint: {}", err);
            std::process::exit(2);
        });
    let input = std::io::stdin();
    let input = input.lock();
    let input = std::io::BufReader::new(input);
    let key = sequoia_openpgp::Cert::from_reader(input)
        .unwrap_or_else(|err| {
            eprintln!("Failed to load key: {}", err);
            std::process::exit(2);
        });

    if key.fingerprint() != fingerprint {
        eprintln!("Fingerprints don't match");
        std::process::exit(1);
    }

    let output = std::io::stdout();
    let output = output.lock();
    let mut output = std::io::BufWriter::new(output);

    key.export(&mut output).unwrap_or_else(|err| {
        eprintln!("Failed to write the key: {}", err);
        std::process::exit(3);
    })
}
