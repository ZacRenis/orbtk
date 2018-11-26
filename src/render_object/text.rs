use backend::Renderer;
use render_object::RenderObject;
use structs::{Label, Point, Rect};
use theme::{Selector, Theme};
use widget::WidgetContainer;

pub struct TextRenderObject;

impl Into<Box<RenderObject>> for TextRenderObject {
    fn into(self) -> Box<RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for TextRenderObject {
    fn render(
        &self,
        renderer: &mut Renderer,
        widget: &WidgetContainer,
        theme: &Theme,
        offset: &Point,
        global_position: &Point,
    ) {
        if let Ok(selector) = widget.borrow_property::<Selector>() {
            if let Ok(bounds) = widget.borrow_property::<Rect>() {
                if let Ok(label) = widget.borrow_property::<Label>() {
                    if let Ok(parent_bounds) = widget.borrow_parent_property::<Rect>() {
                        renderer.render_text(
                            theme,
                            &label.0,
                            bounds,
                            parent_bounds,
                            selector,
                            offset,
                            global_position,
                        );
                    }
                }
            }
        }
    }
}
