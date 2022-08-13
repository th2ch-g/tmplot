
pub mod arg;
pub mod common;
pub mod plot;
pub mod scatter;

use std::env;

use crate::arg::{ * };


fn main() {

    // arg
    let cli: MainArg = arg::arg();


    // mode
    match &cli.mode {

        Mode::Plot(plot_arg) => {
            println!("[INFO] Plot mode execute...");
            plot::execute(plot_arg.clone());
        },

        Mode::Scatter(scatter_arg) => {
            println!("[INFO] Scatter mode execute...");
            scatter::execute(scatter_arg.clone());
        },


    }

    println!("[INFO] {} done", env!("CARGO_PKG_NAME"));

}



