use yixuan_proto::{EnterSceneScNotify, HollowPerformScNotify, Message, NetCmd, ServerPerformMsg};

use crate::listener::{NotifyListener, NotifyListenerExt};

#[derive(Default)]
pub struct HollowGlobalEventHelper {
    enter_notify: Option<HollowPerformScNotify>,
    perform_msg_queue: Vec<ServerPerformMsg>,
}

impl HollowGlobalEventHelper {
    pub fn set_enter_notify(&mut self, notify: EnterSceneScNotify) {
        self.enter_notify = Some(HollowPerformScNotify {
            msg_list: vec![Self::make_perform_msg(notify)],
        });
    }

    pub fn add<Notify: NetCmd + Message>(&mut self, notify: Notify) {
        self.perform_msg_queue.push(Self::make_perform_msg(notify));
    }

    pub fn flush(&mut self, notify_listener: &mut dyn NotifyListener) {
        if let Some(enter_notify) = self.enter_notify.take() {
            notify_listener.add(enter_notify);
        }

        if !self.perform_msg_queue.is_empty() {
            notify_listener.add(HollowPerformScNotify {
                msg_list: std::mem::take(&mut self.perform_msg_queue),
            });
        }
    }

    fn make_perform_msg<Notify: NetCmd + Message>(notify: Notify) -> ServerPerformMsg {
        ServerPerformMsg {
            cmd_id: notify.get_cmd_id() as u32,
            content: notify.encode_to_vec(),
        }
    }
}
