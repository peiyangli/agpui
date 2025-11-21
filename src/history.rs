use std::{array, path::Path};

use gpui::{Axis, Context, Edges, Entity, ImageSource, InteractiveElement as _, IntoElement, ParentElement as _, Pixels, Render, SharedString, Styled as _, Window, div, img, px};
use gpui_component::{ActiveTheme as _, StyledExt as _, h_flex, resizable::{resizable_panel, v_resizable}, v_flex};




pub struct History{
    text: SharedString,
    image: ImageSource,
    height: Pixels,
    hint: i32,
}
const ITEM_HEIGHT: Pixels = px(30.);
const ITEM_HEIGHTS: [gpui::Pixels; 7] = [px(20.), px(30.), px(50.), px(70.), px(110.),px(130.), px(170.)];
impl History {
    pub fn new(txt: impl Into<SharedString>, img: impl Into<ImageSource>)->Self{
        Self { text: txt.into(), image: img.into(), height: ITEM_HEIGHT, hint: 0}
    }
    pub fn newWithI(i: i32)->Self{
        Self { text: format!("Item {}", i).into(), image: Path::new(&format!("G:/research/rustee/agpui/src/assets/{}.png", i as usize%11)).into(), height: ITEM_HEIGHTS[i as usize%7], hint: i}
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

        v_flex()
            .flex_1()
            .h_full()
            .overflow_x_hidden()
            .child(
                h_flex()
                    .id("header")
                    .p_4()
                    .border_b_1()
                    .border_color(theme.border)
                    .bg(theme.blue)
                    .child("title"),
            )
            .child(
                v_resizable("history")
                    .child(
                        div()
                        .size_full()
                        .flex()
                        .child(
                            div()
                                .w_full()
                                .bg(theme.background)
                                .child(
                                    v_flex()
                                        .paddings(Edges{ top: px(10.), right: px(20.), bottom: px(30.), left: px(40.) })
                                        .border_10()
                                        .border_color(theme.yellow)
                                        .scrollable(Axis::Vertical)
                                        .children(self.historys.iter().take(100).map(|item| {
                                            if item.hint % 2 == 0{
                                                div()
                                                .margins(Edges::all(px(10.)))
                                                .flex()
                                                .justify_end()
                                                .text_sm()
                                                .child(item.text.clone())
                                                .child(img(item.image.clone()).max_w(px(600.)))
                                            }else{
                                                div()
                                                .margins(Edges::all(px(10.)))
                                                .text_sm()
                                                .child(item.text.clone())
                                                .child(img(item.image.clone()).max_w(px(600.)))
                                            }
                                        })),
                                )
                        )
                        .into_any_element()
                    )
                    .child(
                        resizable_panel()
                        .size(px(255.))
                        .size_range(px(100.)..px(500.))
                        .child("Input(TODO)")
                    )
            )
    }
}