fn main() {}

// let url = "http://localhost:9933";

// let output = Command::new("cargo")
//     .arg("install")
//     .arg("subxt-cli")
//     .output()
//     .expect("failed to execute process");

// println!("status: {}", output.status);

// io::stdout().write_all(&output.stdout).unwrap();
// io::stderr().write_all(&output.stderr).unwrap();

// assert!(output.status.success());

// subxt metadata -f bytes --url http://localhost:8545 > metadata.scale
// generate metadata
// let output = Command::new("subxt")
//     .arg("metadata")
//     .arg("-f")
//     .arg("bytes")
//     .arg("--url")
//     .arg(url)
//     .arg(">")
//     .output()
//     .expect("failed to execute process");

// let metadata_path = Path::new("./metadata/substrate-node-template-metadata.scale");
// fs::write(metadata_path,&output.stdout).unwrap();

// subxt codegen --url http://localhost:9933 | rustfmt --edition=2018 --emit=stdout > generate.rs
// generate codegen file
// let output = Command::new("subxt")
//     .arg("codegen")
//     .arg("--url")
//     .arg(url)
//     .arg("rustfmt")
//     .arg("--edition=2018")
//     .arg("--emit=stdout")
//     .arg(">")
//     .arg("./codegen/substrate-node-template-metadata.rs")
//     .output()
//     .expect("failed to execute process");

// let codegen_path = Path::new("./codegen/substrate-node-template-metadata.rs");
// fs::write(codegen_path,&output.stdout).unwrap();
// io::stdout().write_all(&output.stdout).unwrap();

// let ret = Command::new("ls")
//     .arg("-la")
//     .current_dir(".")
//     .spawn()
//     .expect("ls command failed to start");
