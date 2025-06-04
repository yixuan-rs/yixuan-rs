use yixuan_proto::{Message, NetCmd};

pub trait NotifyListener {
    fn push(&mut self, cmd_id: u16, body: Vec<u8>);
}

pub trait NotifyListenerExt {
    fn add<Notify: Message + NetCmd>(&mut self, notify: Notify);
}

pub trait LogicEventListener {
    fn main_city_quest_finished(&mut self, quest_id: u32) -> Vec<u32>;
    fn hollow_quest_finished(&mut self, quest_id: u32);
    fn change_back_scene_info(&mut self, section_id: u32, transform: String);
    fn unlock_hollow_quest(&mut self, quest_id: u32);
    fn give_once_reward(&mut self, once_reward_id: u32);
    fn has_gained_once_reward(&self, once_reward_id: u32) -> bool;
    fn hollow_event_executed(&mut self, hollow_event_id: u32);
    fn add_item(&mut self, id: u32, count: u32);
}

impl<T: ?Sized + NotifyListener> NotifyListenerExt for T {
    fn add<Notify: Message + NetCmd>(&mut self, notify: Notify) {
        self.push(notify.get_cmd_id(), notify.encode_to_vec());
    }
}
