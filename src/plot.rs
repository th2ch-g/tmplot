
use crate::arg::{ * };
use crate::common::{ * };


pub fn execute(plot_arg: PlotArg) {


    let (xdata, ydata) = load_xy_data(&plot_arg.xdata, &plot_arg.ydata);
    let (xmin, xmax) = search_minmax(xdata.clone());
    let (ymin, ymax) = search_minmax(ydata.clone());


    // Graph setting
    let image_width = 1440;
    let image_height = 900;
    let caption = &format!("{}_plot", &plot_arg.title);


    if !plot_arg.output_jpg {

        let output_name = &format!("{}.png", &plot_arg.prefix);
        common_plot(image_width, image_height, caption, &output_name, ymin, ymax, xmin, xmax, ydata, xdata, &plot_arg.xlabel, &plot_arg.ylabel, "plot");

    }
    else {

        let output_name = &format!("{}.jpg", &plot_arg.prefix);
        common_plot(image_width, image_height, caption, &output_name, ymin, ymax, xmin, xmax, ydata, xdata, &plot_arg.xlabel, &plot_arg.ylabel, "plot");

    }

}


