use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	
	/// update interval in seconds
	#[clap(short, long, value_parser)]
	interval: u64,

	/// where to send the webhook to
	#[clap(short, long, value_parser)]
	destination: String
}

fn main() {

	// what to do
	let args = Args::parse();

	println!("Update every {} seconds, send info to '{}'", args.interval, args.destination);

	loop {

		// request
		// works 90% of the time
    	println!("Tram 22 is ded");

		// wait
		std::thread::sleep(std::time::Duration::from_secs(args.interval));
	}
}
