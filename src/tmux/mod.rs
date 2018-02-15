use clap::{ArgMatches, SubCommand, App};

use Runner;
use ops;

use common::*;

use me::INSTALL_DIR;

pub struct Tmux {}

lazy_static! {
    static ref TMUX_CONF_PATH: HomePath = HomePath(".tmux.conf".into());
    static ref DOTFILES_TMUX_CONF_PATH: HomePath = {
        INSTALL_DIR.join("dotfiles/tmux/tmux.dconf")
    };
}

impl Tmux {
    fn install(context: Context) -> Result<()> {

        let tmux_conf_path = TMUX_CONF_PATH.try_into_path()?;

        let dotfiles_tmux_conf_path = DOTFILES_TMUX_CONF_PATH.try_into_path()?;

        ops::path::symlink(&context, &dotfiles_tmux_conf_path, &tmux_conf_path)?;

        Ok(())
    }
}


impl Tmux {
    #[cfg(target_os = "linux")]
    fn clean(context: Context) -> Result<()> {
        let tmux_conf_path = TMUX_CONF_PATH.try_into_path()?;
        ops::path::ensure_clean(&context, tmux_conf_path)?;
        Ok(())
    }
}

impl Runner for Tmux {
    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("tmux")
            .about("Setting configuration files of tmux")
            .subcommand(
                SubCommand::with_name("install")
                    .about("install tmux configuration")
            )
            .subcommand(
                SubCommand::with_name("clean")
                    .about("cleaning tmux configuration files")
            )
    }

    fn run(argm: &ArgMatches, context: Context) -> Result<()> {
        match argm.subcommand_name() {
            Some(name) => match name {
                "install" => {
                    Self::install(context)?;
                }
                "clean" => {
                    Self::clean(context)?;
                }
                _ => unreachable!(),
            }
            None => {
                Self::build_cli().print_help().unwrap();
                println!();
            }
        };
        Ok(())
    }
}
