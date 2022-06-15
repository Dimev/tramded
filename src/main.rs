use clap::Parser;
use ureq;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// update interval in seconds
    #[clap(short, long, value_parser)]
    interval: u64,

    /// where to send the webhook to
    #[clap(short, long, value_parser)]
    destination: String,
}

fn main() {
    // what to do
    let args = Args::parse();

    println!(
        "Update every {} seconds, send info to '{}'",
        args.interval, args.destination
    );

    // say that we are online
    if let Err(x) = ureq::post(&args.destination).send_json(ureq::json!({
        "name": "georg"
    })) {
        println!("Failed to send message: {}", x);
    }

    loop {
        // request
        if let Ok(x) =
            ureq::post("https://ovapi.nl/ovi/trains/departureboard?stopAreaId=NL%3AS%3Aut&lang=EN")
                .send_string("")
        {
            // send stats if needed
            if let Err(x) = ureq::post(&args.destination).send_json(ureq::json!({
                "name": "georg"
            })) {
                println!("Failed to send message: {}", x);
            }
        }

        // wait
        std::thread::sleep(std::time::Duration::from_secs(args.interval));
    }
}
