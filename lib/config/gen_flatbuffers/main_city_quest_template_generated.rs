// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum TimeOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Time<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Time<'a> {
  type Inner = Time<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Time<'a> {
  pub const VT_HOUR: flatbuffers::VOffsetT = 4;
  pub const VT_MINUTE: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Time { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args TimeArgs
  ) -> flatbuffers::WIPOffset<Time<'bldr>> {
    let mut builder = TimeBuilder::new(_fbb);
    builder.add_minute(args.minute);
    builder.add_hour(args.hour);
    builder.finish()
  }


  #[inline]
  pub fn hour(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(Time::VT_HOUR, Some(0)).unwrap()}
  }
  #[inline]
  pub fn minute(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(Time::VT_MINUTE, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for Time<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i32>("hour", Self::VT_HOUR, false)?
     .visit_field::<i32>("minute", Self::VT_MINUTE, false)?
     .finish();
    Ok(())
  }
}
pub struct TimeArgs {
    pub hour: i32,
    pub minute: i32,
}
impl<'a> Default for TimeArgs {
  #[inline]
  fn default() -> Self {
    TimeArgs {
      hour: 0,
      minute: 0,
    }
  }
}

pub struct TimeBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> TimeBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_hour(&mut self, hour: i32) {
    self.fbb_.push_slot::<i32>(Time::VT_HOUR, hour, 0);
  }
  #[inline]
  pub fn add_minute(&mut self, minute: i32) {
    self.fbb_.push_slot::<i32>(Time::VT_MINUTE, minute, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> TimeBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    TimeBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Time<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Time<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Time");
      ds.field("hour", &self.hour());
      ds.field("minute", &self.minute());
      ds.finish()
  }
}
pub enum MainCityQuestTemplateOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct MainCityQuestTemplate<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for MainCityQuestTemplate<'a> {
  type Inner = MainCityQuestTemplate<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> MainCityQuestTemplate<'a> {
  pub const VT_ID: flatbuffers::VOffsetT = 4;
  pub const VT_QUEST_NAME: flatbuffers::VOffsetT = 6;
  pub const VT_TYPE_: flatbuffers::VOffsetT = 8;
  pub const VT_SHOW_FINISH_TIP: flatbuffers::VOffsetT = 10;
  pub const VT_UNK_2: flatbuffers::VOffsetT = 12;
  pub const VT_UNK_3: flatbuffers::VOffsetT = 14;
  pub const VT_QUEST_DESC: flatbuffers::VOffsetT = 16;
  pub const VT_UNK_5: flatbuffers::VOffsetT = 18;
  pub const VT_START_TIME: flatbuffers::VOffsetT = 20;
  pub const VT_UNK_6: flatbuffers::VOffsetT = 22;
  pub const VT_UNK_7: flatbuffers::VOffsetT = 24;
  pub const VT_GROUP: flatbuffers::VOffsetT = 26;
  pub const VT_PRIORITY: flatbuffers::VOffsetT = 28;
  pub const VT_AUTO_DISTRIBUTION: flatbuffers::VOffsetT = 30;
  pub const VT_SCRIPT_EXEC_TYPE: flatbuffers::VOffsetT = 32;
  pub const VT_UNK_8: flatbuffers::VOffsetT = 34;
  pub const VT_QUEST_BEGIN_SCRIPT: flatbuffers::VOffsetT = 36;
  pub const VT_QUEST_END_SCRIPT: flatbuffers::VOffsetT = 38;
  pub const VT_QUEST_END_ACTION_TYPE: flatbuffers::VOffsetT = 40;
  pub const VT_ACTION_ARG_1: flatbuffers::VOffsetT = 42;
  pub const VT_ACTION_ARG_2: flatbuffers::VOffsetT = 44;
  pub const VT_ACTION_ARG_3: flatbuffers::VOffsetT = 46;
  pub const VT_ACTION_ARG_4: flatbuffers::VOffsetT = 48;
  pub const VT_UNK_9: flatbuffers::VOffsetT = 50;
  pub const VT_UNK_10: flatbuffers::VOffsetT = 52;
  pub const VT_UNK_11: flatbuffers::VOffsetT = 54;
  pub const VT_UNK_12: flatbuffers::VOffsetT = 56;
  pub const VT_UNK_13: flatbuffers::VOffsetT = 58;
  pub const VT_UNK_14: flatbuffers::VOffsetT = 60;
  pub const VT_UNK_15: flatbuffers::VOffsetT = 62;
  pub const VT_UNK_16: flatbuffers::VOffsetT = 64;
  pub const VT_UNK_17: flatbuffers::VOffsetT = 66;
  pub const VT_UNK_18: flatbuffers::VOffsetT = 68;
  pub const VT_UNK_19: flatbuffers::VOffsetT = 70;
  pub const VT_UNK_20: flatbuffers::VOffsetT = 72;
  pub const VT_UNK_21: flatbuffers::VOffsetT = 74;
  pub const VT_UNK_22: flatbuffers::VOffsetT = 76;
  pub const VT_UNK_23: flatbuffers::VOffsetT = 78;
  pub const VT_NPCS: flatbuffers::VOffsetT = 80;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    MainCityQuestTemplate { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args MainCityQuestTemplateArgs<'args>
  ) -> flatbuffers::WIPOffset<MainCityQuestTemplate<'bldr>> {
    let mut builder = MainCityQuestTemplateBuilder::new(_fbb);
    if let Some(x) = args.npcs { builder.add_npcs(x); }
    if let Some(x) = args.unk_23 { builder.add_unk_23(x); }
    if let Some(x) = args.unk_22 { builder.add_unk_22(x); }
    builder.add_unk_21(args.unk_21);
    builder.add_unk_20(args.unk_20);
    builder.add_unk_19(args.unk_19);
    builder.add_unk_18(args.unk_18);
    builder.add_unk_17(args.unk_17);
    builder.add_unk_15(args.unk_15);
    builder.add_unk_13(args.unk_13);
    builder.add_unk_12(args.unk_12);
    if let Some(x) = args.action_arg_4 { builder.add_action_arg_4(x); }
    if let Some(x) = args.action_arg_3 { builder.add_action_arg_3(x); }
    if let Some(x) = args.action_arg_2 { builder.add_action_arg_2(x); }
    if let Some(x) = args.action_arg_1 { builder.add_action_arg_1(x); }
    builder.add_quest_end_action_type(args.quest_end_action_type);
    builder.add_quest_end_script(args.quest_end_script);
    builder.add_quest_begin_script(args.quest_begin_script);
    if let Some(x) = args.unk_8 { builder.add_unk_8(x); }
    builder.add_script_exec_type(args.script_exec_type);
    builder.add_priority(args.priority);
    builder.add_group(args.group);
    builder.add_unk_7(args.unk_7);
    builder.add_unk_6(args.unk_6);
    if let Some(x) = args.start_time { builder.add_start_time(x); }
    if let Some(x) = args.unk_5 { builder.add_unk_5(x); }
    if let Some(x) = args.quest_desc { builder.add_quest_desc(x); }
    if let Some(x) = args.unk_2 { builder.add_unk_2(x); }
    builder.add_show_finish_tip(args.show_finish_tip);
    builder.add_type_(args.type_);
    if let Some(x) = args.quest_name { builder.add_quest_name(x); }
    builder.add_id(args.id);
    builder.add_unk_16(args.unk_16);
    builder.add_unk_14(args.unk_14);
    builder.add_unk_11(args.unk_11);
    builder.add_unk_10(args.unk_10);
    builder.add_unk_9(args.unk_9);
    builder.add_auto_distribution(args.auto_distribution);
    builder.add_unk_3(args.unk_3);
    builder.finish()
  }


  #[inline]
  pub fn id(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(MainCityQuestTemplate::VT_ID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn quest_name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(MainCityQuestTemplate::VT_QUEST_NAME, None)}
  }
  #[inline]
  pub fn type_(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(MainCityQuestTemplate::VT_TYPE_, Some(0)).unwrap()}
  }
  #[inline]
  pub fn show_finish_tip(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(MainCityQuestTemplate::VT_SHOW_FINISH_TIP, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_2(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(MainCityQuestTemplate::VT_UNK_2, None)}
  }
  #[inline]
  pub fn unk_3(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(MainCityQuestTemplate::VT_UNK_3, Some(false)).unwrap()}
  }
  #[inline]
  pub fn quest_desc(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(MainCityQuestTemplate::VT_QUEST_DESC, None)}
  }
  #[inline]
  pub fn unk_5(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(MainCityQuestTemplate::VT_UNK_5, None)}
  }
  #[inline]
  pub fn start_time(&self) -> Option<Time<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<Time>>(MainCityQuestTemplate::VT_START_TIME, None)}
  }
  #[inline]
  pub fn unk_6(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(MainCityQuestTemplate::VT_UNK_6, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_7(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(MainCityQuestTemplate::VT_UNK_7, Some(0)).unwrap()}
  }
  #[inline]
  pub fn group(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(MainCityQuestTemplate::VT_GROUP, Some(0)).unwrap()}
  }
  #[inline]
  pub fn priority(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(MainCityQuestTemplate::VT_PRIORITY, Some(0)).unwrap()}
  }
  #[inline]
  pub fn auto_distribution(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(MainCityQuestTemplate::VT_AUTO_DISTRIBUTION, Some(false)).unwrap()}
  }
  #[inline]
  pub fn script_exec_type(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(MainCityQuestTemplate::VT_SCRIPT_EXEC_TYPE, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_8(&self) -> Option<flatbuffers::Vector<'a, i32>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, i32>>>(MainCityQuestTemplate::VT_UNK_8, None)}
  }
  #[inline]
  pub fn quest_begin_script(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(MainCityQuestTemplate::VT_QUEST_BEGIN_SCRIPT, Some(0)).unwrap()}
  }
  #[inline]
  pub fn quest_end_script(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(MainCityQuestTemplate::VT_QUEST_END_SCRIPT, Some(0)).unwrap()}
  }
  #[inline]
  pub fn quest_end_action_type(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(MainCityQuestTemplate::VT_QUEST_END_ACTION_TYPE, Some(0)).unwrap()}
  }
  #[inline]
  pub fn action_arg_1(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(MainCityQuestTemplate::VT_ACTION_ARG_1, None)}
  }
  #[inline]
  pub fn action_arg_2(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(MainCityQuestTemplate::VT_ACTION_ARG_2, None)}
  }
  #[inline]
  pub fn action_arg_3(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(MainCityQuestTemplate::VT_ACTION_ARG_3, None)}
  }
  #[inline]
  pub fn action_arg_4(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(MainCityQuestTemplate::VT_ACTION_ARG_4, None)}
  }
  #[inline]
  pub fn unk_9(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(MainCityQuestTemplate::VT_UNK_9, Some(false)).unwrap()}
  }
  #[inline]
  pub fn unk_10(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(MainCityQuestTemplate::VT_UNK_10, Some(false)).unwrap()}
  }
  #[inline]
  pub fn unk_11(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(MainCityQuestTemplate::VT_UNK_11, Some(false)).unwrap()}
  }
  #[inline]
  pub fn unk_12(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(MainCityQuestTemplate::VT_UNK_12, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_13(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(MainCityQuestTemplate::VT_UNK_13, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_14(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(MainCityQuestTemplate::VT_UNK_14, Some(false)).unwrap()}
  }
  #[inline]
  pub fn unk_15(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(MainCityQuestTemplate::VT_UNK_15, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_16(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(MainCityQuestTemplate::VT_UNK_16, Some(false)).unwrap()}
  }
  #[inline]
  pub fn unk_17(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(MainCityQuestTemplate::VT_UNK_17, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_18(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(MainCityQuestTemplate::VT_UNK_18, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_19(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(MainCityQuestTemplate::VT_UNK_19, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_20(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(MainCityQuestTemplate::VT_UNK_20, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_21(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(MainCityQuestTemplate::VT_UNK_21, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_22(&self) -> Option<flatbuffers::Vector<'a, u32>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(MainCityQuestTemplate::VT_UNK_22, None)}
  }
  #[inline]
  pub fn unk_23(&self) -> Option<flatbuffers::Vector<'a, u32>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(MainCityQuestTemplate::VT_UNK_23, None)}
  }
  #[inline]
  pub fn npcs(&self) -> Option<flatbuffers::Vector<'a, u32>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(MainCityQuestTemplate::VT_NPCS, None)}
  }
}

impl flatbuffers::Verifiable for MainCityQuestTemplate<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u32>("id", Self::VT_ID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("quest_name", Self::VT_QUEST_NAME, false)?
     .visit_field::<i32>("type_", Self::VT_TYPE_, false)?
     .visit_field::<i32>("show_finish_tip", Self::VT_SHOW_FINISH_TIP, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("unk_2", Self::VT_UNK_2, false)?
     .visit_field::<bool>("unk_3", Self::VT_UNK_3, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("quest_desc", Self::VT_QUEST_DESC, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("unk_5", Self::VT_UNK_5, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<Time>>("start_time", Self::VT_START_TIME, false)?
     .visit_field::<i32>("unk_6", Self::VT_UNK_6, false)?
     .visit_field::<i32>("unk_7", Self::VT_UNK_7, false)?
     .visit_field::<i32>("group", Self::VT_GROUP, false)?
     .visit_field::<i32>("priority", Self::VT_PRIORITY, false)?
     .visit_field::<bool>("auto_distribution", Self::VT_AUTO_DISTRIBUTION, false)?
     .visit_field::<i32>("script_exec_type", Self::VT_SCRIPT_EXEC_TYPE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, i32>>>("unk_8", Self::VT_UNK_8, false)?
     .visit_field::<i32>("quest_begin_script", Self::VT_QUEST_BEGIN_SCRIPT, false)?
     .visit_field::<i32>("quest_end_script", Self::VT_QUEST_END_SCRIPT, false)?
     .visit_field::<i32>("quest_end_action_type", Self::VT_QUEST_END_ACTION_TYPE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("action_arg_1", Self::VT_ACTION_ARG_1, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("action_arg_2", Self::VT_ACTION_ARG_2, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("action_arg_3", Self::VT_ACTION_ARG_3, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("action_arg_4", Self::VT_ACTION_ARG_4, false)?
     .visit_field::<bool>("unk_9", Self::VT_UNK_9, false)?
     .visit_field::<bool>("unk_10", Self::VT_UNK_10, false)?
     .visit_field::<bool>("unk_11", Self::VT_UNK_11, false)?
     .visit_field::<u32>("unk_12", Self::VT_UNK_12, false)?
     .visit_field::<u32>("unk_13", Self::VT_UNK_13, false)?
     .visit_field::<bool>("unk_14", Self::VT_UNK_14, false)?
     .visit_field::<u32>("unk_15", Self::VT_UNK_15, false)?
     .visit_field::<bool>("unk_16", Self::VT_UNK_16, false)?
     .visit_field::<u32>("unk_17", Self::VT_UNK_17, false)?
     .visit_field::<u32>("unk_18", Self::VT_UNK_18, false)?
     .visit_field::<u32>("unk_19", Self::VT_UNK_19, false)?
     .visit_field::<u32>("unk_20", Self::VT_UNK_20, false)?
     .visit_field::<u32>("unk_21", Self::VT_UNK_21, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u32>>>("unk_22", Self::VT_UNK_22, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u32>>>("unk_23", Self::VT_UNK_23, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u32>>>("npcs", Self::VT_NPCS, false)?
     .finish();
    Ok(())
  }
}
pub struct MainCityQuestTemplateArgs<'a> {
    pub id: u32,
    pub quest_name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub type_: i32,
    pub show_finish_tip: i32,
    pub unk_2: Option<flatbuffers::WIPOffset<&'a str>>,
    pub unk_3: bool,
    pub quest_desc: Option<flatbuffers::WIPOffset<&'a str>>,
    pub unk_5: Option<flatbuffers::WIPOffset<&'a str>>,
    pub start_time: Option<flatbuffers::WIPOffset<Time<'a>>>,
    pub unk_6: i32,
    pub unk_7: i32,
    pub group: i32,
    pub priority: i32,
    pub auto_distribution: bool,
    pub script_exec_type: i32,
    pub unk_8: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, i32>>>,
    pub quest_begin_script: i32,
    pub quest_end_script: i32,
    pub quest_end_action_type: i32,
    pub action_arg_1: Option<flatbuffers::WIPOffset<&'a str>>,
    pub action_arg_2: Option<flatbuffers::WIPOffset<&'a str>>,
    pub action_arg_3: Option<flatbuffers::WIPOffset<&'a str>>,
    pub action_arg_4: Option<flatbuffers::WIPOffset<&'a str>>,
    pub unk_9: bool,
    pub unk_10: bool,
    pub unk_11: bool,
    pub unk_12: u32,
    pub unk_13: u32,
    pub unk_14: bool,
    pub unk_15: u32,
    pub unk_16: bool,
    pub unk_17: u32,
    pub unk_18: u32,
    pub unk_19: u32,
    pub unk_20: u32,
    pub unk_21: u32,
    pub unk_22: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
    pub unk_23: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
    pub npcs: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
}
impl<'a> Default for MainCityQuestTemplateArgs<'a> {
  #[inline]
  fn default() -> Self {
    MainCityQuestTemplateArgs {
      id: 0,
      quest_name: None,
      type_: 0,
      show_finish_tip: 0,
      unk_2: None,
      unk_3: false,
      quest_desc: None,
      unk_5: None,
      start_time: None,
      unk_6: 0,
      unk_7: 0,
      group: 0,
      priority: 0,
      auto_distribution: false,
      script_exec_type: 0,
      unk_8: None,
      quest_begin_script: 0,
      quest_end_script: 0,
      quest_end_action_type: 0,
      action_arg_1: None,
      action_arg_2: None,
      action_arg_3: None,
      action_arg_4: None,
      unk_9: false,
      unk_10: false,
      unk_11: false,
      unk_12: 0,
      unk_13: 0,
      unk_14: false,
      unk_15: 0,
      unk_16: false,
      unk_17: 0,
      unk_18: 0,
      unk_19: 0,
      unk_20: 0,
      unk_21: 0,
      unk_22: None,
      unk_23: None,
      npcs: None,
    }
  }
}

pub struct MainCityQuestTemplateBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> MainCityQuestTemplateBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_id(&mut self, id: u32) {
    self.fbb_.push_slot::<u32>(MainCityQuestTemplate::VT_ID, id, 0);
  }
  #[inline]
  pub fn add_quest_name(&mut self, quest_name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MainCityQuestTemplate::VT_QUEST_NAME, quest_name);
  }
  #[inline]
  pub fn add_type_(&mut self, type_: i32) {
    self.fbb_.push_slot::<i32>(MainCityQuestTemplate::VT_TYPE_, type_, 0);
  }
  #[inline]
  pub fn add_show_finish_tip(&mut self, show_finish_tip: i32) {
    self.fbb_.push_slot::<i32>(MainCityQuestTemplate::VT_SHOW_FINISH_TIP, show_finish_tip, 0);
  }
  #[inline]
  pub fn add_unk_2(&mut self, unk_2: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MainCityQuestTemplate::VT_UNK_2, unk_2);
  }
  #[inline]
  pub fn add_unk_3(&mut self, unk_3: bool) {
    self.fbb_.push_slot::<bool>(MainCityQuestTemplate::VT_UNK_3, unk_3, false);
  }
  #[inline]
  pub fn add_quest_desc(&mut self, quest_desc: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MainCityQuestTemplate::VT_QUEST_DESC, quest_desc);
  }
  #[inline]
  pub fn add_unk_5(&mut self, unk_5: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MainCityQuestTemplate::VT_UNK_5, unk_5);
  }
  #[inline]
  pub fn add_start_time(&mut self, start_time: flatbuffers::WIPOffset<Time<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Time>>(MainCityQuestTemplate::VT_START_TIME, start_time);
  }
  #[inline]
  pub fn add_unk_6(&mut self, unk_6: i32) {
    self.fbb_.push_slot::<i32>(MainCityQuestTemplate::VT_UNK_6, unk_6, 0);
  }
  #[inline]
  pub fn add_unk_7(&mut self, unk_7: i32) {
    self.fbb_.push_slot::<i32>(MainCityQuestTemplate::VT_UNK_7, unk_7, 0);
  }
  #[inline]
  pub fn add_group(&mut self, group: i32) {
    self.fbb_.push_slot::<i32>(MainCityQuestTemplate::VT_GROUP, group, 0);
  }
  #[inline]
  pub fn add_priority(&mut self, priority: i32) {
    self.fbb_.push_slot::<i32>(MainCityQuestTemplate::VT_PRIORITY, priority, 0);
  }
  #[inline]
  pub fn add_auto_distribution(&mut self, auto_distribution: bool) {
    self.fbb_.push_slot::<bool>(MainCityQuestTemplate::VT_AUTO_DISTRIBUTION, auto_distribution, false);
  }
  #[inline]
  pub fn add_script_exec_type(&mut self, script_exec_type: i32) {
    self.fbb_.push_slot::<i32>(MainCityQuestTemplate::VT_SCRIPT_EXEC_TYPE, script_exec_type, 0);
  }
  #[inline]
  pub fn add_unk_8(&mut self, unk_8: flatbuffers::WIPOffset<flatbuffers::Vector<'b , i32>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MainCityQuestTemplate::VT_UNK_8, unk_8);
  }
  #[inline]
  pub fn add_quest_begin_script(&mut self, quest_begin_script: i32) {
    self.fbb_.push_slot::<i32>(MainCityQuestTemplate::VT_QUEST_BEGIN_SCRIPT, quest_begin_script, 0);
  }
  #[inline]
  pub fn add_quest_end_script(&mut self, quest_end_script: i32) {
    self.fbb_.push_slot::<i32>(MainCityQuestTemplate::VT_QUEST_END_SCRIPT, quest_end_script, 0);
  }
  #[inline]
  pub fn add_quest_end_action_type(&mut self, quest_end_action_type: i32) {
    self.fbb_.push_slot::<i32>(MainCityQuestTemplate::VT_QUEST_END_ACTION_TYPE, quest_end_action_type, 0);
  }
  #[inline]
  pub fn add_action_arg_1(&mut self, action_arg_1: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MainCityQuestTemplate::VT_ACTION_ARG_1, action_arg_1);
  }
  #[inline]
  pub fn add_action_arg_2(&mut self, action_arg_2: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MainCityQuestTemplate::VT_ACTION_ARG_2, action_arg_2);
  }
  #[inline]
  pub fn add_action_arg_3(&mut self, action_arg_3: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MainCityQuestTemplate::VT_ACTION_ARG_3, action_arg_3);
  }
  #[inline]
  pub fn add_action_arg_4(&mut self, action_arg_4: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MainCityQuestTemplate::VT_ACTION_ARG_4, action_arg_4);
  }
  #[inline]
  pub fn add_unk_9(&mut self, unk_9: bool) {
    self.fbb_.push_slot::<bool>(MainCityQuestTemplate::VT_UNK_9, unk_9, false);
  }
  #[inline]
  pub fn add_unk_10(&mut self, unk_10: bool) {
    self.fbb_.push_slot::<bool>(MainCityQuestTemplate::VT_UNK_10, unk_10, false);
  }
  #[inline]
  pub fn add_unk_11(&mut self, unk_11: bool) {
    self.fbb_.push_slot::<bool>(MainCityQuestTemplate::VT_UNK_11, unk_11, false);
  }
  #[inline]
  pub fn add_unk_12(&mut self, unk_12: u32) {
    self.fbb_.push_slot::<u32>(MainCityQuestTemplate::VT_UNK_12, unk_12, 0);
  }
  #[inline]
  pub fn add_unk_13(&mut self, unk_13: u32) {
    self.fbb_.push_slot::<u32>(MainCityQuestTemplate::VT_UNK_13, unk_13, 0);
  }
  #[inline]
  pub fn add_unk_14(&mut self, unk_14: bool) {
    self.fbb_.push_slot::<bool>(MainCityQuestTemplate::VT_UNK_14, unk_14, false);
  }
  #[inline]
  pub fn add_unk_15(&mut self, unk_15: u32) {
    self.fbb_.push_slot::<u32>(MainCityQuestTemplate::VT_UNK_15, unk_15, 0);
  }
  #[inline]
  pub fn add_unk_16(&mut self, unk_16: bool) {
    self.fbb_.push_slot::<bool>(MainCityQuestTemplate::VT_UNK_16, unk_16, false);
  }
  #[inline]
  pub fn add_unk_17(&mut self, unk_17: u32) {
    self.fbb_.push_slot::<u32>(MainCityQuestTemplate::VT_UNK_17, unk_17, 0);
  }
  #[inline]
  pub fn add_unk_18(&mut self, unk_18: u32) {
    self.fbb_.push_slot::<u32>(MainCityQuestTemplate::VT_UNK_18, unk_18, 0);
  }
  #[inline]
  pub fn add_unk_19(&mut self, unk_19: u32) {
    self.fbb_.push_slot::<u32>(MainCityQuestTemplate::VT_UNK_19, unk_19, 0);
  }
  #[inline]
  pub fn add_unk_20(&mut self, unk_20: u32) {
    self.fbb_.push_slot::<u32>(MainCityQuestTemplate::VT_UNK_20, unk_20, 0);
  }
  #[inline]
  pub fn add_unk_21(&mut self, unk_21: u32) {
    self.fbb_.push_slot::<u32>(MainCityQuestTemplate::VT_UNK_21, unk_21, 0);
  }
  #[inline]
  pub fn add_unk_22(&mut self, unk_22: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u32>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MainCityQuestTemplate::VT_UNK_22, unk_22);
  }
  #[inline]
  pub fn add_unk_23(&mut self, unk_23: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u32>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MainCityQuestTemplate::VT_UNK_23, unk_23);
  }
  #[inline]
  pub fn add_npcs(&mut self, npcs: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u32>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MainCityQuestTemplate::VT_NPCS, npcs);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> MainCityQuestTemplateBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    MainCityQuestTemplateBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<MainCityQuestTemplate<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for MainCityQuestTemplate<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("MainCityQuestTemplate");
      ds.field("id", &self.id());
      ds.field("quest_name", &self.quest_name());
      ds.field("type_", &self.type_());
      ds.field("show_finish_tip", &self.show_finish_tip());
      ds.field("unk_2", &self.unk_2());
      ds.field("unk_3", &self.unk_3());
      ds.field("quest_desc", &self.quest_desc());
      ds.field("unk_5", &self.unk_5());
      ds.field("start_time", &self.start_time());
      ds.field("unk_6", &self.unk_6());
      ds.field("unk_7", &self.unk_7());
      ds.field("group", &self.group());
      ds.field("priority", &self.priority());
      ds.field("auto_distribution", &self.auto_distribution());
      ds.field("script_exec_type", &self.script_exec_type());
      ds.field("unk_8", &self.unk_8());
      ds.field("quest_begin_script", &self.quest_begin_script());
      ds.field("quest_end_script", &self.quest_end_script());
      ds.field("quest_end_action_type", &self.quest_end_action_type());
      ds.field("action_arg_1", &self.action_arg_1());
      ds.field("action_arg_2", &self.action_arg_2());
      ds.field("action_arg_3", &self.action_arg_3());
      ds.field("action_arg_4", &self.action_arg_4());
      ds.field("unk_9", &self.unk_9());
      ds.field("unk_10", &self.unk_10());
      ds.field("unk_11", &self.unk_11());
      ds.field("unk_12", &self.unk_12());
      ds.field("unk_13", &self.unk_13());
      ds.field("unk_14", &self.unk_14());
      ds.field("unk_15", &self.unk_15());
      ds.field("unk_16", &self.unk_16());
      ds.field("unk_17", &self.unk_17());
      ds.field("unk_18", &self.unk_18());
      ds.field("unk_19", &self.unk_19());
      ds.field("unk_20", &self.unk_20());
      ds.field("unk_21", &self.unk_21());
      ds.field("unk_22", &self.unk_22());
      ds.field("unk_23", &self.unk_23());
      ds.field("npcs", &self.npcs());
      ds.finish()
  }
}
pub enum MainCityQuestTemplateTbOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct MainCityQuestTemplateTb<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for MainCityQuestTemplateTb<'a> {
  type Inner = MainCityQuestTemplateTb<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> MainCityQuestTemplateTb<'a> {
  pub const VT_DATA: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    MainCityQuestTemplateTb { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args MainCityQuestTemplateTbArgs<'args>
  ) -> flatbuffers::WIPOffset<MainCityQuestTemplateTb<'bldr>> {
    let mut builder = MainCityQuestTemplateTbBuilder::new(_fbb);
    if let Some(x) = args.data { builder.add_data(x); }
    builder.finish()
  }


  #[inline]
  pub fn data(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<MainCityQuestTemplate<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<MainCityQuestTemplate>>>>(MainCityQuestTemplateTb::VT_DATA, None)}
  }
}

impl flatbuffers::Verifiable for MainCityQuestTemplateTb<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<MainCityQuestTemplate>>>>("data", Self::VT_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct MainCityQuestTemplateTbArgs<'a> {
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<MainCityQuestTemplate<'a>>>>>,
}
impl<'a> Default for MainCityQuestTemplateTbArgs<'a> {
  #[inline]
  fn default() -> Self {
    MainCityQuestTemplateTbArgs {
      data: None,
    }
  }
}

pub struct MainCityQuestTemplateTbBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> MainCityQuestTemplateTbBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<MainCityQuestTemplate<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MainCityQuestTemplateTb::VT_DATA, data);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> MainCityQuestTemplateTbBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    MainCityQuestTemplateTbBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<MainCityQuestTemplateTb<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for MainCityQuestTemplateTb<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("MainCityQuestTemplateTb");
      ds.field("data", &self.data());
      ds.finish()
  }
}
