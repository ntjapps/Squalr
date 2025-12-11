use slint::Model;
use std::sync::Arc;

/// Binding for collections of view model data
pub struct ViewCollectionBinding<T> {
    model: Arc<dyn Model<Data = T>>,
}

impl<T> ViewCollectionBinding<T> {
    /// Create a new view collection binding from a model
    pub fn new(model: Arc<dyn Model<Data = T>>) -> Self {
        Self { model }
    }

    /// Get the underlying model
    pub fn get_model(&self) -> &Arc<dyn Model<Data = T>> {
        &self.model
    }
}

impl<T> Clone for ViewCollectionBinding<T> {
    fn clone(&self) -> Self {
        Self {
            model: self.model.clone(),
        }
    }
}
