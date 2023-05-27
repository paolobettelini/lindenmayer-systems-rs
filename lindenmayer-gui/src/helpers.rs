use crate::{animations::LSystemAnimator, logic::update_renderer};
use gtk::{glib::*, prelude::*, *};
use lindenmayer_parser::*;
use lindenmayer_renderer::meval;
use lindenmayer_renderer::LSystemRenderer;
use std::{
    cell::{RefCell, RefMut},
    io::Write,
    rc::Rc,
};

const CSS_ERROR_CLASS: &str = "error";

pub fn set_error_status(context: &MainContext, label: &Label, error: &str) {
    let error = error.to_string();
    log::warn!("{error}");

    context.spawn_local(clone!(@weak label => async move {
        label.set_visible(true);
        label.add_css_class(CSS_ERROR_CLASS);
        label.set_text(&error);
    }));
}

pub fn hide_status(context: &MainContext, label: &Label) {
    context.spawn_local(clone!(@weak label => async move {
        label.set_visible(false);
    }));
}

pub fn meval_error_msg(error: &meval::Error) -> String {
    match error {
        meval::Error::UnknownVariable(v) => format!("Invalid var: {v}"),
        meval::Error::Function(f, _) => format!("Invalid func: {f}"),
        _ => format!("Parsing error"),
    }
}

pub fn add_text_input(container: &gtk::Box, placeholder: &str) -> Entry {
    let text_input = Entry::builder().placeholder_text(placeholder).build();

    text_input.add_css_class("conf-textbox");
    container.append(&text_input);

    text_input
}

pub fn create_retract_button(
    name: &str,
    container: &gtk::Box,
    scroller: &gtk::ScrolledWindow,
) -> Button {
    let button = Button::new();
    button.set_label(name);

    let is_expanded = std::cell::Cell::new(true);
    button.connect_clicked(clone!(@weak container, @weak scroller => move |_| {
        is_expanded.set(!is_expanded.get());
        if is_expanded.get() {
            container.set_visible(true);
            scroller.set_min_content_height(200);
        } else {
            container.set_visible(false);
            scroller.set_min_content_height(0);
        }
    }));

    button
}

pub fn get_content(file: &gio::File) -> Option<String> {
    let path = file.path()?;
    if let Ok(content) = std::fs::read_to_string(path) {
        Some(content)
    } else {
        None
    }
}

pub fn write_content(file: &gio::File, content: &str) -> Option<()> {
    let path = file.path()?;

    let mut file = std::fs::File::create(path).ok()?;

    if file.write_all(content.as_bytes()).is_ok() {
        Some(())
    } else {
        None
    }
}

pub fn clear_editor(
    conf: &Rc<RefCell<ConfEditor>>,
    editor_config_box: &gtk::Box,
    editor_vars_box: &gtk::Box,
    editor_operations_box: &gtk::Box,
    editor_rules_box: &gtk::Box,
) {
    let mut conf_ref = conf.borrow_mut();

    conf_ref.variables.clear();
    conf_ref.rules.clear();
    conf_ref.operations.clear();
    conf_ref.configurations.clear();

    while let Some(child) = editor_config_box.first_child() {
        editor_config_box.remove(&child);
    }

    while let Some(child) = editor_vars_box.first_child() {
        editor_vars_box.remove(&child);
    }

    while let Some(child) = editor_operations_box.first_child() {
        editor_operations_box.remove(&child);
    }

    while let Some(child) = editor_rules_box.first_child() {
        editor_rules_box.remove(&child);
    }
}

// Use to insert Entries in the "Configuration" section.
pub fn create_config_entry(
    conf: &Rc<RefCell<ConfEditor>>,
    conf_ref: &mut RefMut<'_, ConfEditor>,
    container: &gtk::Box,
    animator: &Rc<RefCell<LSystemAnimator>>,
    command: &str,
    label_name: &str,
    value: &str,
    error: bool,
) {
    let label = Label::new(Some(label_name));
    container.append(&label);
    let entry = add_text_input(container, "");

    entry.set_text(value);
    let index = conf_ref.configurations.len();
    conf_ref.configurations.push(format!("{command} {value}"));
    let command_ = command.to_string();

    if error {
        log::warn!("Input is incorrect");
        entry.add_css_class(CSS_ERROR_CLASS);
    }

    entry.connect_changed(clone!(@strong conf, @strong animator => move |input| {
        // update value into conf
        let mut conf_ref = conf.borrow_mut();
        if let Some(value) = conf_ref.configurations.get_mut(index) {
            let content: String = input.text().into();
            *value = format!("{command_} {content}");

            let mut new_renderer = LSystemRenderer::default();

            if new_renderer.update(value).is_err() {
                input.add_css_class(CSS_ERROR_CLASS);
                log::warn!("Input is incorrect");
                // we already found an error
                return;
            } else {
                input.remove_css_class(CSS_ERROR_CLASS);
            }

            let errors = update_renderer(&mut new_renderer, &conf_ref);

            if !errors {
                (*animator.borrow_mut()).update_renderer(new_renderer);
            }
        }

    }));
}

pub fn create_dynamic_entry(
    conf: &Rc<RefCell<ConfEditor>>,
    conf_ref: &mut RefMut<ConfEditor>,
    line_type: LineType,
    animator: &Rc<RefCell<LSystemAnimator>>,
    container: &gtk::Box,
    line: &str,
    error: bool,
) {
    let (conf_lines, placeholder) = match line_type {
        LineType::Variable => (&mut conf_ref.variables, "<variable>"),
        LineType::Operation => (&mut conf_ref.operations, "<operation>"),
        LineType::Rule => (&mut conf_ref.rules, "<rule>"),
        _ => return,
    };

    let text_input = add_text_input(container, placeholder);
    text_input.set_text(line);
    let index = conf_lines.len();
    conf_lines.push(line.to_string());

    if error {
        log::warn!("Input is incorrect");
        text_input.add_css_class(CSS_ERROR_CLASS);
    }

    text_input.connect_changed(
        clone!(@weak container, @weak conf, @strong animator => move |input| {
            // Add or remove a input box if necessary
            { // Inner scope to mutably borrow
                let mut conf_ref = conf.borrow_mut();

                let list = match line_type {
                    LineType::Variable => &mut conf_ref.variables,
                    LineType::Operation => &mut conf_ref.operations,
                    LineType::Rule => &mut conf_ref.rules,
                    _ => return,
                };

                let is_empty = input.text().is_empty();
                let list_len = list.len();
                if index + 1 == list_len && !is_empty {
                    // Create new textbox
                    log::debug!("Adding entry");
                    create_dynamic_entry(
                        &conf,
                        &mut conf_ref,
                        line_type,
                        &animator,
                        &container,
                        "",
                        false,
                    );
                } else if index + 2 == list_len && is_empty {
                    if let Some(value) = list.get_mut(index + 1) {
                        if value.is_empty() {
                            log::info!("Removing entry");
                            let child = container.last_child();
                            if let Some(entry) = child {
                                container.remove(&entry);
                                list.pop();
                            }
                        }
                    }
                }
            }

            let mut conf_ref = conf.borrow_mut();

            let list = match line_type {
                LineType::Variable => &mut conf_ref.variables,
                LineType::Operation => &mut conf_ref.operations,
                LineType::Rule => &mut conf_ref.rules,
                _ => return,
            };

            if let Some(value) = list.get_mut(index) {
                // update value in conf
                *value = input.text().into();
                let is_empty = value.is_empty();

                let mut new_renderer = LSystemRenderer::default();

                // Check if the line is not correct
                // => if it's not empty
                // and (it's not of the correct type
                // or it gave an error)
                let error = {
                    !is_empty &&
                    {
                        let res = new_renderer.update(value);
                        if let Ok(v) = res {
                            v != line_type
                        } else {
                            true
                        }
                    }
                };

                // Add or remove error flag accordingly
                if error {
                    input.add_css_class(CSS_ERROR_CLASS);
                    log::warn!("Input is incorrect");
                    // we already found an error
                    return;
                } else {
                    input.remove_css_class(CSS_ERROR_CLASS);
                }

                let errors = update_renderer(&mut new_renderer, &conf_ref);

                // Update fractal and redraw
                if !errors {
                    (*animator.borrow_mut()).update_renderer(new_renderer);
                }
            }
        }),
    );
}

macro_rules! load_css {
    ($file:tt) => {
        log::info!("Loading CSS stylesheet");
        let provider = gtk::CssProvider::new();
        provider.load_from_data(include_str!($file));
        let display = {
            let res = Display::default();
            if let Some(display) = res {
                display
            } else {
                log::error!("Could not retrieve default display");
                std::process::exit(1);
            }
        };
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    };
}

pub(crate) use load_css;
