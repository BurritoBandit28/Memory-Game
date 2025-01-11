
use std::sync::Mutex;
use std::time::SystemTime;
use log::{Level, LevelFilter, Metadata, Record};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use num::integer::{sqrt, Roots};
use num::pow;
use uuid::Uuid;
use crate::entity::{Entity,};

pub fn mul_vec(vec : &mut (f32, f32), val : f32) {
    vec.0 *= val;
    vec.1 *= val;
}

// broken : todo fix
pub fn normalise_vec(vec : (f32, f32)) -> (f32,f32) {
    // get the square root of the object
    let mag = f32::sqrt((vec.0 * vec.0) + (vec.1 * vec.1));
    if mag == 0.0 {
        return (0.0, 0.0)
    }
    (vec.0 / mag, vec.1 / mag)

}



pub(crate) fn order_sort(entities : &mut Vec<Box<Mutex<dyn Entity>>>) -> Vec<(usize, usize, f32)> {
    //              list   index  amount
    let mut list : Vec<(usize, usize, f32)> = vec![];
    let mut iter = 0usize;
    for s in entities {
        list.push((0usize, iter, s.lock().unwrap().get_coords().1) );
        iter+=1;
    }

    sort(&mut list);

    list

}

// a quick sort alogorithm designed to get the order of renderable objects
fn sort(list : &mut Vec<(usize, usize, f32)>) {
    if !(list.len() <= 1) {
        let mut indx1 = 1usize;
        let mut indx2 = list.len() - 1;
        let pivot = 0;
        while indx2 >= indx1 {
            while indx1 <= indx2 && list[indx1].2 <= list[pivot].2 {
                indx1 += 1;
            }
            while indx2 >= indx1 && list[indx2].2 >= list[pivot].2 {
                indx2 -= 1;
            }
            if indx2 > indx1 {
                list.swap(indx1, indx2)
            }
        }
        list.swap(pivot, indx2);

        let mut left = list[0..indx2].to_vec();
        sort(&mut left);
        let mut right = list[(indx2 + 1)..list.len()].to_vec();
        sort(&mut right);

        left.push(list[indx2]);
        left.append(&mut right);
        list.clear();
        list.extend_from_slice(&left);
    }
}

pub fn init_logger() {

    // following code from https://github.com/estk/log4rs/blob/main/examples/log_to_file.rs
    // slightly edited

    let level = log::LevelFilter::Info;
    let file_path = format!("log/{:}.log", chrono::offset::Local::now().to_string().replace(" ", "_").replace(":", "-"));

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config);
}

pub fn create_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn get_dist(a : &(f32, f32), b : &(f32, f32)) -> u32 {
    let a_squared = (a.0 + b.0) * (a.0 + b.0);
    let b_squared = (a.1 + b.1) * (a.1 + b.1);
    let c_squared = a_squared + b_squared;
    u32::sqrt(&(c_squared as u32))
}

