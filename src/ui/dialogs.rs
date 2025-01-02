use std::error::Error;
use std::backtrace::Backtrace;

use relm4::prelude::*;
use adw::prelude::*;

/// Display critical error dialog. It will allow user to look
/// through the given error, current thread's backtrace, and
/// close the app safely.
pub fn critical_error(title: impl AsRef<str>, error: Box<dyn Error>, parent: Option<&adw::Window>) {
    let backtrace = Backtrace::force_capture().to_string();

    let dialog = adw::AlertDialog::builder()
        .heading(title.as_ref())
        .body(error.to_string())
        .can_close(false)
        .build();

    dialog.add_responses(&[
        ("close", "Close"),
        ("backtrace", "Backtrace")
    ]);

    dialog.set_response_appearance("close", adw::ResponseAppearance::Destructive);

    dialog.connect_response(Some("close"), |_, _| {
        relm4::main_adw_application().quit();
    });

    let backtrace_parent = dialog.clone();

    dialog.connect_response(Some("backtrace"), move |_, _| {
        relm4::view! {
            dialog = adw::Dialog {
                set_title: "Backtrace",
                set_size_request: (900, 700),

                #[wrap(Some)]
                set_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    adw::HeaderBar,

                    gtk::ScrolledWindow {
                        set_hexpand: true,
                        set_vexpand: true,

                        gtk::Label {
                            set_halign: gtk::Align::Start,

                            set_selectable: true,

                            set_label: backtrace.as_str()
                        }
                    }
                }
            }
        }

        dialog.present(Some(&backtrace_parent));
    });

    if parent.is_some() {
        dialog.present(parent);
    } else if let Some(window) = relm4::main_adw_application().active_window() {
        dialog.present(Some(&window));
    } else {
        dialog.present(None as Option<&adw::Window>);
    }
}
