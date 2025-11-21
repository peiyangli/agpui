use agpui::{AppTitleBar, HistoryView};
use gpui::*;
use gpui_component::{button::*, history::History, plot::label::Text, resizable::{ResizablePanel, h_resizable, resizable_panel, v_resizable}, sidebar::{Sidebar, SidebarGroup, SidebarMenu, SidebarMenuItem}, *};
use gpui_component_assets::Assets;


pub struct ChatContact {
    pub name: SharedString,
    pub description: SharedString
}

impl ChatContact {
    pub fn new(name: impl Into<SharedString>, description: impl Into<SharedString>) -> Self{
        Self { name: name.into(), description: description.into()}
    }
}

pub struct MainView{
    name: SharedString,
    collapsed: bool,
    contacts: Vec<(&'static str, Vec<ChatContact>)>,

    history: Entity<HistoryView>,
}

impl MainView {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {


        let contacts = vec![
            (
                "Hello world",
                vec![
                    ChatContact::new("Hello", "Hello. The new Street"), 
                    ChatContact::new("World", "World. Any where")
                ]
            ),
            (
                "Tom&Jerry",
                vec![
                    ChatContact::new("Tom", "Tom Cat. LP Street"), 
                    ChatContact::new("Jerry", "Jerry Mouse. LP Street Hole")
                ]
            )
        ];


        let history = cx.new(|cx| HistoryView::new(window, cx));

        Self { 
            name: SharedString::default(),
            collapsed: false,
            contacts: contacts,
            history: history,
        }
    }
}

impl Render for MainView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        
        h_resizable("gallery-container")
            .child(
                resizable_panel()
                    .size(px(255.))
                    .size_range(px(160.)..px(360.))
                    .child("GO")
                )
            .child(
                self.history.clone().into_any_element(),
            )
        // div().bg(theme.blue)
    }
}

pub struct MainWindow{
    title_bar: Entity<AppTitleBar>,
    view: AnyView,
}

impl MainWindow {
    pub fn new(
        title: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let title_bar = cx.new(|cx| AppTitleBar::new(title, window, cx));
        let view = cx.new(|cx| MainView::new(window, cx));
        Self {
            title_bar,
            view: view.into(),
        }
    }
}
impl Render for MainWindow {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(
                v_flex()
                    .size_full()
                    .child(self.title_bar.clone())
                    .child(div().flex_1().overflow_hidden().child(self.view.clone())),
                    // .child(div().flex_1().overflow_hidden().child("Hello, World!"))
            )
    }
}

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        let window_size: Option<gpui::Size<Pixels>> = None;
            let mut window_size = window_size.unwrap_or(size(px(1600.0), px(1200.0)));
            if let Some(display) = cx.primary_display() {
                let display_size = display.bounds().size;
                window_size.width = window_size.width.min(display_size.width * 0.85);
                window_size.height = window_size.height.min(display_size.height * 0.85);
            }
        let window_bounds = Bounds::centered(None, window_size, cx);

        cx.spawn(async move |cx| {
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(window_bounds)),
                titlebar: Some(TitleBar::title_bar_options()),
                window_min_size: Some(gpui::Size {
                    width: px(480.),
                    height: px(320.),
                }),
                kind: WindowKind::Normal,
                #[cfg(target_os = "linux")]
                window_background: gpui::WindowBackgroundAppearance::Transparent,
                #[cfg(target_os = "linux")]
                window_decorations: Some(gpui::WindowDecorations::Client),
                ..Default::default()
            };
            let title = SharedString::from("APP".to_string());
            let window = cx.open_window(options, |window, cx| {

                let view = cx.new(|cx| MainWindow::new(title.clone(), window, cx));
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            }).expect("open_window");


            

            window
            .update(cx, |_, window, _| {
                window.activate_window();
                window.set_window_title(&title);
            })
            .expect("failed to update window");

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}