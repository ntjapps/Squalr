/// Trait for converting view data back to model data
pub trait ConvertFromViewData<TViewData, TModel> {
    /// Convert view data to model
    fn convert_from_view_data(&self, view_data: &TViewData) -> TModel;
}
