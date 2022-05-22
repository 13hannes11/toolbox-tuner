use std::process::Command;

use relm4::factory::DynamicIndex;
use relm4::{send, MessageHandler, Sender};
use tokio::runtime::{Builder, Runtime};
use tokio::sync::mpsc::{channel, Sender as TokioSender};

use super::{
    messages::AppMsg,
    model::{AppModel, ToolbxEntry},
};

// Code adapted from https://relm4.org/book/stable/message_handler.html
pub struct AsyncHandler {
    _rt: Runtime,
    sender: TokioSender<AsyncHandlerMsg>,
}

#[derive(Debug)]
pub enum AsyncHandlerMsg {
    StopToolbx(DynamicIndex, ToolbxEntry),
    StartToolbx(DynamicIndex, ToolbxEntry),
    OpenToolbxTerminal(DynamicIndex, ToolbxEntry),
}

impl MessageHandler<AppModel> for AsyncHandler {
    type Msg = AsyncHandlerMsg;
    type Sender = TokioSender<AsyncHandlerMsg>;

    fn init(_parent_model: &AppModel, parent_sender: Sender<AppMsg>) -> Self {
        let (sender, mut rx) = channel::<AsyncHandlerMsg>(10);

        let rt = Builder::new_multi_thread()
            .worker_threads(8)
            .enable_time()
            .build()
            .unwrap();

        rt.spawn(async move {
            while let Some(msg) = rx.recv().await {
                let parent_sender = parent_sender.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    match msg {
                        AsyncHandlerMsg::StopToolbx(index, mut tbx) => {
                            tbx.toolbx_container.stop();
                            tbx.changing_status = false;
                            send! {parent_sender, AppMsg::ToolbxContainerChanged(index, tbx)};
                        }
                        AsyncHandlerMsg::StartToolbx(index, mut tbx) => {
                            tbx.toolbx_container.start();
                            tbx.changing_status = false;
                            send! {parent_sender, AppMsg::ToolbxContainerChanged(index, tbx)};
                        }
                        AsyncHandlerMsg::OpenToolbxTerminal(index, mut tbx) => {
                            // TODO: support many terminals and check which are installed
                            let output = Command::new("gnome-terminal")
                                .arg("--")
                                .arg("toolbox")
                                .arg("enter")
                                .arg(tbx.toolbx_container.name.clone())
                                .output();

                            println!("{:?}", output);

                            // TODO: update status
                        }
                    }
                });
            }
        });

        AsyncHandler { _rt: rt, sender }
    }

    fn send(&self, msg: Self::Msg) {
        self.sender.blocking_send(msg).unwrap();
    }

    fn sender(&self) -> Self::Sender {
        self.sender.clone()
    }
}
