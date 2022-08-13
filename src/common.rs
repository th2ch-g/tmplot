

use std::io::{self, Read, BufReader, BufRead};
use std::fs::File;
use std::process;
use plotters::prelude::*;


fn pipe_2_data() -> (Vec<f64>, Vec<f64>) {


    let mut buf = String::from("");
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf).expect("Failed to read PIPE input");

    let v: Vec<&str> = buf.split("\n").collect();

    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();


    if v[0].contains(" ") {

        println!("[INFO] PIPE data seems to be SPACE split");
        let tmp: Vec<&str> = v[0].split(" ").collect();

        if tmp.len() != 2 {
            println!("[ERROR] PIPE data seems to be only 1 data");
            process::exit(1);
        }

        for i in 0..v.len() {

            let tmp: Vec<&str> = v[i].split(" ").collect();

            if i == v.len()-1 {
                if tmp[0] == "" {
                    break
                }
            }

            x.push(tmp[0].parse().unwrap());
            y.push(tmp[1].parse().unwrap());

        }

    }
    else if v[0].contains("\t") {

        println!("[INFO] PIPE data seems to be TAB split");
        let tmp: Vec<&str> = v[0].split("\t").collect();

        if tmp.len() != 2 {
            println!("[ERROR] PIPE data seems to be only 1 data");
            process::exit(1);
        }

        for i in 0..v.len() {

            let tmp: Vec<&str> = v[i].split("\t").collect();

            if i == v.len()-1 {
                if tmp[0] == "" {
                    break
                }
            }

            x.push(tmp[0].parse().unwrap());
            y.push(tmp[1].parse().unwrap());

        }

    }
    else if v[0].contains(",") {

        println!("[INFO] PIPE data seems to be COMMA split");
        let tmp: Vec<&str> = v[0].split(",").collect();

        if tmp.len() != 2 {
            println!("[ERROR] PIPE data seems to be only 1 data");
            process::exit(1);
        }

        for i in 0..v.len() {

            let tmp: Vec<&str> = v[i].split(",").collect();

            if i == v.len()-1 {
                if tmp[0] == "" {
                    break
                }
            }

            x.push(tmp[0].parse().unwrap());
            y.push(tmp[1].parse().unwrap());

        }

    }
    else {
        println!("[ERROR] PIPE data seems to be only 1 data");
        process::exit(1);
    }


    (x, y)

}



fn pipe_1_data() -> Vec<f64> {


    let mut buf = String::from("");
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf).expect("Failed to read PIPE input");

    let v: Vec<&str> = buf.split("\n").collect();
    let mut data: Vec<f64> = Vec::new();

    for i in 0..v.len() {

        if i == v.len() - 1 {
            if v[i] == "" {
                break
            }
        }
        data.push(v[i].parse().unwrap());

    }

    data


}



fn file_1_data(file: &str) -> Vec<f64> {

    let mut data: Vec<f64> = Vec::new();

    let f = File::open(file).unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();

        data.push(line.parse().unwrap());
    }

    data

}


pub fn load_x_data(hyphen: &str) -> Vec<f64> {

    if hyphen == "-" {
        pipe_1_data()
    }
    else {
        file_1_data(hyphen)
    }

}


pub fn load_xy_data(x_hyphen: &str, y_hyphen: &str) -> (Vec<f64>, Vec<f64>) {

    if x_hyphen == "-" && y_hyphen == "-" {
        pipe_2_data()
    }
    else if x_hyphen == "-" && y_hyphen != "-" {
        (pipe_1_data(), file_1_data(y_hyphen))
    }
    else if x_hyphen != "-" && y_hyphen == "-" {
        (file_1_data(x_hyphen), pipe_1_data())
    }
    else {
        (file_1_data(x_hyphen), file_1_data(y_hyphen))
    }

}


pub fn search_minmax(data: Vec<f64>) -> (f64, f64) {

    data.iter().fold(
        (0.0/0.0, 0.0/0.0),
        |(m, n), v| (v.min(m), v.max(n))
                    )

}



pub fn common_plot(image_width: u32, image_height: u32, caption: &str, output_name: &str, ymin: f64, ymax: f64, xmin: f64, xmax: f64, ydata: Vec<f64>, xdata: Vec<f64>, xlabel: &str, ylabel: &str, mode: &str) {

    let font = ("Arial", 20); // sans-serif

    let root = BitMapBackend::new(
       &output_name, (image_width, image_height)
                                 ).into_drawing_area();

    root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
        .caption(caption, font.into_font())
        .margin(10)
        .x_label_area_size(42)
        .y_label_area_size(42)
        .build_cartesian_2d(
            xmin..xmax,
            ymin..ymax,
                           ).unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_desc(xlabel)
        .y_desc(ylabel)
        .draw().unwrap();


    if mode == "plot" {
        let line_series = LineSeries::new(
            xdata.iter().zip(ydata.iter()).map(|(x, y)| (*x, *y)), &BLUE
                                        );
        chart.draw_series(line_series).unwrap();

    }
    else if mode == "scatter" {

        let mut xydata = Vec::new();
        for i in 0..xdata.len() {
            xydata.push((xdata[i], ydata[i]));
        }
        chart.draw_series(xydata.iter().map(|point| Circle::new(*point, 2, &BLUE))).unwrap();

    }


}


