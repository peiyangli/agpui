mod title_bar;
mod history;
mod contacts;
// 显式引用 assets 包，确保图标资源被嵌入到二进制文件中
use gpui_component_assets as _;

pub use title_bar::AppTitleBar;
pub use history::HistoryView;
// pub use contacts::ContactsListDelegate;