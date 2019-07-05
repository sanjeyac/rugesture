
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::num::ParseFloatError;

#[allow(dead_code)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum GestureDirection { NONE, UP, DOWN, LEFT, RIGHT }

pub fn parse_direction_from(line: &String) -> GestureDirection {


    let elements = line.split_whitespace().collect::<Vec<_>>();

    let speeds = extract_speeds(elements[4].to_string());

    match speeds {
        Ok(speed) => direction_of( speed),
        Err(e) => GestureDirection::NONE
    }
 
}


pub fn extract_speeds(string: String)  -> Result<(f32, f32), ParseFloatError> {

    let speeds = string.trim().split("/").collect::<Vec<_>>();

    let x = match speeds[0].parse::<f32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };

    let y = match speeds[1].parse::<f32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };    

    return Ok((x,y));

}

pub fn direction_of( speeds : (f32, f32) ) -> GestureDirection {

    // avoid problems between positive and negative numbers
    let abs_0 = speeds.0.abs();
    let abs_1 = speeds.1.abs();

    // speed on x axis
    if abs_0 > abs_1 {
        if speeds.0 < 0.0 {
            return GestureDirection::LEFT;
        } else {
            return GestureDirection::RIGHT;
        }
    } else {
        if speeds.1 < 0.0 {
            return GestureDirection::UP;
        } else {
            return GestureDirection::DOWN;
        }
    }

}

pub fn sanitarize(line: &String) -> String {
    let safe_line = line.replace("/ ", "/");
    let trimmed = safe_line.trim();
    let cutted_line: String = match trimmed.find('(') {
                Some(i) => trimmed[0..i].to_string(),
                None => trimmed.to_string()
            };
    return cutted_line;
}

pub fn compute(line: String, last_update: &mut String) {        

    let safe_line = sanitarize(&line);

    if safe_line.contains("GESTURE_SWIPE_END") {        
        let direction = parse_direction_from(&last_update.to_string());
        println!("COMMAND {:?} ", direction);
    }    
    if safe_line.contains("GESTURE_SWIPE_UPDATE") {
        last_update.clear();
        last_update.push_str(&safe_line);

    }       
}




pub fn run_debug_input() -> Result<(), Error> {
    let mut last_update = "".to_string();

    let LIBINPUT_DEBUG_COMMAND = "libinput-debug-events";

    let stdout = Command::new(LIBINPUT_DEBUG_COMMAND)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}",line));

    Ok(())
}



#[cfg(test)]
mod tests {

    use super::*;

    fn nearly_equal(a: f32, b: f32) -> bool {
        let diff = (a - b).abs();
        return  diff < 0.0001;
    }    

    #[test]
    fn test_extract_speeds() {
        let result = extract_speeds("0.12/0.34".to_string());
        let tuple = result.unwrap();
        assert_eq!( nearly_equal(tuple.0, 0.12 ) , true );
        assert_eq!( nearly_equal(tuple.1, 0.34 ) , true );
    }

    #[test]
    fn test_extract_speeds_with_spaces() {
        let result = extract_speeds(" 0.12/0.34 ".to_string());
        let tuple = result.unwrap();
        assert_eq!( nearly_equal(tuple.0, 0.12 ) , true );
        assert_eq!( nearly_equal(tuple.1, 0.34 ) , true );
    }    

    #[test]
    fn test_extract_speeds_with_negative_values() {
        let result = extract_speeds(" -0.12/-0.34 ".to_string());
        let tuple = result.unwrap();
        assert_eq!( nearly_equal(tuple.0, -0.12 ) , true );
        assert_eq!( nearly_equal(tuple.1, -0.34 ) , true );
    }       

    #[test]
    fn test_extract_direction_left() {
        let result = direction_of((-1.0,0.2));                
        assert!( GestureDirection::LEFT == result );
    }       

    #[test]
    fn test_extract_direction_right() {
        let result = direction_of((1.0, 0.2));                
        assert!( GestureDirection::RIGHT == result );
    }     

    #[test]
    fn test_extract_direction_up() {
        let result = direction_of((1.0,-3.2));                
        assert!( GestureDirection::UP == result );
    }     

    #[test]
    fn test_extract_direction_down() {
        let result = direction_of(( 1.0, 3.2));                
        assert!( GestureDirection::DOWN == result );
    }                 


}
