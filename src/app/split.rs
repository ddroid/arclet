use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Paned, Widget};

use crate::backend::{create_backend, TerminalBackend};
use crate::config::TerminalConfig;

pub enum SplitDirection {
    Horizontal,
    Vertical,
}

pub struct TerminalPane {
    container: GtkBox,
    active_pane_id: Rc<RefCell<usize>>,
    panes: Rc<RefCell<Vec<(usize, Box<dyn TerminalBackend>)>>>,
    config: TerminalConfig,
    next_id: Rc<RefCell<usize>>,
}

impl TerminalPane {
    pub fn new(config: &TerminalConfig) -> Result<Self, anyhow::Error> {
        let container = GtkBox::new(Orientation::Vertical, 0);
        container.set_hexpand(true);
        container.set_vexpand(true);

        let active_pane_id = Rc::new(RefCell::new(1));
        let next_id = Rc::new(RefCell::new(2));
        let panes = Rc::new(RefCell::new(Vec::new()));

        let initial_backend = create_backend(config)?;
        let widget = initial_backend.widget();
        widget.set_hexpand(true);
        widget.set_vexpand(true);
        container.append(&widget);

        panes.borrow_mut().push((1, initial_backend));

        Ok(Self {
            container,
            active_pane_id,
            panes,
            config: config.clone(),
            next_id,
        })
    }

    pub fn widget(&self) -> Widget {
        self.container.clone().upcast::<Widget>()
    }

    pub fn split(&self, direction: SplitDirection) -> Result<(), anyhow::Error> {
        let new_backend = create_backend(&self.config)?;
        let new_id = {
            let mut id = self.next_id.borrow_mut();
            let val = *id;
            *id += 1;
            val
        };

        let new_widget = new_backend.widget();
        new_widget.set_hexpand(true);
        new_widget.set_vexpand(true);

        self.panes.borrow_mut().push((new_id, new_backend));
        *self.active_pane_id.borrow_mut() = new_id;

        // Take existing child of container and wrap in Paned
        if let Some(first_child) = self.container.first_child() {
            self.container.remove(&first_child);

            let paned_orientation = match direction {
                SplitDirection::Horizontal => Orientation::Horizontal,
                SplitDirection::Vertical => Orientation::Vertical,
            };

            let paned = Paned::new(paned_orientation);
            paned.set_hexpand(true);
            paned.set_vexpand(true);
            paned.set_start_child(Some(&first_child));
            paned.set_end_child(Some(&new_widget));
            paned.set_position(400);

            self.container.append(&paned);
        } else {
            self.container.append(&new_widget);
        }

        new_widget.grab_focus();
        Ok(())
    }

    pub fn grab_focus(&self) {
        let active_id = *self.active_pane_id.borrow();
        let panes = self.panes.borrow();
        if let Some((_, backend)) = panes.iter().find(|(id, _)| *id == active_id) {
            backend.grab_focus();
        } else if let Some((_, backend)) = panes.first() {
            backend.grab_focus();
        }
    }

    pub fn active_backend_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Box<dyn TerminalBackend>) -> R,
    {
        let active_id = *self.active_pane_id.borrow();
        let mut panes = self.panes.borrow_mut();
        if let Some((_, backend)) = panes.iter_mut().find(|(id, _)| *id == active_id) {
            Some(f(backend))
        } else if let Some((_, backend)) = panes.first_mut() {
            Some(f(backend))
        } else {
            None
        }
    }

    pub fn is_alive(&self) -> bool {
        let panes = self.panes.borrow();
        panes.iter().any(|(_, b)| b.is_alive())
    }
}
