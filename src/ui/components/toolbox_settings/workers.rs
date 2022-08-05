use std::time::Duration;

use crate::toolbx::ToolbxContainer;

use relm4::factory::DynamicIndex;
use relm4::{send, MessageHandler, Sender};
use tokio::runtime::{Builder, Runtime};
use tokio::sync::mpsc::{channel, Sender as TokioSender};

use crate::ui::app::{messages::AppMsg, model::AppModel};

use super::model::ToolboxPreferences;
use crate::ui::components::toolbox_settings::messages::ToolboxSettingsDialogMsg;
use crate::ui::components::ToolboxSettingsDialogModel;

// Code adapted from https://relm4.org/book/stable/message_handler.html
pub struct AsyncSettingsHandler {
    _rt: Runtime,
    sender: TokioSender<AsyncSettingsHandlerMsg>,
}

#[derive(Debug)]
pub enum AsyncSettingsHandlerMsg {
    RequestToolbxSettings(ToolbxContainer),
    SaveToolboxSettings(ToolboxPreferences),
}

impl MessageHandler<ToolboxSettingsDialogModel> for AsyncSettingsHandler {
    type Msg = AsyncSettingsHandlerMsg;
    type Sender = TokioSender<AsyncSettingsHandlerMsg>;

    fn init(
        _parent_model: &ToolboxSettingsDialogModel,
        parent_sender: Sender<ToolboxSettingsDialogMsg>,
    ) -> Self {
        let (sender, mut rx) = channel::<AsyncSettingsHandlerMsg>(10);

        let rt = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_time()
            .build()
            .unwrap();

        rt.spawn(async move {
            while let Some(msg) = rx.recv().await {
                let parent_sender = parent_sender.clone();
                tokio::spawn(async move {
                    match msg {
                        AsyncSettingsHandlerMsg::SaveToolboxSettings(tbx_settings) => {
                            // TODO: actually save settings
                            println!("Received Toolbx Settings save request");
                        }
                        AsyncSettingsHandlerMsg::RequestToolbxSettings(tbx) => {
                            println!("Received ToolbxSettings Request");
                            parent_sender
                                .send(ToolboxSettingsDialogMsg::ReplyToolbxSettings(
                                    ToolboxPreferences {},
                                ))
                                .unwrap();
                        }
                    }
                });
            }
        });

        AsyncSettingsHandler { _rt: rt, sender }
    }

    fn send(&self, msg: Self::Msg) {
        self.sender.blocking_send(msg).unwrap();
    }

    fn sender(&self) -> Self::Sender {
        self.sender.clone()
    }
}
