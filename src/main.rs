use std::{path::Path, rc::Rc, time::Duration};

use agpui::{AppTitleBar, HistoryView};
use fake::Fake;
use gpui::{
    AnyView, App, AppContext, Application, Bounds, Context, Edges, ElementId, Entity, FocusHandle, Focusable, Hsla, ImageSource, InteractiveElement, IntoElement, ParentElement, Pixels, Render, RenderOnce, ScrollStrategy, SharedString, Styled, Subscription, Task, Timer, Window, WindowBounds, WindowKind, WindowOptions, actions, div, prelude::FluentBuilder as _, px, size
};

use gpui_component::{
    ActiveTheme, Icon, IconName, IndexPath, Root, Selectable, Sizable, StyledExt, TitleBar, WindowExt, accordion::Accordion, alert::Alert, avatar::{Avatar, AvatarGroup}, badge::Badge, button::Button, checkbox::Checkbox, h_flex, label::Label, list::{List, ListDelegate, ListEvent, ListItem, ListState}, resizable::{h_resizable, resizable_panel}, v_flex, webview
};
use gpui_component_assets::Assets;
use gpui_component::webview::WebView;
use gpui_component::wry;

pub struct ChatContact {
    pub name: SharedString,
    pub description: SharedString
}

impl ChatContact {
    pub fn new(name: impl Into<SharedString>, description: impl Into<SharedString>) -> Self{
        Self { name: name.into(), description: description.into()}
    }
}

#[derive(Clone, Default)]
struct Contact {
    id: i64,
    name: SharedString,
    avatar: Option<ImageSource>,
    description: SharedString,
    last_done: f64,
    prev_close: f64,

    change_percent: f64,
    change_percent_str: SharedString,
    last_done_str: SharedString,
    prev_close_str: SharedString,
    // description: String,
}

fn random_contact() -> Contact {
    let last_done = (0.0..999.0).fake();
    let prev_close = last_done * (-0.1..0.1).fake::<f64>();

    let id = (0..9999999999).fake();

    Contact {
        id: id,
        avatar: Some(Path::new(&format!("G:/research/rustee/agpui/images/{}.png", id as usize%11)).into()),
        name: fake::faker::name::en::Name()
            .fake::<String>()
            .into(),
        description: fake::faker::company::en::Industry().fake::<String>().into(),
        last_done,
        prev_close,
        ..Default::default()
    }
}

#[derive(IntoElement)]
struct ContactListItem {
    base: ListItem,
    ix: IndexPath,
    contact: Rc<Contact>,
    selected: bool,
}

impl ContactListItem {
    pub fn new(
        id: impl Into<ElementId>,
        contact: Rc<Contact>,
        ix: IndexPath,
        selected: bool,
    ) -> Self {
        ContactListItem {
            contact,
            ix,
            base: ListItem::new(id),
            selected,
        }
    }
}

impl Selectable for ContactListItem {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl RenderOnce for ContactListItem {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let background: Hsla;
        if self.selected{
            background = theme.list_active;
        }else{
            background = theme.list_even
        }

        let mut img = Avatar::new().name(self.contact.name.clone());
        if let Some(avatar) = &self.contact.avatar {
            img = img.src(avatar.clone())
        }
        self.base
            .rounded(theme.radius)
            .bg(background)
            // .on_mouse_enter(|e, mut w, mut app|{})
            .paddings(Edges{ top: px(6.), right: px(6.), bottom: px(6.), left: px(6.) })
            .child(
                h_flex()
                .gap_1()
                .child(img)
                .child(
                    v_flex()
                    .gap_1()
                    .child(Label::new(self.contact.name.clone()))
                    .child(Label::new(self.contact.description.clone()))
                )
            )
    }
}

struct ContactsListDelegate {
    // industries: Vec<SharedString>,
    contacts: Vec<Rc<Contact>>,
    // matched_companies: Vec<Vec<Rc<Company>>>,
    selected_index: Option<IndexPath>,
    // confirmed_index: Option<IndexPath>,
    query: SharedString,
    loading: bool,
    eof: bool,
    lazy_load: bool,
}
actions!(contacts, [SelectedContact]);

impl ContactsListDelegate {
    fn prepare(&mut self, query: impl Into<SharedString>) {
        self.query = query.into();
    }

    fn extend_more(&mut self, len: usize) {
        self.contacts
            .extend((0..len).map(|_| Rc::new(random_contact())));
        self.prepare(self.query.clone());
    }
}

impl ListDelegate for ContactsListDelegate {
    type Item = ContactListItem;

    fn sections_count(&self, _: &App) -> usize {
        1
    }

    fn items_count(&self, section: usize, _: &App) -> usize {
        self.contacts.len()
    }

    fn perform_search(
        &mut self,
        query: &str,
        _: &mut Window,
        _: &mut Context<ListState<Self>>,
    ) -> Task<()> {
        self.prepare(query.to_owned());
        Task::ready(())
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<ListState<Self>>) {
        println!("Confirmed with secondary: {}", secondary);
        window.dispatch_action(Box::new(SelectedContact), cx);
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        win: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();


        win.open_dialog(cx, |dialog, _, _| {
                dialog
                .title("Welcome")
                .child("This is a dialog dialog.")
            })
    }

    // fn render_section_header(
    //     &self,
    //     section: usize,
    //     _: &mut Window,
    //     cx: &mut App,
    // ) -> Option<impl IntoElement> {
    //     Some(
    //         h_flex()
    //             .pb_1()
    //             .px_2()
    //             .gap_2()
    //             .text_sm()
    //             .text_color(cx.theme().muted_foreground)
    //             .child(Icon::new(IconName::Folder))
    //     )
    // }
    // fn render_section_footer(
    //     &self,
    //     section: usize,
    //     _: &mut Window,
    //     cx: &mut App,
    // ) -> Option<impl IntoElement> {
    //     Option<Element>::None
    // }

    fn render_item(&mut self, ix: IndexPath, _: &mut Window, _: &mut Context<ListState<Self>>,) -> Option<Self::Item> {
        let selected = Some(ix) == self.selected_index;
        if let Some(contact) = self.contacts.get(ix.row) {
            return Some(ContactListItem::new(ix, contact.clone(), ix, selected));
        }
        None
    }

    fn loading(&self, _: &App) -> bool {
        self.loading
    }

    fn is_eof(&self, _: &App) -> bool {
        return !self.loading && !self.eof;
    }

    fn load_more_threshold(&self) -> usize {
        150
    }

    fn load_more(&mut self, window: &mut Window, cx: &mut Context<ListState<Self>>) {
        if !self.lazy_load {
            return;
        }

        cx.spawn_in(window, async move |view, window| {
            // Simulate network request, delay 1s to load data.
            Timer::after(Duration::from_secs(1)).await;

            _ = view.update_in(window, move |view, window, cx| {
                let query = view.delegate().query.clone();
                view.delegate_mut().extend_more(200);
                _ = view.delegate_mut().perform_search(&query, window, cx);
                view.delegate_mut().eof = view.delegate().contacts.len() >= 6000;
            });
        })
        .detach();
    }
}


pub struct MainView{
    name: SharedString,
    collapsed: bool,
    // contacts: Vec<(&'static str, Vec<ChatContact>)>,

    history: Entity<HistoryView>,

    contacts: Entity<gpui_component::list::ListState<ContactsListDelegate>>,
}

impl MainView {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {


        let mut delegate = ContactsListDelegate {
            // industries: vec![],
            // matched_companies: vec![vec![]],
            contacts: vec![],
            // selected_index: Some(IndexPath::default()),
            // confirmed_index: None,
            // query: "".into(),
            loading: false,
            eof: false,
            lazy_load: false,
            selected_index: None,
            query: "".into(),
        };
        delegate.extend_more(20);
        // let contacts = vec![
        //     (
        //         "Hello world",
        //         vec![
        //             ChatContact::new("Hello", "Hello. The new Street"), 
        //             ChatContact::new("World", "World. Any where")
        //         ]
        //     ),
        //     (
        //         "Tom&Jerry",
        //         vec![
        //             ChatContact::new("Tom", "Tom Cat. LP Street"), 
        //             ChatContact::new("Jerry", "Jerry Mouse. LP Street Hole")
        //         ]
        //     )
        // ];


        let history = cx.new(|cx| HistoryView::new(window, cx));
        let contacts = cx.new(|cx| ListState::new(delegate, window, cx));
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
                    .child(
                        List::new(&self.contacts)
                            .p(px(8.))
                            .flex_1()
                            .w_full()
                            .border_1()
                            .border_color(cx.theme().border)
                            .rounded(cx.theme().radius),
                    )
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
    webview: Entity<WebView>,
}

impl MainWindow {
    pub fn new(
        title: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let title_bar = cx.new(|cx| AppTitleBar::new(title, window, cx));
        let view = cx.new(|cx| MainView::new(window, cx));


        let webview = cx.new(|cx| {
            let builder = wry::WebViewBuilder::new()
            .with_url("https://www.baidu.com");

            #[cfg(any(target_os = "windows", target_os = "macos", target_os = "ios", target_os = "android"))]
            let webview = {
                use raw_window_handle::HasWindowHandle;
                use wry::raw_window_handle;
                let window_handle = window.window_handle().expect("No window handle");
                builder.build_as_child(&window_handle).unwrap()
            };

            #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "ios", target_os = "android")))]
            let webview = {
                use gtk::prelude::*;
                use wry::WebViewBuilderExtUnix;
                let fixed = gtk::Fixed::builder().build();
                fixed.show_all();
                builder.build_gtk(&fixed).unwrap()
            };

            let view = WebView::new(webview, window, cx);
            view.set_bounds(wry::Rect {
                position: wry::dpi::LogicalPosition::new(0, 0).into(),
                size: wry::dpi::LogicalSize::new(400, 300).into(),
                }).unwrap();
            view
        });

        Self {
            title_bar,
            view: view.into(),
            webview: webview
        }
    }
}
impl Render for MainWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);
        
        div()
            .size_full()
            .child(
                v_flex()
                    .size_full()
                    .child(self.title_bar.clone())
                    .child(div().flex_1().overflow_hidden().child(self.view.clone()))
                    // .child(div().flex_1().overflow_hidden().child("Hello, World!"))
                    /* //webview
                    .child(Button::new("go").label("go to baidu").on_click(cx.listener(|this, _, _, cx| {
                        // let webview = this.webview.clone();
                        this.webview.update(cx, |webview, _| {
                            webview.show();
                            
                            let html_content = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Local Content</title>
    <style>
        body { font-family: Arial, sans-serif; padding: 20px; }
        .highlight { background-color: yellow; }
    </style>
</head>
<body>
    <h1>Hello from Local HTML</h1>
    <p class="highlight">This content is loaded locally!</p>
    <script>
        console.log('Local HTML loaded successfully');
    </script>
</body>
</html>
"#;

webview.load_html(html_content).unwrap();
                        });
                    })))
                    .child(
                        div()
                        .flex_1()
                        .size_full()
                        // .bg(cx.theme().red)
                        .child(self.webview.clone())
                    )
                     */
            )
            .children(dialog_layer)
            .children(sheet_layer)
            .children(notification_layer)
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