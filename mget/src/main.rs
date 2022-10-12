use clap::Parser;
use smoltcp::phy::TapInterface;
use url::Url;

mod dns;
mod ethernet;
mod http;

#[derive(Parser, Debug)]
struct Args {
    url: Url,
    tap_device: String,
    dns_server: Option<String>,
}

fn main() {
    let args: Args = Args::parse();
    println!("hello: {:?}", args);

    let dns_server: String = match args.dns_server {
        Some(server) => server,
        None => "8.8.8.8".to_string(),
    };

    if args.url.scheme() != "http" || args.url.host_str().is_none() {
        eprintln!("error: only http protocol supported; host name must not be empty");
        return;
    }

    let tap = TapInterface::new(&args.tap_device).expect(&format!(
        "error: unable to use {} as a network interface",
        args.tap_device
    ));

    let addr = dns::resolve(&dns_server, &args.url.host_str().unwrap())
        .unwrap()
        .unwrap();
    println!("addr: {:?}", addr);
    let mac = ethernet::MacAddress::new().into();
    http::get(tap, mac, addr, args.url).unwrap();
}
