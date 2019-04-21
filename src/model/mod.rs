// type model mod 

pub mod msg;
pub mod user;
pub mod rut;
pub mod item;
pub mod tag;
pub mod etc;

use actix_web::Error;
use regex::Regex;

// for validate request content input
pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}


// re test
// for re test uname
pub fn re_test_name(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = 
            Regex::new(r"^[\w-]{3,42}$").unwrap();
    }
    RE.is_match(text)
}

// for re test psw
pub fn re_test_psw(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = 
            Regex::new(r"^[\w#@~%^$&*-]{8,18}$").unwrap();
    }
    RE.is_match(text)
}

// for re test url
pub fn re_test_url(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = 
            Regex::new(r"^(https?)://([^/:]+)(:[0-9]+)?(/.*)?$").unwrap();
    }
    RE.is_match(text)
}

pub fn test_len_limit(text: &str, min: usize, max: usize) -> bool {
    let l = text.trim().len();
    l >= min && l <= max
}

// some const 
const TITLE_LEN: usize = 256;
const URL_LEN: usize = 256;
const UIID_LEN: usize = 32;
const ST_LEN: usize = 16;  // for some short input: category
const MID_LEN: usize = 32;  // for some mid input: lcoation
const LG_LEN: usize = 64;   // for sone longer input: 