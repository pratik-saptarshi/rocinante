use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{define_class, msg_send, sel, DefinedClass, MainThreadOnly};
use objc2_app_kit::{NSApplication, NSApplicationActivationOptions, NSRunningApplication};
use objc2_foundation::{
    MainThreadMarker, NSAppleEventDescriptor, NSAppleEventManager, NSObject, NSObjectProtocol,
};
use std::sync::{mpsc::Sender, Arc, Mutex};

const INTERNET_EVENT_CLASS: u32 = u32::from_be_bytes(*b"GURL");
const GET_URL_EVENT_ID: u32 = u32::from_be_bytes(*b"GURL");
const DIRECT_OBJECT_KEY: u32 = u32::from_be_bytes(*b"----");
const MAX_URL_BYTES: usize = 8 * 1024;

struct UrlHandlerIvars {
    sender: Sender<String>,
    repaint_context: Arc<Mutex<Option<eframe::egui::Context>>>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "RocinanteURLAppleEventHandler"]
    #[ivars = UrlHandlerIvars]
    struct UrlHandler;

    unsafe impl NSObjectProtocol for UrlHandler {}

    impl UrlHandler {
        #[unsafe(method(handleGetURLEvent:withReplyEvent:))]
        fn handle_get_url_event(
            &self,
            event: &NSAppleEventDescriptor,
            _reply: &NSAppleEventDescriptor,
        ) {
            let direct_object: Option<Retained<NSAppleEventDescriptor>> = unsafe {
                msg_send![event, paramDescriptorForKeyword: DIRECT_OBJECT_KEY]
            };
            let Some(direct_object) = direct_object else {
                return;
            };
            let Some(url) = direct_object.stringValue() else {
                return;
            };
            let url = url.to_string();
            if url.len() > MAX_URL_BYTES || super::deep_link::parse_deep_link(&url).is_none() {
                return;
            }
            let repaint_context = self
                .ivars()
                .repaint_context
                .lock()
                .ok()
                .and_then(|context| context.clone());
            if self.ivars().sender.send(url).is_ok() {
                if let Some(context) = repaint_context {
                    context.send_viewport_cmd(eframe::egui::ViewportCommand::Visible(true));
                    context.send_viewport_cmd(eframe::egui::ViewportCommand::Minimized(false));
                    context.send_viewport_cmd(eframe::egui::ViewportCommand::Focus);
                    let _ = activate_application();
                    context.request_repaint();
                }
            }
        }
    }
);

pub struct UrlEventHandler {
    _handler: Retained<UrlHandler>,
}

pub fn activate_application() -> bool {
    let running_application = NSRunningApplication::currentApplication();
    let accepted =
        running_application.activateWithOptions(NSApplicationActivationOptions::ActivateAllWindows);
    if let Some(main_thread) = MainThreadMarker::new() {
        NSApplication::sharedApplication(main_thread).activate();
    }
    accepted
}

pub fn request_application_termination_after_ui_pass() {
    if let Some(main_thread) = MainThreadMarker::new() {
        let application = NSApplication::sharedApplication(main_thread);
        // Winit closes its windows from applicationWillTerminate. Deferring the
        // request lets the current eframe UI callback return before that happens.
        unsafe {
            let _: () = msg_send![
                &*application,
                performSelector: sel!(terminate:),
                withObject: None::<&AnyObject>,
                afterDelay: 0.01
            ];
        }
    }
}

pub fn register(
    sender: Sender<String>,
    repaint_context: Arc<Mutex<Option<eframe::egui::Context>>>,
) -> Result<UrlEventHandler, Box<dyn std::error::Error + Send + Sync>> {
    let handler: Retained<UrlHandler> = unsafe {
        let main_thread = MainThreadMarker::new().ok_or_else(|| {
            std::io::Error::other("macOS URL registration requires the main thread")
        })?;
        let _application = NSApplication::sharedApplication(main_thread);
        let allocated = UrlHandler::alloc(main_thread).set_ivars(UrlHandlerIvars {
            sender,
            repaint_context,
        });
        msg_send![super(allocated), init]
    };
    let manager = NSAppleEventManager::sharedAppleEventManager();
    unsafe {
        let _: () = msg_send![&*manager, setEventHandler: (&*handler as &AnyObject), andSelector: sel!(handleGetURLEvent:withReplyEvent:), forEventClass: INTERNET_EVENT_CLASS, andEventID: GET_URL_EVENT_ID];
    }
    Ok(UrlEventHandler { _handler: handler })
}

impl Drop for UrlEventHandler {
    fn drop(&mut self) {
        let manager = NSAppleEventManager::sharedAppleEventManager();
        unsafe {
            let _: () = msg_send![&*manager, removeEventHandlerForEventClass: INTERNET_EVENT_CLASS, andEventID: GET_URL_EVENT_ID];
        }
    }
}
