use relm4::RelmComponent;
use relm4::RelmMsgHandler;
use relm4::Sender;

use super::app::model::AppModel;
use super::app::workers::AsyncHandler;

#[derive(relm4::Components)]
pub struct AppComponents {
    pub async_handler: RelmMsgHandler<AsyncHandler, AppModel>,
}
