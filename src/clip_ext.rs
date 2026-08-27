use crate::{AnimotionElement, AnimotionExt, ClipBuilder};
use gpui::*;

pub trait AnimotionClipExt {
    fn animotion_clip<P, Render>(
        self,
        id: impl Into<ElementId>,
        setup: impl FnOnce(&mut ClipBuilder) -> P,
        render: Render,
    ) -> AnimotionElement
    where
        P: Send + Sync + 'static, // <-- Added Send + Sync here
        Render: Fn(Div, &P) -> Div + Send + Sync + 'static;
}

impl AnimotionClipExt for Div {
    fn animotion_clip<P, Render>(
        self,
        id: impl Into<ElementId>,
        setup: impl FnOnce(&mut ClipBuilder) -> P,
        render: Render,
    ) -> AnimotionElement
    where
        P: Send + Sync + 'static, // <-- Added Send + Sync here
        Render: Fn(Div, &P) -> Div + Send + Sync + 'static,
    {
        let mut builder = ClipBuilder::new();
        let props = setup(&mut builder);

        builder.register_render(props, render);

        let tracks = builder.build();
        self.animotion(id, tracks)
    }
}
