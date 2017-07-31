use std::env;

fn main() {
  // Try to determine the Java home directory so that we can link to `libjvm`. (only for tests)
  //
  // on mac:
  // Trying to run may give:
  //
  // ```
  // dyld: Library not loaded: @rpath/libjvm.dylib
  //  Referenced from: ./target/debug/rucaja
  //  Reason: image not found
  // Abort trap: 6
  // ```
  //
  // to fix run:
  //
  // ```
  // sudo ln -s $(/usr/libexec/java_home)/jre/lib/server/libjvm.dylib /usr/local/lib
  // ```
  if let Ok(java_home) = env::var("JAVA_HOME") {
    print!("cargo:rustc-link-search=native=");
    println!("{}/jre/lib/server", java_home);

    print!("cargo:rustc-link-search=native=");
    println!("{}/jre/lib/amd64/server", java_home);

    print!("cargo:rustc-link-search=native=");
    println!("{}/jre/lib/i386/server", java_home);
  }
}
