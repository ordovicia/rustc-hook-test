#![feature(rustc_private)]

extern crate getopts;
extern crate rustc;
extern crate rustc_driver;

use std::{env, process};

use rustc::session::Session;
use rustc_driver::{driver::CompileController, Compilation, CompilerCalls};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    rustc_driver::run_compiler(&args, Box::new(CallHook), None, None);

    process::exit(0);
}

struct CallHook;

impl<'a> CompilerCalls<'a> for CallHook {
    fn build_controller(
        self: Box<Self>,
        _: &Session,
        _: &getopts::Matches,
    ) -> CompileController<'a> {
        let mut controller = CompileController::basic();

        controller.after_parse.stop = Compilation::Stop;
        controller.after_parse.callback = Box::new(|_| println!("after_parse"));

        controller
    }
}
