use slint::{ComponentHandle, Weak};
use std::sync::{Arc, Mutex};

/// Binding between a view model and a Slint view component
pub struct ViewBinding<T: ComponentHandle> {
    view_handle: Arc<Mutex<Weak<T>>>,
}

impl<T: ComponentHandle> ViewBinding<T> {
    /// Create a new view binding from a weak reference to a component
    pub fn new(weak: Weak<T>) -> Self {
        Self {
            view_handle: Arc::new(Mutex::new(weak)),
        }
    }

    /// Get the weak reference to the view
    pub fn get_view_handle(&self) -> &Arc<Mutex<Weak<T>>> {
        &self.view_handle
    }

    /// Execute a closure on the UI thread with access to the view
    pub fn execute_on_ui_thread<F>(&self, f: F)
    where
        F: FnOnce(T, Arc<ViewBinding<T>>) + Send + 'static,
        T: 'static,
    {
        let view_handle = self.view_handle.clone();
        let view_binding = Arc::new(ViewBinding {
            view_handle: self.view_handle.clone(),
        });

        slint::invoke_from_event_loop(move || {
            if let Ok(handle) = view_handle.lock() {
                if let Some(view) = handle.upgrade() {
                    f(view, view_binding);
                }
            }
        })
        .ok();
    }
}

impl<T: ComponentHandle> Clone for ViewBinding<T> {
    fn clone(&self) -> Self {
        Self {
            view_handle: self.view_handle.clone(),
        }
    }
}
