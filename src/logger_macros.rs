//Copyright (c) 2016, Ruslan Baratov, Alex Frappier Lachapelle
//All rights reserved.

use std::process::exit;

extern crate thread_id;

////////////////////////////////////////////////////////////
//                         Macros                         //
////////////////////////////////////////////////////////////

macro_rules! crash {
    ($($msg:tt)*) => {
        error!("**CRASH**: {}", format_args!($($msg)*));
        error!("**INTERNAL** STOP!!!");
        println!("**CRASH**: {}", format_args!($($msg)*));
        exit(-1);
    }
}

macro_rules! thread_crash {
    ($($msg:tt)*) => {
        thread_error!("**CRASH**: {}", format_args!($($msg)*));
        error!("**INTERNAL** STOP!!!");
        println!("**CRASH**: {}", format_args!($($msg)*));
        exit(-1);
    }
}

macro_rules! thread_error {
    ($($msg:tt)*) => {
        error!("Thread ID {}: {}", thread_id::get(), format_args!($($msg)*));
    }
}

macro_rules! thread_warn {
    ($($msg:tt)*) => {
        warn!("Thread ID {}: {}", thread_id::get(), format_args!($($msg)*));
    }
}

macro_rules! thread_info {
    ($($msg:tt)*) => {
        info!("Thread ID {}: {}", thread_id::get(), format_args!($($msg)*));
    }
}

macro_rules! thread_debug {
    ($($msg:tt)*) => {
        debug!("Thread ID {}: {}", thread_id::get(), format_args!($($msg)*));
    }
}

macro_rules! thread_trace {
    ($($msg:tt)*) => {
        trace!("Thread ID {}: {}", thread_id::get(), format_args!($($msg)*));
    }
}
