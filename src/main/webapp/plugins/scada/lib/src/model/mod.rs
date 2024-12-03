
pub mod traits;
pub mod common;
pub mod diagram;
pub mod widget;
pub mod mx_cell;
pub mod mx_editor;
pub mod mx_utils;
pub mod mx_graph;
pub mod mx_graph_model;
pub mod mx_rectangle;
pub mod editor_ui;
pub mod cell_meta;
pub mod widget_group;

#[macro_export]
macro_rules! rrefcell {
    ($var:ident) => (
        Rc::new(RefCell::new($var))
    );
}