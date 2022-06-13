use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, PointStyle};

fn main() {
    draw();
}

fn draw() {
    let file = File::open("datas/data2.csv")
        .expect("open failed");

    let mut x_axis: Vec<f64> = vec![];
    let mut y1_axis: Vec<f64> = vec![];
    let mut y2_axis: Vec<f64> = vec![];
    let mut y3_axis: Vec<f64> = vec![];
                                                              
    for line in BufReader::new(file).lines() {
        let info = line.unwrap();
        let info_list: Vec<&str> = info.split(',').collect();
                                                              
        x_axis.push(info_list[0].parse::<f64>().unwrap().log10());
        y1_axis.push(info_list[1].parse::<f64>().unwrap().log10()); 
        y2_axis.push(info_list[2].parse::<f64>().unwrap().log10()); 
        y3_axis.push(info_list[3].parse::<f64>().unwrap().log10()); 
    }
                                                              
    println!("{:?}, {:?}, {:?}, {:?}", x_axis, y1_axis, y2_axis, y3_axis);
    
    let data1: Vec<(_, _)>  = zip(x_axis.clone(), y1_axis.clone()).collect();
    let data2: Vec<(_, _)>  = zip(x_axis.clone(), y2_axis.clone()).collect();
    let data3: Vec<(_, _)>  = zip(x_axis.clone(), y3_axis.clone()).collect();
    
    println!("{:?}, {:?}, {:?}", data1, data2, data3);

    let s1: Plot = Plot::new(data1).point_style(
        PointStyle::new()
            .marker(PointMarker::Square) // setting the marker to be a square
            .colour("#DD3355")
    ).legend("mult".to_string()); // and a custom colour                                                    
    let s2: Plot = Plot::new(data2).point_style(                               
        PointStyle::new()
            .marker(PointMarker::Cross) // setting the marker to be a square
            .colour("#35C788"),
    ).legend("mult_recur".to_string()); // and a custom colour
    let s3: Plot = Plot::new(data3).point_style(                              
        PointStyle::new()
            .marker(PointMarker::Circle) // setting the marker to be a square
            .colour("#00A4FF"),
    ).legend("mult_recur_pro".to_string()); // and a custom colour

    // The 'view' describes what set of data is drawn         
    let view = ContinuousView::new()
        .add(s1)
        .add(s2)
        .add(s3)
        .x_range(0., 7.)
        .y_range(-2.5, 10.0)
        .x_label("log of length")
        .y_label("log of time");
                                                              
    // A page with a single view is then saved to an SVG file
    Page::single(&view).save("results/result2.svg").unwrap();
}

