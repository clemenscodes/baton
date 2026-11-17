use crate::layer::Layer;
use crate::layer::PresentationLayer;
use crate::view::View;

pub trait Model: for<'view> From<&'view Self::View>
    + Clone
    + PartialEq
    + Layer<Layer = PresentationLayer>
{
    type View: View;
}
