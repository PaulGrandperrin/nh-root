use crate::interface::{ManArgs, NHRunnable, NHParser};
use crate::Result;
use std::io::Write;
use std::io::stdout;

impl NHRunnable for ManArgs {
    fn run(&self) -> Result<()> {
        let cmd = <NHParser as clap::CommandFactory>::command();
        let man = clap_mangen::Man::new(cmd);
        let mut stdout = stdout();
        man.render(&mut stdout)?;
        Ok(())
    }
}
