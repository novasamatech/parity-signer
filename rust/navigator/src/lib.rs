//! This is experimental crossplatform navigation for Signer.
//! Ideally it should replace almost everything and become the only interface

pub mod screens;
use screens::Screen;

mod actions;
use actions::Action;

///This should be called from UI; returns new UI information as JSON
pub fn do_action(
    origin_str: &str,
    action_str: &str,
    details_str: &str,
) -> String {
    let origin = Screen::parse(origin_str);
    let action = Action::parse(action_str);
    action.perform(details_str)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
