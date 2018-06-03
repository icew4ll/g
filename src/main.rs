// imports {{{
#![allow(unused_must_use)]
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate quicli;
#[macro_use]
extern crate duct;
// }}}
// types {{{
use quicli::prelude::*;
// }}}
// structs {{{
/// IceSync
#[derive(Debug, StructOpt)]
struct Cli {
    /// gitvim, gitdot, gitrgit, pulldot, gitall
    command: String,
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

// }}}
// main {{{
main!(|args: Cli, log_level: verbosity| match args.command.as_ref() {
    "hd1" => rd(dotenv!("U1").to_string(), dotenv!("P3").to_string(), r"216.230.241.165".to_string()),
    "hd3" => proxy(dotenv!("U1").to_string(), dotenv!("P1").to_string(), r"172.21.241.238".to_string()),
    "di1" => rd(dotenv!("U1").to_string(), dotenv!("P4").to_string(), r"216.230.241.238".to_string()),
    "di2" => rd(dotenv!("U1").to_string(), dotenv!("P4").to_string(), r"216.230.241.126".to_string()),
    "di3" => rd(dotenv!("U1").to_string(), dotenv!("P4").to_string(), r"216.230.241.48".to_string()),
    "di4" => rd(dotenv!("U1").to_string(), dotenv!("P4").to_string(), r"216.230.241.20".to_string()),
    // "am" => exp(r"am".to_string()),
    "psan" => rd(dotenv!("U1").to_string(), dotenv!("P5").to_string(), r"216.230.243.51".to_string()),
    "osan" => rd(dotenv!("U2").to_string(), dotenv!("P6").to_string(), r"216.230.254.80".to_string()),
    "rdhv" => rd(dotenv!("U1").to_string(), dotenv!("P3").to_string(), r"216.230.240.15".to_string()),
    "high" => rd(dotenv!("U1").to_string(), dotenv!("P1").to_string(), r"216.230.241.143".to_string()),
    "amo5" => ssh(dotenv!("U3").to_string(), dotenv!("P2").to_string(), r"216.230.254.45".to_string()),
    "amo6" => ssh(dotenv!("U3").to_string(), dotenv!("P2").to_string(), r"216.230.254.46".to_string()),
    "amo7" => ssh(dotenv!("U3").to_string(), dotenv!("P2").to_string(), r"216.230.254.47".to_string()),
    "amo8" => ssh(dotenv!("U3").to_string(), dotenv!("P2").to_string(), r"216.230.254.48".to_string()),
    "amo9" => ssh(dotenv!("U4").to_string(), dotenv!("P1").to_string(), r"216.230.254.49".to_string()),
    "amo10" => ssh(dotenv!("U4").to_string(), dotenv!("P1").to_string(), r"216.230.254.50".to_string()),
    "smtp2" => ssh(dotenv!("U3").to_string(), dotenv!("P1").to_string(), r"216.230.240.17".to_string()),
    "ns1" => ssh(dotenv!("U3").to_string(), dotenv!("P1").to_string(), r"216.230.241.2 ".to_string()),
    "am" => ssh(dotenv!("U5").to_string(), dotenv!("P1").to_string(), r"216.230.243.216".to_string()),
    _ => println!("none"),
});

fn proxy(user: String, pass: String, ip: String) {
    let command = format!("ssh -f -N -D9050 pspinc@216.230.243.216; proxychains rdesktop -u {} -p '{}' {}", user, pass, ip);
    cmd!("sh", "-c", command).run();
}

fn rd(user: String, pass: String, ip: String) {
    let command = format!("rdesktop -g 1280x960 -5 -K -r clipboard:CLIPBOARD -u {} -p '{}' {}", user, pass, ip);
    // requires #![allow(unused_must_use)] to not return error with .unwrap()
    cmd!("sh", "-c", command).run();
}

fn ssh(user: String, pass: String, ip: String) {
    if user == "psp" {
        let command = format!("expect -c 'spawn ssh {}@{};expect \"password\";send \"{}\n\";expect \"psp\";send \"sudo su -\n\";expect \"password\";send \"{}\n\";interact'", user, ip, pass, pass);
        cmd!("sh", "-c", command).run();
    } else if ip == "216.230.243.216" {
        // ambk10
        let command = format!("expect -c 'spawn ssh {}@{};expect \"pspinc\";send \"sudo su -\n\";expect \"password\";send \"{}\n\";interact'", user, ip, pass);
        cmd!("sh", "-c", command).run();
    } else {
        let command = format!("expect -c 'spawn ssh {}@{};expect \"password\";send \"{}\n\";interact'", user, ip, pass);
        cmd!("sh", "-c", command).run();
    }
}
