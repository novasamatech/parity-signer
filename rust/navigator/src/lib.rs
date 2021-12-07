//! This is experimental crossplatform navigation for Signer.
//! Ideally it should replace almost everything and become the only interface

//do we support mutex?
use std::sync::{Mutex, TryLockError};
use lazy_static::lazy_static;

use db_handling;
use definitions::keyring::NetworkSpecsKey;

pub mod screens;
use screens::Screen;

pub mod alerts;

pub mod modals;
use modals::Modal;

mod navstate;
use navstate::{Navstate, State};

mod actions;
use actions::Action;

//TODO: multithread here some day!
lazy_static!{
///Navigation state of the app
///
///Navigation state is unsafe either way, since it has to persist
///No matter if here or beyond FFI
    pub static ref STATE: Mutex<State> = Mutex::new(
        State{
            navstate: Navstate::new(),
            dbname: None,
            seed_names: Vec::new(),
            networks: Vec::new(),
        }
    );
}

///This should be called from UI; returns new UI information as JSON
pub fn do_action(
    action_str: &str,
    details_str: &str,
) -> String {
    //If can't lock - debounce failed, ignore action
    //
    //guard is defined here to outline lifetime properly
    let mut guard = STATE.try_lock();
    match guard {
        Ok(mut state) => {
            let action = Action::parse(action_str);
            let details = (*state).perform(action, details_str);
            (*state).generate_json(details)
        },
        Err(TryLockError::Poisoned(_)) => {
            //TODO: maybe more grace here?
            //Maybe just silently restart navstate? But is it safe?
            panic!("Concurrency error! Restart the app.");
         },
        Err(TryLockError::WouldBlock) => return "".to_string(),
    }
}

///Should be called in the beginning to recall things stored only by phone
pub fn init_navigation(
    dbname: &str,
    seed_names: &str,
) -> () {
    //This operation has to happen; lock thread and do not ignore.
    let guard = STATE.lock();
    match guard {
        Ok(mut navstate) => {
            (*navstate).dbname = Some(dbname.to_string());
            (*navstate).seed_names = seed_names.split(",").map(|a| a.to_string()).collect();
            (*navstate).seed_names.sort();
            (*navstate).seed_names.dedup();
            match db_handling::network_details::get_all_networks(dbname) {
                Ok(a) => for x in a.iter() {
                    (*navstate).networks.push(NetworkSpecsKey::from_parts(&x.genesis_hash.to_vec(), &x.encryption));
                },
                Err(e) => println!("No networks could be fetched: {:?}", e),
            };
        },
        Err(_) => {
            //TODO: maybe more grace here?
            panic!("Concurrency error! Restart the app.");
         },
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
