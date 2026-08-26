use crate::PropertyTrack;
use gpui::*;
use std::time::Instant;

pub struct AnimotionElement {
    id: ElementId,
    child: Option<Div>,
    tracks: Vec<Box<dyn PropertyTrack>>,
}

impl ParentElement for AnimotionElement {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        if let Some(child) = self.child.as_mut() {
            child.extend(elements);
        }
    }
}

impl IntoElement for AnimotionElement {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

pub trait AnimotionExt {
    fn animotion(self, id: impl Into<ElementId>, tracks: Vec<Box<dyn PropertyTrack>>) -> AnimotionElement;
}

impl AnimotionExt for Div {
    fn animotion(self, id: impl Into<ElementId>, tracks: Vec<Box<dyn PropertyTrack>>) -> AnimotionElement {
        AnimotionElement {
            id: id.into(),
            child: Some(self),
            tracks,
        }
    }
}

impl Element for AnimotionElement {
    type RequestLayoutState = AnyElement;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let global_id = id.unwrap();

        let (layout_id, element) = window.with_element_state(global_id, |state: Option<Instant>, window_cx| {
            let start = state.unwrap_or_else(Instant::now);
            let elapsed = start.elapsed();

            let mut current_child = self.child.take().unwrap_or_else(div);
            for track in &self.tracks {
                current_child = track.apply(current_child, elapsed);
            }

            window_cx.request_animation_frame();

            let mut any_el = current_child.into_any_element();
            let layout_id = any_el.request_layout(window_cx, cx);

            ((layout_id, any_el), start)
        });

        (layout_id, element)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        element: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) {
        element.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        element: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        element.paint(window, cx);
    }
}
