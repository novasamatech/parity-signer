//! Navigation state of the app

use crate::screens::Screen;

///Navigation state is completely defined here
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Navstate {
    pub screen: Screen,
}

impl Navstate {
    ///This converts state into renderable block
    pub fn generate_json(self) -> String {
        let mut output = String::from("{");
        let screen = self.screen;
        if let Some(screen_name) = screen.get_name() {
            output.push_str(&format!("\"screen\":\"{}\",\"screenLabel\":\"{}\",\"back\":{},", screen_name, self.get_screen_label(), false));
        }
        output.pop();
        output.push_str("}");
        output
    }

    ///Generate screen label taking into account state
    fn get_screen_label(self) -> String {
        self.screen.get_default_label()
    }
}
