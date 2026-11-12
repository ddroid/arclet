use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Box as GtkBox, Button, Label, Notebook, Orientation, Widget};

use super::split::{SplitDirection, TerminalPane};
use crate::config::TerminalConfig;

pub struct TabManager {
    notebook: Notebook,
    tabs: Rc<RefCell<Vec<Rc<TerminalPane>>>>,
    config: TerminalConfig,
}

impl TabManager {
    pub fn new(config: &TerminalConfig) -> Result<Self, anyhow::Error> {
        let notebook = Notebook::builder()
            .hexpand(true)
            .vexpand(true)
            .show_border(false)
            .scrollable(true)
            .build();

        let tabs = Rc::new(RefCell::new(Vec::new()));
        let manager = Self {
            notebook,
            tabs,
            config: config.clone(),
        };

        // Open initial tab
        manager.new_tab()?;

        Ok(manager)
    }

    pub fn widget(&self) -> Widget {
        self.notebook.clone().upcast::<Widget>()
    }

    pub fn new_tab(&self) -> Result<(), anyhow::Error> {
        let tab_index = self.tabs.borrow().len() + 1;
        let pane = Rc::new(TerminalPane::new(&self.config)?);

        // Custom tab header with title and close button
        let tab_box = GtkBox::new(Orientation::Horizontal, 6);
        let title_label = Label::new(Some(&format!("Terminal {tab_index}")));
        let close_btn = Button::builder()
            .icon_name("window-close-symbolic")
            .has_frame(false)
            .tooltip_text("Close Tab")
            .build();

        tab_box.append(&title_label);
        tab_box.append(&close_btn);

        let pane_widget = pane.widget();
        let page_num = self.notebook.append_page(&pane_widget, Some(&tab_box));
        self.notebook.set_tab_reorderable(&pane_widget, true);

        self.tabs.borrow_mut().push(pane.clone());
        self.notebook.set_current_page(Some(page_num));
        pane.grab_focus();

        // Close button click handler
        let notebook_clone = self.notebook.clone();
        let tabs_clone = self.tabs.clone();
        let pane_widget_clone = pane_widget.clone();
        close_btn.connect_clicked(move |_| {
            if let Some(pos) = notebook_clone.page_num(&pane_widget_clone) {
                notebook_clone.remove_page(Some(pos));
                let mut tabs = tabs_clone.borrow_mut();
                if (pos as usize) < tabs.len() {
                    tabs.remove(pos as usize);
                }
            }
        });

        // Hide tab bar if only 1 tab, show if >= 2
        self.update_tab_bar_visibility();

        Ok(())
    }

    pub fn close_current_tab(&self) {
        if let Some(current_page) = self.notebook.current_page() {
            self.notebook.remove_page(Some(current_page));
            let mut tabs = self.tabs.borrow_mut();
            if (current_page as usize) < tabs.len() {
                tabs.remove(current_page as usize);
            }
        }
        self.update_tab_bar_visibility();
    }

    pub fn next_tab(&self) {
        let current = self.notebook.current_page().unwrap_or(0);
        let total = self.notebook.n_pages();
        if total > 1 {
            let next = (current + 1) % total;
            self.notebook.set_current_page(Some(next));
            if let Some(pane) = self.tabs.borrow().get(next as usize) {
                pane.grab_focus();
            }
        }
    }

    pub fn prev_tab(&self) {
        let current = self.notebook.current_page().unwrap_or(0);
        let total = self.notebook.n_pages();
        if total > 1 {
            let prev = if current == 0 { total - 1 } else { current - 1 };
            self.notebook.set_current_page(Some(prev));
            if let Some(pane) = self.tabs.borrow().get(prev as usize) {
                pane.grab_focus();
            }
        }
    }

    pub fn current_pane(&self) -> Option<Rc<TerminalPane>> {
        let current = self.notebook.current_page().unwrap_or(0) as usize;
        self.tabs.borrow().get(current).cloned()
    }

    pub fn split_horizontal(&self) -> Result<(), anyhow::Error> {
        if let Some(pane) = self.current_pane() {
            pane.split(SplitDirection::Horizontal)?;
        }
        Ok(())
    }

    pub fn split_vertical(&self) -> Result<(), anyhow::Error> {
        if let Some(pane) = self.current_pane() {
            pane.split(SplitDirection::Vertical)?;
        }
        Ok(())
    }

    fn update_tab_bar_visibility(&self) {
        let count = self.notebook.n_pages();
        self.notebook.set_show_tabs(count > 1);
    }
}
