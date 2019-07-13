use crate::utils::*;

pub struct EventInput{
    pub sec: u64,
    pub dot_sec: u64,
    pub evtype: u16,
    pub code: u16,
    pub value: i32
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Finger {
    pub pressed: bool,
    pub x: i32,
    pub y: i32,   
}

#[derive(Clone)]
#[derive(Debug)]
pub struct TouchStatus{    
    pub sec: u64,
    pub dot_sec: u64,
    pub fingers: [Finger; 5],
    pub tap: bool,
    pub double_tap: bool,
    pub triple_tap: bool,
    pub quad_tap: bool,
    pub quin_tap: bool,
    pub current_slot: usize
}

/**
 * Create an event input from array of bytes
 */
pub fn create_input_from(bytes:&[u8; 24]) -> EventInput {
    EventInput {
        sec: as_u64(&bytes[0..8]),
        dot_sec: as_u64(&bytes[8..16]),
        evtype: as_u16(&bytes[16..18]),
        code: as_u16(&bytes[18..20]),
        value: as_i32(&bytes[20..24])
    }
}

pub fn init_status() -> TouchStatus {
    return TouchStatus {
        sec: 0,
        dot_sec: 0,
        fingers: [ 
            Finger { x:0, y:0, pressed: false },
            Finger { x:0, y:0, pressed: false },
            Finger { x:0, y:0, pressed: false },
            Finger { x:0, y:0, pressed: false },
            Finger { x:0, y:0, pressed: false }
        ],
        tap: false,
        double_tap: false,
        triple_tap: false,
        quad_tap: false,
        quin_tap: false,
        current_slot: 0
    };
}

/**
    Event code 272 (BTN_LEFT)
    Event code 325 (BTN_TOOL_FINGER)
    Event code 328 (BTN_TOOL_QUINTTAP)
    Event code 330 (BTN_TOUCH)
    Event code 333 (BTN_TOOL_DOUBLETAP)
    Event code 334 (BTN_TOOL_TRIPLETAP)
    Event code 335 (BTN_TOOL_QUADTAP)
 */
pub fn update_status_by(queue: &Vec<EventInput>, status: &mut TouchStatus) {
    //let mut current_finger = 0;
    for each_event in queue {
        status.sec = each_event.sec;
        status.dot_sec = each_event.dot_sec;
        match each_event.code {
            //0 => println!("this is a SYN"),
            47  => status.current_slot = each_event.value as usize,
            53  => status.fingers[status.current_slot].x = each_event.value,
            54  => status.fingers[status.current_slot].y = each_event.value,
            330 => status.fingers[status.current_slot].pressed = each_event.value > 0,
            333 => status.double_tap = each_event.value == 1,
            334 => status.triple_tap = each_event.value == 1,
            335 => status.quad_tap = each_event.value == 1,
            328 => status.quin_tap = each_event.value == 1,
            _   => {}
        }
    }
    //println!("status: {:?}",status);
}

