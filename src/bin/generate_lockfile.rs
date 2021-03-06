use std::env;

use cargo::ops;
use cargo::util::{CliResult, CliError, Config};
use cargo::util::important_paths::find_root_manifest_for_cwd;

#[derive(RustcDecodable)]
struct Options {
    flag_manifest_path: Option<String>,
    flag_verbose: bool,
    flag_quiet: bool,
}

pub const USAGE: &'static str = "
Generate the lockfile for a project

Usage:
    cargo generate-lockfile [options]

Options:
    -h, --help              Print this message
    --manifest-path PATH    Path to the manifest to generate a lockfile for
    -v, --verbose           Use verbose output
    -q, --quiet             No output printed to stdout
";

pub fn execute(options: Options, config: &Config) -> CliResult<Option<()>> {
    debug!("executing; cmd=cargo-generate-lockfile; args={:?}", env::args().collect::<Vec<_>>());
    try!(config.shell().set_verbosity(options.flag_verbose, options.flag_quiet));
    let root = try!(find_root_manifest_for_cwd(options.flag_manifest_path));

    ops::generate_lockfile(&root, config)
        .map(|_| None).map_err(|err| CliError::from_boxed(err, 101))
}
