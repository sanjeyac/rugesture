
use crate::event_input::TouchStatus;

use svg::node::element::Path;
use svg::node::element::path::Data;

use std::fs::File;
use std::io::{BufWriter, Write};


use svg::Document;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn save_as_csv(filename: &str, status_history: &Vec<TouchStatus>) {
    
    let f = File::create(filename).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    for status in status_history.iter(){
        let data = format!("{},{},\t    {}, {}, {},\t  {}, {}, {},\t  {}, {}, {},\t  {}, {}, {},\t  {}, {}, {} \n", 
            status.sec,
            status.dot_sec,
            status.fingers[0].x,
            status.fingers[0].y,
            status.fingers[0].pressed,
            status.fingers[1].x,
            status.fingers[1].y,
            status.fingers[1].pressed,
            status.fingers[2].x,
            status.fingers[2].y,
            status.fingers[2].pressed,
            status.fingers[3].x,
            status.fingers[3].y,
            status.fingers[3].pressed,
            status.fingers[4].x,
            status.fingers[4].y,
            status.fingers[4].pressed,
        );

        f.write_all(data.as_bytes()).expect("Unable to write data");          
    }
    
}



pub fn create_line_from(finger_points: Vec<(i32,i32)>, color: &str) -> Path {

    let data = finger_points.iter()
                    .filter( |data| data.0 > 0)
                    .fold( Data::new(), |data, &point| { 
                        if data.len() == 0 {
                            data.move_to( point )
                        } else {
                            data.line_to( point )
                        }
                    });

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("stroke-width", 30)
        .set("d", data);

    return path;
}


pub fn save_as_svg(filename: &str, status_history: &Vec<TouchStatus>) {
    
    let mut finger0: Vec<(i32,i32)> = Vec::new();
    let mut finger1: Vec<(i32,i32)> = Vec::new();
    let mut finger2: Vec<(i32,i32)> = Vec::new();
    let mut finger3: Vec<(i32,i32)> = Vec::new();
    let mut finger4: Vec<(i32,i32)> = Vec::new();

    for status in status_history.iter(){
    // tracking fingres for the svg
            finger0.push( (status.fingers[0].x,status.fingers[0].y) );
            finger1.push( (status.fingers[1].x,status.fingers[1].y) );
            finger2.push( (status.fingers[2].x,status.fingers[2].y) );
            finger3.push( (status.fingers[3].x,status.fingers[3].y) );
            finger4.push( (status.fingers[4].x,status.fingers[4].y) );         
    }

    let document = Document::new()
        .set("viewBox", (0, 0, 3200, 2000))
        .add(create_line_from(finger0, "black"))
        .add(create_line_from(finger1, "green"))
        .add(create_line_from(finger2, "blue"))
        .add(create_line_from(finger3, "red"))
        .add(create_line_from(finger4, "orange"));

    svg::save(filename, &document).unwrap();
    
}