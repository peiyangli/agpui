use std::array;

use gpui::{Axis, Context, Entity, IntoElement, ParentElement as _, Pixels, Render, SharedString, Styled as _, Window, div, px};
use gpui_component::{ActiveTheme as _, StyledExt as _, v_flex};




pub struct History{
    text: SharedString,
    height: Pixels,
    hint: i32,
}
const ITEM_HEIGHT: Pixels = px(30.);
const ITEM_HEIGHTS: [gpui::Pixels; 7] = [px(20.), px(30.), px(50.), px(70.), px(110.),px(130.), px(170.)];
impl History {
    pub fn new(txt: impl Into<SharedString>)->Self{
        Self { text: txt.into(), height: ITEM_HEIGHT, hint: 0}
    }
    pub fn newWithI(i: i32)->Self{
        Self { text: format!("Item {}", i).into(), height: ITEM_HEIGHTS[i as usize%7], hint: i}
    }
}

pub struct HistoryView {
    historys: Vec<History>
}

impl HistoryView {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let hs = (0..5000).map(|i| History::newWithI(i)).collect::<Vec<_>>();
        Self {
            historys: hs,
        }
    }
}


impl Render for HistoryView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .relative()
            .border_1()
            .border_color(theme.border)
            .w_full()
            .child(
                v_flex()
                    .scrollable(Axis::Vertical)
                    .children(self.historys.iter().take(500).map(|item| {
                        div()
                            // .h(item.height)
                            .bg(theme.background)
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .child(item.text.to_string())
                    })),
            )
    }
}