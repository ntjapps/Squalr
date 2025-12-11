/// Trait for converting model data to view data
pub trait ConvertToViewData<TModel, TViewData> {
    /// Convert a collection of models to view data
    fn convert_collection(&self, models: &Vec<TModel>) -> Vec<TViewData>;

    /// Convert a single model to view data
    fn convert_to_view_data(&self, model: &TModel) -> TViewData;
}
