

use clap::{ Parser, Subcommand };


#[derive(Parser, Debug, Clone)]
#[clap(version, about)] //#[clap(author, version, about)]
pub struct MainArg {

    #[clap(subcommand)]
    pub mode: Mode,

}



#[derive(Subcommand, Debug, Clone)]
pub enum Mode {

    /// Simple plot Connecting dots to draw
    #[clap(display_order = 1)]
    Plot(PlotArg),

    /// Simple plot NOT Connecting dots to draw
    #[clap(display_order = 2)]
    Scatter(ScatterArg),

    /*
    #[clap(display_order = 3)]
    Bar(BarArg),

    #[clap(display_order = 4)]
    Hist(HistArg),
    */

}



#[derive(Debug, clap::Args, Clone)]
#[clap(arg_required_else_help = true, version)]
pub struct PlotArg {

    #[clap(short = 'x', long = "xdata", value_name = "DATA", help = "x_data of 2D-plot. \nSupports FILE name or PIPE input. For pipe input, use \"-x - \"", display_order = 1)]
    pub xdata: String,

    #[clap(short = 'y', long = "ydata", value_name = "DATA", help = "y_data of 2D-plot. \nSupports FILE name or PIPE input. For pipe input, use \"-y - \"", display_order = 2)]
    pub ydata: String,

    #[clap(long = "prefix", value_name = "STR", default_value = "out", help = "output picture file prefix", display_order = 3)]
    pub prefix: String,

    #[clap(long = "xlabel", value_name = "STR", default_value = "xlabel", help = "output picture xlabel", display_order = 4)]
    pub xlabel: String,

    #[clap(long = "ylabel", value_name = "STR", default_value = "ylabel", help = "output picture ylabel", display_order = 5)]
    pub ylabel: String,

    #[clap(long = "title", value_name = "STR", default_value = "title", help = "output picture title", display_order = 6)]
    pub title: String,

    #[clap(long = "jpg", help = "Flag whether JPG output is performed.", display_order = 7)]
    pub output_jpg: bool,

}


#[derive(Debug, clap::Args, Clone)]
#[clap(arg_required_else_help = true, version)]
pub struct ScatterArg {

    #[clap(short = 'x', long = "xdata", value_name = "DATA", help = "x_data of 2D-plot. \nSupports FILE name or PIPE input. For pipe input, use \"-x - \"", display_order = 1)]
    pub xdata: String,

    #[clap(short = 'y', long = "ydata", value_name = "DATA", help = "y_data of 2D-plot. \nSupports FILE name or PIPE input. For pipe input, use \"-y - \"", display_order = 2)]
    pub ydata: String,

    #[clap(long = "prefix", value_name = "STR", default_value = "out", help = "output picture file prefix", display_order = 3)]
    pub prefix: String,

    #[clap(long = "xlabel", value_name = "STR", default_value = "xlabel", help = "output picture xlabel", display_order = 4)]
    pub xlabel: String,

    #[clap(long = "ylabel", value_name = "STR", default_value = "ylabel", help = "output picture ylabel", display_order = 5)]
    pub ylabel: String,

    #[clap(long = "title", value_name = "STR", default_value = "title", help = "output picture title", display_order = 6)]
    pub title: String,

    #[clap(long = "jpg", help = "Flag whether JPG output is performed.", display_order = 7)]
    pub output_jpg: bool,

}

/*
#[derive(Debug, clap::Args, Clone)]
#[clap(arg_required_else_help = true, version)]
pub struct BarArg {

}


#[derive(Debug, clap::Args, Clone)]
#[clap(arg_required_else_help = true, version)]
pub struct HistArg {

    #[clap(long = "data", value_name = "DATA", help = "data of 2D-plot. \nSupports FILE name or PIPE input. For pipe input, use \"--data - \"", display_order = 1)]
    pub data: String,

    #[clap(long = "prefix", value_name = "STR", default_value = "out", help = "output picture file prefix", display_order = 2)]
    pub prefix: String,

    #[clap(long = "xlabel", value_name = "STR", default_value = "xlabel", help = "output picture xlabel", display_order = 3)]
    pub xlabel: String,

    #[clap(long = "ylabel", value_name = "STR", default_value = "ylabel", help = "output picture ylabel", display_order = 4)]
    pub ylabel: String,

    #[clap(long = "title", value_name = "STR", default_value = "title", help = "output picture title", display_order = 5)]
    pub title: String,

    #[clap(long = "jpg", help = "Flag whether JPG output is performed.", display_order = 6)]
    pub output_jpg: bool,

}
*/




pub fn arg() -> MainArg {

    MainArg::parse()

}



