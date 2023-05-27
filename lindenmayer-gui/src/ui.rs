use gtk::{prelude::*, Orientation::*, *};

use crate::helpers::*;

pub struct UiElements {
    pub window: ApplicationWindow,
    pub editor_config_box: gtk::Box,
    pub editor_vars_box: gtk::Box,
    pub editor_operations_box: gtk::Box,
    pub editor_rules_box: gtk::Box,
    pub drawing_area: DrawingArea,
    pub toggle_anim_button: Button,
    pub reset_anim_button: Button,
    pub status_label: Label,
    pub frame_label: Label,
    pub time_label: Label,
    pub elapsed_label: Label,
    pub length_label: Label,
    pub clear_button: Button,
    pub import_button: Button,
    pub export_button: Button,
}

pub fn init_elements() -> UiElements {
    log::info!("Building UI");

    let window = ApplicationWindow::builder()
        .title("Lindenmayer Garden")
        .build();

    let header_bar = HeaderBar::new();
    window.set_titlebar(Some(&header_bar));

    let clear_button = Button::builder().label("Clear").build();
    let import_button = Button::builder().label("Import").build();
    let export_button = Button::builder().label("Export").build();

    header_bar.pack_end(&clear_button);
    header_bar.pack_end(&import_button);
    header_bar.pack_end(&export_button);

    let container = gtk::Box::new(Horizontal, 5);
    let left_box = gtk::Box::new(Vertical, 5);
    let right_box = gtk::Box::new(Vertical, 5);
    left_box.set_hexpand(false);

    macro_rules! scrolled_window {
        () => {
            ScrolledWindow::builder()
                .hscrollbar_policy(PolicyType::Never)
                .vscrollbar_policy(PolicyType::Automatic)
                .hexpand(true) // expand and shrink inputs with space
                .propagate_natural_width(true)
                .build()
        };
    }

    macro_rules! editor_box {
        () => {
            gtk::Box::builder()
                .orientation(Vertical)
                .spacing(5)
                .vexpand(true)
                .build()
        };
    }

    let editor_config_scroller = scrolled_window!();
    let editor_vars_scroller = scrolled_window!();
    let editor_operations_scroller = scrolled_window!();
    let editor_rules_scroller = scrolled_window!();

    let editor_config_box = editor_box!();
    let editor_vars_box = editor_box!();
    let editor_operations_box = editor_box!();
    let editor_rules_box = editor_box!();

    let container_toggle_config = create_retract_button(
        "Configuration", //
        &editor_config_box,
        &editor_config_scroller,
    );
    let container_toggle_variables = create_retract_button(
        "Variables", //
        &editor_vars_box,
        &editor_vars_scroller,
    );
    let container_toggle_operations = create_retract_button(
        "Operations", //
        &editor_operations_box,
        &editor_operations_scroller,
    );
    let container_toggle_rules = create_retract_button(
        "Rules", //
        &editor_rules_box,
        &editor_rules_scroller,
    );

    let status_label = Label::new(Some(""));
    status_label.set_visible(false);

    let drawing_area = DrawingArea::new();

    let playback = gtk::Box::builder()
        .orientation(Horizontal)
        .spacing(30)
        .baseline_position(BaselinePosition::Center)
        .hexpand(true)
        .build();

    let toggle_anim_button = Button::builder().label("Play/Stop").build();
    let reset_anim_button = Button::builder().label("Reset").build();

    let frame_label = Label::new(Some("Frame: 0"));
    let time_label = Label::new(Some("Time: 0"));
    let elapsed_label = Label::new(Some("Elapsed: 0 ms"));
    let length_label = Label::new(Some("Length: 0"));

    editor_config_scroller.set_child(Some(&editor_config_box));
    editor_vars_scroller.set_child(Some(&editor_vars_box));
    editor_operations_scroller.set_child(Some(&editor_operations_box));
    editor_rules_scroller.set_child(Some(&editor_rules_box));

    playback.append(&toggle_anim_button);
    playback.append(&reset_anim_button);
    playback.append(&length_label);
    playback.append(&frame_label);
    playback.append(&time_label);
    playback.append(&elapsed_label);

    left_box.append(&drawing_area);
    left_box.append(&playback);

    right_box.append(&status_label);
    right_box.append(&container_toggle_config);
    right_box.append(&editor_config_scroller);
    right_box.append(&container_toggle_variables);
    right_box.append(&editor_vars_scroller);
    right_box.append(&container_toggle_operations);
    right_box.append(&editor_operations_scroller);
    right_box.append(&container_toggle_rules);
    right_box.append(&editor_rules_scroller);

    container.append(&left_box);
    container.append(&right_box);

    window.set_child(Some(&container));

    window.present();

    UiElements {
        window,
        editor_config_box,
        editor_vars_box,
        editor_operations_box,
        editor_rules_box,
        drawing_area,
        toggle_anim_button,
        reset_anim_button,
        status_label,
        frame_label,
        time_label,
        elapsed_label,
        length_label,
        clear_button,
        import_button,
        export_button,
    }
}
