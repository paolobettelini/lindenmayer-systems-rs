use gtk::{glib::*, prelude::*, *};
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use lindenmayer_parser::{export, *};
use lindenmayer_renderer::LSystemRenderer;

use crate::animations::*;
use crate::config::*;
use crate::helpers::*;
use crate::logic::gio::Cancellable;
use crate::ui::*;

pub fn initialize_config(application: &gtk::Application, configuration_lines: &str) {
    log::info!("Initializing UI content");

    // Init UI elements
    let ui_elements = init_elements();
    ui_elements.window.set_application(Some(application));

    let window = &ui_elements.window;
    let drawing_area = &Rc::new(ui_elements.drawing_area);
    let toggle_anim_button = &ui_elements.toggle_anim_button;
    let reset_anim_button = &ui_elements.reset_anim_button;
    let status_label = &ui_elements.status_label;
    let frame_label = &Rc::new(ui_elements.frame_label);
    let time_label = &Rc::new(ui_elements.time_label);
    let elapsed_label = &Rc::new(ui_elements.elapsed_label);
    let length_label = &Rc::new(ui_elements.length_label);
    let clear_button = &ui_elements.clear_button;
    let import_button = &ui_elements.import_button;
    let export_button = &ui_elements.export_button;
    let editor_config_box = ui_elements.editor_config_box;
    let editor_vars_box = ui_elements.editor_vars_box;
    let editor_operations_box = ui_elements.editor_operations_box;
    let editor_rules_box = ui_elements.editor_rules_box;

    // Logic structures
    let conf = Rc::new(RefCell::new(ConfEditor::default()));
    let fractal = Rc::new(RefCell::new(LSystemRenderer::default()));
    let animator = LSystemAnimator::new(
        fractal,
        drawing_area.clone(),
        status_label,
        frame_label.clone(),
        time_label.clone(),
        elapsed_label.clone(),
        length_label.clone(),
    );
    let animator = Rc::new(RefCell::new(animator));

    // Playback logic
    toggle_anim_button.connect_clicked(clone!(@weak animator => move |_| {
        animator.borrow().toggle_playing();
    }));

    reset_anim_button.connect_clicked(clone!(@weak animator => move |_| {
        animator.borrow().reset();
    }));

    // Clear/Import/Export logic

    // Channel to signal imports
    let (sender, receiver) = MainContext::channel::<String>(Priority::default());
    receiver.attach(
        None,
        clone!(
            @weak editor_config_box,
            @weak editor_vars_box,
            @weak editor_operations_box,
            @weak editor_rules_box,
            @weak conf,
            @weak animator => @default-return Continue(true),
            move |text| {
                init_data(
                    &text,
                    &editor_config_box,
                    &editor_vars_box,
                    &editor_operations_box,
                    &editor_rules_box,
                    &animator,
                    conf,
                    true
                );

                Continue(true)
            }
        ),
    );

    // Initialize default data
    init_data(
        configuration_lines,
        &editor_config_box,
        &editor_vars_box,
        &editor_operations_box,
        &editor_rules_box,
        &animator,
        conf.clone(),
        false,
    );

    clear_button.connect_clicked(clone!(
        @weak conf,
        @weak editor_config_box,
        @weak editor_vars_box,
        @weak editor_operations_box,
        @weak editor_rules_box,
        @weak animator => move |_| {
        init_data(
            "", // Initialize with empty configuration
            &editor_config_box,
            &editor_vars_box,
            &editor_operations_box,
            &editor_rules_box,
            &animator,
            conf,
            true
        );
    }));

    import_button.connect_clicked(clone!(@weak window, @strong sender => move |_| {
        let file_dialog = FileDialog::new();
        let parent = Some(&window);

        let sender = sender.clone();

        let callback = move |res| {
            if let Ok(file) = res {
                log::info!("Importing configuration");
                let content = get_content(&file);

                if let Some(text) = content {
                    // Send to context thread
                    let res = sender.send(text);

                    if res.is_err() {
                        log::error!("Could not send configuration to context thread");
                    }
                } else {
                    log::info!("Could not read file");
                }
            }
        };

        file_dialog.open(parent, Some(&Cancellable::new()), callback);
    }));

    export_button.connect_clicked(clone!(@weak conf, @weak window => move |_| {
        let conf = conf.borrow();
        let content = export::serialize_renderer(&conf);

        let file_dialog = FileDialog::new();

        let callback = move |res| {
            if let Ok(file) = res {
                log::info!("Exporting configuration");
                let write_res = write_content(&file, &content);

                if write_res.is_none() {
                    log::error!("Could not save file");
                }
            }
        };

        file_dialog.save(Some(&window), Some(&Cancellable::new()), callback);
    }));
}

fn init_data(
    configuration_lines: &str,
    editor_config_box: &gtk::Box,
    editor_vars_box: &gtk::Box,
    editor_operations_box: &gtk::Box,
    editor_rules_box: &gtk::Box,
    animator: &Rc<RefCell<LSystemAnimator>>,
    conf: Rc<RefCell<ConfEditor>>,
    clear: bool,
) {
    log::info!("Initializing configuration");

    if clear {
        clear_editor(
            &conf,
            editor_config_box,
            editor_vars_box,
            editor_operations_box,
            editor_rules_box,
        );
    }

    // New renderer
    let mut renderer = LSystemRenderer::default();
    let mut conf_ref = conf.borrow_mut();

    let mut config_lines = ConfigLines::default();

    // Initialization
    for line in configuration_lines.lines() {
        let error = renderer.update(line).is_err();
        let line_type = renderer.get_line_type(line);

        macro_rules! create_dyamic_entry {
            ($container:expr) => {{
                create_dynamic_entry(
                    &conf,
                    &mut conf_ref,
                    line_type,
                    &animator,
                    $container,
                    line,
                    error,
                );
            }};
        }

        match line_type {
            LineType::Config => {
                config_lines.update(line, error);
                // Initialize entries after using the initialized fractal
            }
            LineType::Variable => create_dyamic_entry!(editor_vars_box),
            LineType::Rule => create_dyamic_entry!(editor_rules_box),
            LineType::Operation => create_dyamic_entry!(editor_operations_box),
            LineType::Unknown => { /* Discard */ }
            LineType::Nothing => { /* Discard */ }
        }
    }

    // Add empty entries for extra configuration
    add_empty_entries(
        &conf,
        &mut conf_ref,
        animator,
        editor_vars_box,
        editor_rules_box,
        editor_operations_box,
    );

    // Initialize config editor section

    init_config_editor(
        &config_lines,
        &conf,
        &mut conf_ref,
        editor_config_box,
        animator,
    );

    // Send to render
    (*animator.borrow_mut()).update_renderer(renderer);
}

pub fn update_renderer(renderer: &mut LSystemRenderer, conf: &ConfEditor) -> bool {
    let mut errors = false;

    let iterator = conf
        .configurations
        .iter()
        .chain(conf.rules.iter())
        .chain(conf.operations.iter())
        .chain(conf.variables.iter());

    for line in iterator {
        if !line.is_empty() {
            let res = renderer.update(line);

            if res.is_err() {
                // The configuration is not ready
                log::warn!("Configuration is incorrect");
                errors = true;
                break;
            }
        }
    }

    errors
}

pub fn add_empty_entries(
    conf: &Rc<RefCell<ConfEditor>>,
    conf_ref: &mut RefMut<ConfEditor>,
    animator: &Rc<RefCell<LSystemAnimator>>,
    editor_vars_box: &gtk::Box,
    editor_rules_box: &gtk::Box,
    editor_operations_box: &gtk::Box,
) {
    macro_rules! create_dynamic_entry {
        ($line_type:expr, $container:expr) => {
            create_dynamic_entry(conf, conf_ref, $line_type, animator, $container, "", false)
        };
    }

    create_dynamic_entry!(LineType::Variable, editor_vars_box);
    create_dynamic_entry!(LineType::Rule, editor_rules_box);
    create_dynamic_entry!(LineType::Operation, editor_operations_box);
}

fn init_config_editor(
    config_lines: &ConfigLines,
    conf: &Rc<RefCell<ConfEditor>>,
    conf_ref: &mut RefMut<ConfEditor>,
    editor_config_box: &gtk::Box,
    animator: &Rc<RefCell<LSystemAnimator>>,
) {
    let values = [
        (
            "axiom",
            "Axiom",
            &config_lines.axiom.0,
            config_lines.axiom.1,
        ),
        (
            "iter",
            "Iterations",
            &config_lines.iter.0,
            config_lines.iter.1,
        ),
        (
            "initial_pos",
            "Initial Position",
            &config_lines.initial_pos.0,
            config_lines.initial_pos.1,
        ),
        (
            "initial_rot",
            "Initial Rotation",
            &config_lines.initial_rot.0,
            config_lines.initial_rot.1,
        ),
        (
            "initial_thickness",
            "Initial Thickness",
            &config_lines.initial_thickness.0,
            config_lines.initial_thickness.1,
        ),
        (
            "background",
            "Background Color",
            &config_lines.background_color.0,
            config_lines.background_color.1,
        ),
        (
            "initial_color",
            "Initial Color",
            &config_lines.initial_color.0,
            config_lines.initial_color.1,
        ),
        (
            "canvas",
            "Canvas Size",
            &config_lines.canvas.0,
            config_lines.canvas.1,
        ),
        (
            "seed",
            "Random Seed",
            &config_lines.seed.0,
            config_lines.seed.1,
        ),
        (
            "inject",
            "Injection",
            &config_lines.injections.0,
            config_lines.injections.1,
        ),
    ];

    for value in values {
        create_config_entry(
            conf,
            conf_ref,
            editor_config_box,
            animator,
            value.0,
            value.1,
            value.2,
            value.3,
        );
    }
}
