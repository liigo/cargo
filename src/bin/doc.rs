use cargo::ops;
use cargo::util::{CliResult, CliError, Config};
use cargo::util::important_paths::{find_root_manifest_for_cwd};

#[derive(RustcDecodable)]
struct Options {
    flag_target: Option<String>,
    flag_features: Vec<String>,
    flag_jobs: Option<u32>,
    flag_manifest_path: Option<String>,
    flag_no_default_features: bool,
    flag_no_deps: bool,
    flag_open: bool,
    flag_verbose: bool,
    flag_quiet: bool,
    flag_package: Option<String>,
}

pub const USAGE: &'static str = "
Build a package's documentation

Usage:
    cargo doc [options]

Options:
    -h, --help               Print this message
    --open                   Opens the docs in a browser after the operation
    -p SPEC, --package SPEC  Package to document
    --no-deps                Don't build documentation for dependencies
    -j N, --jobs N           The number of jobs to run in parallel
    --features FEATURES      Space-separated list of features to also build
    --no-default-features    Do not build the `default` feature
    --target TRIPLE          Build for the target triple
    --manifest-path PATH     Path to the manifest to document
    -v, --verbose            Use verbose output
    -q, --quiet              No output printed to stdout

By default the documentation for the local package and all dependencies is
built. The output is all placed in `target/doc` in rustdoc's usual format.

If the --package argument is given, then SPEC is a package id specification
which indicates which package should be documented. If it is not given, then the
current package is documented. For more information on SPEC and its format, see
the `cargo help pkgid` command.
";

pub fn execute(options: Options, config: &Config) -> CliResult<Option<()>> {
    try!(config.shell().set_verbosity(options.flag_verbose, options.flag_quiet));

    let root = try!(find_root_manifest_for_cwd(options.flag_manifest_path));

    let mut doc_opts = ops::DocOptions {
        open_result: options.flag_open,
        compile_opts: ops::CompileOptions {
            config: config,
            jobs: options.flag_jobs,
            target: options.flag_target.as_ref().map(|t| &t[..]),
            features: &options.flag_features,
            no_default_features: options.flag_no_default_features,
            spec: options.flag_package.as_ref().map(|s| &s[..]),
            exec_engine: None,
            filter: ops::CompileFilter::Everything,
            release: false,
            mode: ops::CompileMode::Doc {
                deps: !options.flag_no_deps,
            },
            target_rustc_args: None,
        },
    };

    try!(ops::doc(&root, &mut doc_opts).map_err(|err| {
        CliError::from_boxed(err, 101)
    }));

    Ok(None)
}

