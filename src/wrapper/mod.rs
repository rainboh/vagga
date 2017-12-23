use std::env::{current_dir};
use std::io::{stdout, stderr, Write};
use std::path::Path;
use std::sync::Arc;

use argparse::{ArgumentParser, Store, StoreOption, List};

use super::config::{find_config_or_exit, Config, Settings};
use super::config::command::MainCommand::{Command, Supervise, CapsuleCommand};
use config::read_settings::{read_settings, MergedSettings};


mod debug;
mod build;
mod run;
pub mod setup;
mod util;
mod clean;
mod pack;
mod snapshot;
mod init_persistent;
// Command types
mod supervise;
mod commandline;
mod capsule;


pub struct Wrapper<'a> {
    config: &'a Config,
    settings: &'a Arc<Settings>,
    project_root: &'a Path,
    ext_settings: &'a MergedSettings,
    root: Option<String>,
}


pub fn run(input_args: Vec<String>) -> i32 {
println!("wrapper run 1: input_args={:?}", input_args);
    let mut err = stderr();
    let mut cmd: String = "".to_string();
    let mut args: Vec<String> = Vec::new();
    let mut root = None;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("
            Internal vagga tool to setup basic system sandbox
            ");
        ap.refer(&mut root)
          .add_option(&["--root"], StoreOption, "
            Root to choose for running container (for command that run in a
            container)");
        ap.refer(&mut cmd)
          .add_argument("command", Store,
                "A vagga command to run")
          .required();
        ap.refer(&mut args)
          .add_argument("args", List,
                "Arguments for the command");
        ap.stop_on_first_argument(true);
        match ap.parse(input_args, &mut stdout(), &mut stderr()) {
            Ok(()) => {}
            Err(0) => return 0,
            Err(_) => {
                return 122;
            }
        }
    }

    let workdir = current_dir().unwrap();

println!("wrapper run 2: workdir={:?}", workdir);
    let (config, project_root) = find_config_or_exit(&workdir, false);
println!("wrapper run 3: project_root={:?}", project_root);
    let (ext_settings, int_settings) = match read_settings(&project_root)
    {
        Ok(tup) => tup,
        Err(e) => {
            writeln!(&mut err, "{}", e).ok();
            return 126;
        }
    };

println!("wrapper run 4: workdir={:?}", workdir);
    let wrapper = Wrapper {
        root: root,
        config: &config,
        settings: &Arc::new(int_settings),
        project_root: &project_root,
        ext_settings: &ext_settings,
    };

    args.insert(0, format!("vagga {}", cmd));

println!("wrapper run 5: cmd={:?}", cmd);
    let result = match &cmd[..] {
        "_build_shell" => Ok(debug::run_interactive_build_shell(&wrapper)),
        "_check_overlayfs_support" => Ok(debug::check_overlayfs(&wrapper)),
        "_build" => build::build_container_cmd(&wrapper, args),
        "_version_hash" => build::print_version_hash_cmd(&wrapper, args),
        "_run" | "_run_in_netns" => run::run_command_cmd(&wrapper, args),
        "_clean" => clean::clean_cmd(&wrapper, args),
        "_pack_image" => pack::pack_image_cmd(&wrapper, args),
        _ => {
            match config.commands.get(&cmd) {
                Some(&Command(ref cmd_info)) => {
                    commandline::commandline_cmd(&cmd,
                        cmd_info, &wrapper, args)
                }
                Some(&CapsuleCommand(ref cmd_info)) => {
                    capsule::commandline_cmd(&cmd, cmd_info, &wrapper, args)
                }
                Some(&Supervise(ref svc_info)) => {
                    supervise::supervise_cmd(&cmd, svc_info, &wrapper, args)
                }
                None => {
                    error!("Unknown command {}", cmd);
                    return 127;
                }
            }
        }
    };
    match result {
        Ok(x) => return x,
        Err(e) => {
            error!("Error executing {}: {}", cmd, e);
            return 124;
        }
    };
}
