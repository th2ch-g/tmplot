
use crate::arg::{ * };
use crate::common::{ * };


pub fn execute(scatter_arg: ScatterArg) {


    let (xdata, ydata) = load_xy_data(&scatter_arg.xdata, &scatter_arg.ydata);
    let (xmin, xmax) = search_minmax(xdata.clone());
    let (ymin, ymax) = search_minmax(ydata.clone());


    // Graph setting
    let image_width = 1440;
    let image_height = 900;
    let caption = &format!("{}_scatter", &scatter_arg.title);


    if !scatter_arg.output_jpg {

        let output_name = &format!("{}.png", &scatter_arg.prefix);
        common_plot(image_width, image_height, caption, &output_name, ymin, ymax, xmin, xmax, ydata, xdata, &scatter_arg.xlabel, &scatter_arg.ylabel, "scatter");

    }
    else {

        let output_name = &format!("{}.jpg", &scatter_arg.prefix);
        common_plot(image_width, image_height, caption, &output_name, ymin, ymax, xmin, xmax, ydata, xdata, &scatter_arg.xlabel, &scatter_arg.ylabel, "scatter");

    }

}




