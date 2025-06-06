// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum EquipmentLevelTemplateOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct EquipmentLevelTemplate<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for EquipmentLevelTemplate<'a> {
  type Inner = EquipmentLevelTemplate<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> EquipmentLevelTemplate<'a> {
  pub const VT_RARITY: flatbuffers::VOffsetT = 4;
  pub const VT_LEVEL: flatbuffers::VOffsetT = 6;
  pub const VT_PROPERTY_RATE: flatbuffers::VOffsetT = 8;
  pub const VT_UNK_3: flatbuffers::VOffsetT = 10;
  pub const VT_UNK_4: flatbuffers::VOffsetT = 12;
  pub const VT_UNK_5: flatbuffers::VOffsetT = 14;
  pub const VT_UNK_6: flatbuffers::VOffsetT = 16;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    EquipmentLevelTemplate { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args EquipmentLevelTemplateArgs
  ) -> flatbuffers::WIPOffset<EquipmentLevelTemplate<'bldr>> {
    let mut builder = EquipmentLevelTemplateBuilder::new(_fbb);
    builder.add_unk_6(args.unk_6);
    builder.add_unk_5(args.unk_5);
    builder.add_unk_4(args.unk_4);
    builder.add_unk_3(args.unk_3);
    builder.add_property_rate(args.property_rate);
    builder.add_level(args.level);
    builder.add_rarity(args.rarity);
    builder.finish()
  }


  #[inline]
  pub fn rarity(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(EquipmentLevelTemplate::VT_RARITY, Some(0)).unwrap()}
  }
  #[inline]
  pub fn level(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(EquipmentLevelTemplate::VT_LEVEL, Some(0)).unwrap()}
  }
  #[inline]
  pub fn property_rate(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(EquipmentLevelTemplate::VT_PROPERTY_RATE, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_3(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(EquipmentLevelTemplate::VT_UNK_3, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_4(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(EquipmentLevelTemplate::VT_UNK_4, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_5(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(EquipmentLevelTemplate::VT_UNK_5, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_6(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(EquipmentLevelTemplate::VT_UNK_6, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for EquipmentLevelTemplate<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u32>("rarity", Self::VT_RARITY, false)?
     .visit_field::<u32>("level", Self::VT_LEVEL, false)?
     .visit_field::<u32>("property_rate", Self::VT_PROPERTY_RATE, false)?
     .visit_field::<i32>("unk_3", Self::VT_UNK_3, false)?
     .visit_field::<i32>("unk_4", Self::VT_UNK_4, false)?
     .visit_field::<i32>("unk_5", Self::VT_UNK_5, false)?
     .visit_field::<i32>("unk_6", Self::VT_UNK_6, false)?
     .finish();
    Ok(())
  }
}
pub struct EquipmentLevelTemplateArgs {
    pub rarity: u32,
    pub level: u32,
    pub property_rate: u32,
    pub unk_3: i32,
    pub unk_4: i32,
    pub unk_5: i32,
    pub unk_6: i32,
}
impl<'a> Default for EquipmentLevelTemplateArgs {
  #[inline]
  fn default() -> Self {
    EquipmentLevelTemplateArgs {
      rarity: 0,
      level: 0,
      property_rate: 0,
      unk_3: 0,
      unk_4: 0,
      unk_5: 0,
      unk_6: 0,
    }
  }
}

pub struct EquipmentLevelTemplateBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> EquipmentLevelTemplateBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_rarity(&mut self, rarity: u32) {
    self.fbb_.push_slot::<u32>(EquipmentLevelTemplate::VT_RARITY, rarity, 0);
  }
  #[inline]
  pub fn add_level(&mut self, level: u32) {
    self.fbb_.push_slot::<u32>(EquipmentLevelTemplate::VT_LEVEL, level, 0);
  }
  #[inline]
  pub fn add_property_rate(&mut self, property_rate: u32) {
    self.fbb_.push_slot::<u32>(EquipmentLevelTemplate::VT_PROPERTY_RATE, property_rate, 0);
  }
  #[inline]
  pub fn add_unk_3(&mut self, unk_3: i32) {
    self.fbb_.push_slot::<i32>(EquipmentLevelTemplate::VT_UNK_3, unk_3, 0);
  }
  #[inline]
  pub fn add_unk_4(&mut self, unk_4: i32) {
    self.fbb_.push_slot::<i32>(EquipmentLevelTemplate::VT_UNK_4, unk_4, 0);
  }
  #[inline]
  pub fn add_unk_5(&mut self, unk_5: i32) {
    self.fbb_.push_slot::<i32>(EquipmentLevelTemplate::VT_UNK_5, unk_5, 0);
  }
  #[inline]
  pub fn add_unk_6(&mut self, unk_6: i32) {
    self.fbb_.push_slot::<i32>(EquipmentLevelTemplate::VT_UNK_6, unk_6, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> EquipmentLevelTemplateBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    EquipmentLevelTemplateBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<EquipmentLevelTemplate<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for EquipmentLevelTemplate<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("EquipmentLevelTemplate");
      ds.field("rarity", &self.rarity());
      ds.field("level", &self.level());
      ds.field("property_rate", &self.property_rate());
      ds.field("unk_3", &self.unk_3());
      ds.field("unk_4", &self.unk_4());
      ds.field("unk_5", &self.unk_5());
      ds.field("unk_6", &self.unk_6());
      ds.finish()
  }
}
pub enum EquipmentLevelTemplateTbOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct EquipmentLevelTemplateTb<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for EquipmentLevelTemplateTb<'a> {
  type Inner = EquipmentLevelTemplateTb<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> EquipmentLevelTemplateTb<'a> {
  pub const VT_DATA: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    EquipmentLevelTemplateTb { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args EquipmentLevelTemplateTbArgs<'args>
  ) -> flatbuffers::WIPOffset<EquipmentLevelTemplateTb<'bldr>> {
    let mut builder = EquipmentLevelTemplateTbBuilder::new(_fbb);
    if let Some(x) = args.data { builder.add_data(x); }
    builder.finish()
  }


  #[inline]
  pub fn data(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<EquipmentLevelTemplate<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<EquipmentLevelTemplate>>>>(EquipmentLevelTemplateTb::VT_DATA, None)}
  }
}

impl flatbuffers::Verifiable for EquipmentLevelTemplateTb<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<EquipmentLevelTemplate>>>>("data", Self::VT_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct EquipmentLevelTemplateTbArgs<'a> {
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<EquipmentLevelTemplate<'a>>>>>,
}
impl<'a> Default for EquipmentLevelTemplateTbArgs<'a> {
  #[inline]
  fn default() -> Self {
    EquipmentLevelTemplateTbArgs {
      data: None,
    }
  }
}

pub struct EquipmentLevelTemplateTbBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> EquipmentLevelTemplateTbBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<EquipmentLevelTemplate<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(EquipmentLevelTemplateTb::VT_DATA, data);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> EquipmentLevelTemplateTbBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    EquipmentLevelTemplateTbBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<EquipmentLevelTemplateTb<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for EquipmentLevelTemplateTb<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("EquipmentLevelTemplateTb");
      ds.field("data", &self.data());
      ds.finish()
  }
}
