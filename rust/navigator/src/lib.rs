//! This is experimental crossplatform navigation for Signer.
//! Ideally it should replace almost everything and become the only interface

//do we support mutex?
use std::sync::{Mutex, TryLockError};
use lazy_static::lazy_static;

pub mod screens;
use screens::Screen;

mod navstate;
use navstate::Navstate;

mod actions;
use actions::Action;

//TODO: multithread here some day!
lazy_static!{
///Navigation state of the app
///
///Navigation state is unsafe either way, since it has to persist
///No matter if here or beyond FFI
   pub static ref STATE: Mutex<Navstate> = Mutex::new(Navstate{
        screen: Screen::Log,
    });
}

///This should be called from UI; returns new UI information as JSON
pub fn do_action(
    _origin_str: &str,
    action_str: &str,
    details_str: &str,
) -> String {
    let mut process_navstate = Navstate{
        screen: Screen::Log,
    };
   
    //guard should have proper lifetime to lock mutex
    let mut guard = STATE.try_lock();
    match guard {
        Ok(mut navstate) => {
            process_navstate = *navstate;
            let origin = process_navstate.screen; 
            let action = Action::parse(action_str);
            *navstate = action.perform(*navstate, details_str);
            (*navstate).generate_json()
        },
        Err(TryLockError::Poisoned(_)) => {
            //TODO: maybe more grace here?
            //Maybe just silently restart navstate? But is it safe?
            panic!("Concurrency error! Restart the app.");
         },
        Err(TryLockError::WouldBlock) => return "".to_string(),
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
