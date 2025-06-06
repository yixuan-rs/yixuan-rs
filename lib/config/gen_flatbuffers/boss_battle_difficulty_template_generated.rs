// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum BossBattleDifficultyTemplateOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct BossBattleDifficultyTemplate<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for BossBattleDifficultyTemplate<'a> {
  type Inner = BossBattleDifficultyTemplate<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> BossBattleDifficultyTemplate<'a> {
  pub const VT_DIFFICULTY: flatbuffers::VOffsetT = 4;
  pub const VT_BATTLE_TYPE: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    BossBattleDifficultyTemplate { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args BossBattleDifficultyTemplateArgs
  ) -> flatbuffers::WIPOffset<BossBattleDifficultyTemplate<'bldr>> {
    let mut builder = BossBattleDifficultyTemplateBuilder::new(_fbb);
    builder.add_battle_type(args.battle_type);
    builder.add_difficulty(args.difficulty);
    builder.finish()
  }


  #[inline]
  pub fn difficulty(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(BossBattleDifficultyTemplate::VT_DIFFICULTY, Some(0)).unwrap()}
  }
  #[inline]
  pub fn battle_type(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(BossBattleDifficultyTemplate::VT_BATTLE_TYPE, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for BossBattleDifficultyTemplate<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u32>("difficulty", Self::VT_DIFFICULTY, false)?
     .visit_field::<u32>("battle_type", Self::VT_BATTLE_TYPE, false)?
     .finish();
    Ok(())
  }
}
pub struct BossBattleDifficultyTemplateArgs {
    pub difficulty: u32,
    pub battle_type: u32,
}
impl<'a> Default for BossBattleDifficultyTemplateArgs {
  #[inline]
  fn default() -> Self {
    BossBattleDifficultyTemplateArgs {
      difficulty: 0,
      battle_type: 0,
    }
  }
}

pub struct BossBattleDifficultyTemplateBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> BossBattleDifficultyTemplateBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_difficulty(&mut self, difficulty: u32) {
    self.fbb_.push_slot::<u32>(BossBattleDifficultyTemplate::VT_DIFFICULTY, difficulty, 0);
  }
  #[inline]
  pub fn add_battle_type(&mut self, battle_type: u32) {
    self.fbb_.push_slot::<u32>(BossBattleDifficultyTemplate::VT_BATTLE_TYPE, battle_type, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> BossBattleDifficultyTemplateBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    BossBattleDifficultyTemplateBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<BossBattleDifficultyTemplate<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for BossBattleDifficultyTemplate<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("BossBattleDifficultyTemplate");
      ds.field("difficulty", &self.difficulty());
      ds.field("battle_type", &self.battle_type());
      ds.finish()
  }
}
pub enum BossBattleDifficultyTemplateTbOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct BossBattleDifficultyTemplateTb<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for BossBattleDifficultyTemplateTb<'a> {
  type Inner = BossBattleDifficultyTemplateTb<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> BossBattleDifficultyTemplateTb<'a> {
  pub const VT_DATA: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    BossBattleDifficultyTemplateTb { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args BossBattleDifficultyTemplateTbArgs<'args>
  ) -> flatbuffers::WIPOffset<BossBattleDifficultyTemplateTb<'bldr>> {
    let mut builder = BossBattleDifficultyTemplateTbBuilder::new(_fbb);
    if let Some(x) = args.data { builder.add_data(x); }
    builder.finish()
  }


  #[inline]
  pub fn data(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<BossBattleDifficultyTemplate<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<BossBattleDifficultyTemplate>>>>(BossBattleDifficultyTemplateTb::VT_DATA, None)}
  }
}

impl flatbuffers::Verifiable for BossBattleDifficultyTemplateTb<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<BossBattleDifficultyTemplate>>>>("data", Self::VT_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct BossBattleDifficultyTemplateTbArgs<'a> {
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<BossBattleDifficultyTemplate<'a>>>>>,
}
impl<'a> Default for BossBattleDifficultyTemplateTbArgs<'a> {
  #[inline]
  fn default() -> Self {
    BossBattleDifficultyTemplateTbArgs {
      data: None,
    }
  }
}

pub struct BossBattleDifficultyTemplateTbBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> BossBattleDifficultyTemplateTbBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<BossBattleDifficultyTemplate<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(BossBattleDifficultyTemplateTb::VT_DATA, data);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> BossBattleDifficultyTemplateTbBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    BossBattleDifficultyTemplateTbBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<BossBattleDifficultyTemplateTb<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for BossBattleDifficultyTemplateTb<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("BossBattleDifficultyTemplateTb");
      ds.field("data", &self.data());
      ds.finish()
  }
}
