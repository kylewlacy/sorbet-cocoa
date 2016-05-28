use objc;
use objc::runtime as rt;
use {ShareId, Object, AnyObject, RawObjCObject, NSObject, IsNSObject, NSEvent};

#[repr(C)]
pub struct NSResponder {
    super_: NSObject
}

unsafe impl objc::Message for NSResponder { }

unsafe impl RawObjCObject for NSResponder { }

impl Object for NSResponder {
    type Super = NSObject;

    fn super_ref(&self) -> &Self::Super {
        &self.super_
    }

    fn super_mut(&mut self) -> &mut Self::Super {
        &mut self.super_
    }
}

pub trait IsNSResponder: IsNSObject {
    fn accepts_first_responder(&self) -> bool;
    fn become_first_responder(&self) -> bool;
    fn resign_first_responder(&self) -> bool;
    fn validate_proposed_first_responder_for_event(&self, responder: ShareId<NSResponder>, event: Option<ShareId<NSEvent>>) -> bool;

    unsafe fn next_responder(&self) -> Option<ShareId<NSResponder>>;
    unsafe fn set_next_responder(&self, next_responder: Option<ShareId<NSResponder>>);

    fn mouse_down(&self, event: ShareId<NSEvent>);
    fn mouse_dragged(&self, event: ShareId<NSEvent>);
    fn mouse_up(&self, event: ShareId<NSEvent>);
    fn mouse_moved(&self, event: ShareId<NSEvent>);
    fn mouse_entered(&self, event: ShareId<NSEvent>);
    fn mouse_exited(&self, event: ShareId<NSEvent>);
    fn right_mouse_down(&self, event: ShareId<NSEvent>);
    fn right_mouse_dragged(&self, event: ShareId<NSEvent>);
    fn right_mouse_up(&self, event: ShareId<NSEvent>);
    fn other_mouse_down(&self, event: ShareId<NSEvent>);
    fn other_mouse_dragged(&self, event: ShareId<NSEvent>);
    fn other_mouse_up(&self, event: ShareId<NSEvent>);

    // fn key_down(&self, event: ShareId<NSEvent>);
    // fn key_up(&self, event: ShareId<NSEvent>);
    // fn interpret_key_events(&self, event: Vec<ShareId<NSEvent>>);
    // fn perform_key_equivalent(&self, event: ShareId<NSEvent>) -> bool;
    // fn flush_buffered_key_events(&self);
    //
    // fn pressure_change_with_event(&self, event: ShareId<NSEvent>);
    // fn cursor_update(&self, event: ShareId<NSEvent>);
    // fn flags_changed(&self, event: ShareId<NSEvent>);
    // fn tablet_point(&self, event: ShareId<NSEvent>);
    // fn tablet_proximity(&self, event: ShareId<NSEvent>);
    // fn help_requested(&self, event: ShareId<NSEvent>);
    // fn scroll_wheel(&self, event: ShareId<NSEvent>);
    // fn quick_look_with_event(&self, event: ShareId<NSEvent>);
    //
    // fn cancel_operation(&self, sender: Option<ShareId<NSObject>>);
    // fn capitalize_word(&self, sender: Option<ShareId<NSObject>>);
    // fn center_selection_in_visible_area(&self, sender: Option<ShareId<NSObject>>);
    // fn change_case_of_letter(&self, sender: Option<ShareId<NSObject>>);
    // fn complete(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_backward(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_backward_by_decomposing_previous_character(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_forward(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_to_beginning_of_line(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_to_beginning_of_paragraph(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_to_end_of_line(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_to_end_of_paragraph(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_to_mark(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_word_backward(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_word_forward(&self, sender: Option<ShareId<NSObject>>);
}

objc_trait! {
    pub unsafe objc trait IsNSResponder: IsNSObject {
        type Base = NSResponder;
        trait Sub = SubNSResponder;

        fn accepts_first_responder(&self) -> bool
            => [self, acceptsFirstResponder] -> rt::BOOL;
        fn become_first_responder(&self) -> bool
            => [self, becomeFirstResponder] -> rt::BOOL;
        fn resign_first_responder(&self) -> bool
            => [self, resignFirstResponder] -> rt::BOOL;
        fn validate_proposed_first_responder_for_event(
            &self,
            responder: ShareId<NSResponder>,
            event: Option<ShareId<NSEvent>>
        )
            -> bool
            => [self, validateProposedFirstResponder:(responder: *mut AnyObject)
                                            forEvent:(event: *mut AnyObject)]
                -> rt::BOOL;

        unsafe fn next_responder(&self) -> Option<ShareId<NSResponder>>
            => [self, nextResponder] -> *mut AnyObject;
        unsafe fn set_next_responder(
            &self,
            next_responder: Option<ShareId<NSResponder>>
        )
            => [self, setNextResponder:(responder:*mut AnyObject)];

        fn mouse_down(&self, event: ShareId<NSEvent>)
            => [self, mouseDown:(event: *mut AnyObject)];
        fn mouse_dragged(&self, event: ShareId<NSEvent>)
            => [self, mouseDragged:(event: *mut AnyObject)];
        fn mouse_up(&self, event: ShareId<NSEvent>)
            => [self, mouseUp:(event: *mut AnyObject)];
        fn mouse_moved(&self, event: ShareId<NSEvent>)
            => [self, mouseMoved:(event: *mut AnyObject)];
        fn mouse_entered(&self, event: ShareId<NSEvent>)
            => [self, mouseEntered:(event: *mut AnyObject)];
        fn mouse_exited(&self, event: ShareId<NSEvent>)
            => [self, mouseExited:(event: *mut AnyObject)];
        fn right_mouse_down(&self, event: ShareId<NSEvent>)
            => [self, rightMouseDown:(event: *mut AnyObject)];
        fn right_mouse_dragged(&self, event: ShareId<NSEvent>)
            => [self, rightMouseDragged:(event: *mut AnyObject)];
        fn right_mouse_up(&self, event: ShareId<NSEvent>)
            => [self, rightMouseUp:(event: *mut AnyObject)];
        fn other_mouse_down(&self, event: ShareId<NSEvent>)
            => [self, otherMouseDown:(event: *mut AnyObject)];
        fn other_mouse_dragged(&self, event: ShareId<NSEvent>)
            => [self, otherMouseDragged:(event: *mut AnyObject)];
        fn other_mouse_up(&self, event: ShareId<NSEvent>)
            => [self, otherMouseUp:(event: *mut AnyObject)];
    }
}

#[macro_export]
macro_rules! NSResponder {
    ($($args:tt)*) => {
        __objc_inheritance_for! {
            $crate::NSResponder => $crate::SubNSResponder: NSObject!;
            $($args)*
        }
    };
}
