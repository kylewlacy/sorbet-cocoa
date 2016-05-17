mod ns_object;
mod ns_responder;
mod ns_window;
mod ns_window_controller;
mod ns_application;

pub use self::ns_object::*;
pub use self::ns_responder::*;
pub use self::ns_window::*;
pub use self::ns_window_controller::*;
pub use self::ns_application::*;

pub type NSEvent = NSObject;
pub type NSViewController = NSObject;
pub type NSStoryboard = NSObject;
pub type NSNotification = NSObject;
pub type NSMenu = NSObject;
pub type NSMenuItem = NSObject;
