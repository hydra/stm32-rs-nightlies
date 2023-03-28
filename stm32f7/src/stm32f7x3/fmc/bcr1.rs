///Register `BCR1` reader
pub struct R(crate::R<BCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BCR1` writer
pub struct W(crate::W<BCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MBKEN` reader - MBKEN
pub type MBKEN_R = crate::BitReader<MBKEN_A>;
///MBKEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBKEN_A {
    ///0: Corresponding memory bank is disabled
    Disabled = 0,
    ///1: Corresponding memory bank is enabled
    Enabled = 1,
}
impl From<MBKEN_A> for bool {
    #[inline(always)]
    fn from(variant: MBKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MBKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MBKEN_A {
        match self.bits {
            false => MBKEN_A::Disabled,
            true => MBKEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MBKEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MBKEN_A::Enabled
    }
}
///Field `MBKEN` writer - MBKEN
pub type MBKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, MBKEN_A, O>;
impl<'a, const O: u8> MBKEN_W<'a, O> {
    ///Corresponding memory bank is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MBKEN_A::Disabled)
    }
    ///Corresponding memory bank is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MBKEN_A::Enabled)
    }
}
///Field `MUXEN` reader - MUXEN
pub type MUXEN_R = crate::BitReader<MUXEN_A>;
///MUXEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUXEN_A {
    ///0: Address/Data non-multiplexed
    Disabled = 0,
    ///1: Address/Data multiplexed on databus
    Enabled = 1,
}
impl From<MUXEN_A> for bool {
    #[inline(always)]
    fn from(variant: MUXEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MUXEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MUXEN_A {
        match self.bits {
            false => MUXEN_A::Disabled,
            true => MUXEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUXEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUXEN_A::Enabled
    }
}
///Field `MUXEN` writer - MUXEN
pub type MUXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, MUXEN_A, O>;
impl<'a, const O: u8> MUXEN_W<'a, O> {
    ///Address/Data non-multiplexed
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MUXEN_A::Disabled)
    }
    ///Address/Data multiplexed on databus
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MUXEN_A::Enabled)
    }
}
///Field `MTYP` reader - MTYP
pub type MTYP_R = crate::FieldReader<u8, MTYP_A>;
///MTYP
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MTYP_A {
    ///0: SRAM memory type
    Sram = 0,
    ///1: PSRAM (CRAM) memory type
    Psram = 1,
    ///2: NOR Flash/OneNAND Flash
    Flash = 2,
}
impl From<MTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: MTYP_A) -> Self {
        variant as _
    }
}
impl MTYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MTYP_A> {
        match self.bits {
            0 => Some(MTYP_A::Sram),
            1 => Some(MTYP_A::Psram),
            2 => Some(MTYP_A::Flash),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Sram`
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MTYP_A::Sram
    }
    ///Checks if the value of the field is `Psram`
    #[inline(always)]
    pub fn is_psram(&self) -> bool {
        *self == MTYP_A::Psram
    }
    ///Checks if the value of the field is `Flash`
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == MTYP_A::Flash
    }
}
///Field `MTYP` writer - MTYP
pub type MTYP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR1_SPEC, u8, MTYP_A, 2, O>;
impl<'a, const O: u8> MTYP_W<'a, O> {
    ///SRAM memory type
    #[inline(always)]
    pub fn sram(self) -> &'a mut W {
        self.variant(MTYP_A::Sram)
    }
    ///PSRAM (CRAM) memory type
    #[inline(always)]
    pub fn psram(self) -> &'a mut W {
        self.variant(MTYP_A::Psram)
    }
    ///NOR Flash/OneNAND Flash
    #[inline(always)]
    pub fn flash(self) -> &'a mut W {
        self.variant(MTYP_A::Flash)
    }
}
///Field `MWID` reader - MWID
pub type MWID_R = crate::FieldReader<u8, MWID_A>;
///MWID
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MWID_A {
    ///0: Memory data bus width 8 bits
    Bits8 = 0,
    ///1: Memory data bus width 16 bits
    Bits16 = 1,
    ///2: Memory data bus width 32 bits
    Bits32 = 2,
}
impl From<MWID_A> for u8 {
    #[inline(always)]
    fn from(variant: MWID_A) -> Self {
        variant as _
    }
}
impl MWID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MWID_A> {
        match self.bits {
            0 => Some(MWID_A::Bits8),
            1 => Some(MWID_A::Bits16),
            2 => Some(MWID_A::Bits32),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Bits8`
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == MWID_A::Bits8
    }
    ///Checks if the value of the field is `Bits16`
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == MWID_A::Bits16
    }
    ///Checks if the value of the field is `Bits32`
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == MWID_A::Bits32
    }
}
///Field `MWID` writer - MWID
pub type MWID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR1_SPEC, u8, MWID_A, 2, O>;
impl<'a, const O: u8> MWID_W<'a, O> {
    ///Memory data bus width 8 bits
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(MWID_A::Bits8)
    }
    ///Memory data bus width 16 bits
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(MWID_A::Bits16)
    }
    ///Memory data bus width 32 bits
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(MWID_A::Bits32)
    }
}
///Field `FACCEN` reader - FACCEN
pub type FACCEN_R = crate::BitReader<FACCEN_A>;
///FACCEN
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FACCEN_A {
    ///0: Corresponding NOR Flash memory access is disabled
    Disabled = 0,
    ///1: Corresponding NOR Flash memory access is enabled
    Enabled = 1,
}
impl From<FACCEN_A> for bool {
    #[inline(always)]
    fn from(variant: FACCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FACCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FACCEN_A {
        match self.bits {
            false => FACCEN_A::Disabled,
            true => FACCEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FACCEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FACCEN_A::Enabled
    }
}
///Field `FACCEN` writer - FACCEN
pub type FACCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, FACCEN_A, O>;
impl<'a, const O: u8> FACCEN_W<'a, O> {
    ///Corresponding NOR Flash memory access is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FACCEN_A::Disabled)
    }
    ///Corresponding NOR Flash memory access is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FACCEN_A::Enabled)
    }
}
///Field `BURSTEN` reader - BURSTEN
pub type BURSTEN_R = crate::BitReader<BURSTEN_A>;
///BURSTEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BURSTEN_A {
    ///0: Burst mode disabled
    Disabled = 0,
    ///1: Burst mode enabled
    Enabled = 1,
}
impl From<BURSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: BURSTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BURSTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BURSTEN_A {
        match self.bits {
            false => BURSTEN_A::Disabled,
            true => BURSTEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BURSTEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BURSTEN_A::Enabled
    }
}
///Field `BURSTEN` writer - BURSTEN
pub type BURSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, BURSTEN_A, O>;
impl<'a, const O: u8> BURSTEN_W<'a, O> {
    ///Burst mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BURSTEN_A::Disabled)
    }
    ///Burst mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BURSTEN_A::Enabled)
    }
}
///Field `WAITPOL` reader - WAITPOL
pub type WAITPOL_R = crate::BitReader<WAITPOL_A>;
///WAITPOL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITPOL_A {
    ///0: NWAIT active low
    ActiveLow = 0,
    ///1: NWAIT active high
    ActiveHigh = 1,
}
impl From<WAITPOL_A> for bool {
    #[inline(always)]
    fn from(variant: WAITPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl WAITPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WAITPOL_A {
        match self.bits {
            false => WAITPOL_A::ActiveLow,
            true => WAITPOL_A::ActiveHigh,
        }
    }
    ///Checks if the value of the field is `ActiveLow`
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == WAITPOL_A::ActiveLow
    }
    ///Checks if the value of the field is `ActiveHigh`
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == WAITPOL_A::ActiveHigh
    }
}
///Field `WAITPOL` writer - WAITPOL
pub type WAITPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, WAITPOL_A, O>;
impl<'a, const O: u8> WAITPOL_W<'a, O> {
    ///NWAIT active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(WAITPOL_A::ActiveLow)
    }
    ///NWAIT active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(WAITPOL_A::ActiveHigh)
    }
}
///Field `WAITCFG` reader - WAITCFG
pub type WAITCFG_R = crate::BitReader<WAITCFG_A>;
///WAITCFG
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITCFG_A {
    ///0: NWAIT signal is active one data cycle before wait state
    BeforeWaitState = 0,
    ///1: NWAIT signal is active during wait state
    DuringWaitState = 1,
}
impl From<WAITCFG_A> for bool {
    #[inline(always)]
    fn from(variant: WAITCFG_A) -> Self {
        variant as u8 != 0
    }
}
impl WAITCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WAITCFG_A {
        match self.bits {
            false => WAITCFG_A::BeforeWaitState,
            true => WAITCFG_A::DuringWaitState,
        }
    }
    ///Checks if the value of the field is `BeforeWaitState`
    #[inline(always)]
    pub fn is_before_wait_state(&self) -> bool {
        *self == WAITCFG_A::BeforeWaitState
    }
    ///Checks if the value of the field is `DuringWaitState`
    #[inline(always)]
    pub fn is_during_wait_state(&self) -> bool {
        *self == WAITCFG_A::DuringWaitState
    }
}
///Field `WAITCFG` writer - WAITCFG
pub type WAITCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, WAITCFG_A, O>;
impl<'a, const O: u8> WAITCFG_W<'a, O> {
    ///NWAIT signal is active one data cycle before wait state
    #[inline(always)]
    pub fn before_wait_state(self) -> &'a mut W {
        self.variant(WAITCFG_A::BeforeWaitState)
    }
    ///NWAIT signal is active during wait state
    #[inline(always)]
    pub fn during_wait_state(self) -> &'a mut W {
        self.variant(WAITCFG_A::DuringWaitState)
    }
}
///Field `WREN` reader - WREN
pub type WREN_R = crate::BitReader<WREN_A>;
///WREN
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WREN_A {
    ///0: Write operations disabled for the bank by the FMC
    Disabled = 0,
    ///1: Write operations enabled for the bank by the FMC
    Enabled = 1,
}
impl From<WREN_A> for bool {
    #[inline(always)]
    fn from(variant: WREN_A) -> Self {
        variant as u8 != 0
    }
}
impl WREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WREN_A {
        match self.bits {
            false => WREN_A::Disabled,
            true => WREN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WREN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WREN_A::Enabled
    }
}
///Field `WREN` writer - WREN
pub type WREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, WREN_A, O>;
impl<'a, const O: u8> WREN_W<'a, O> {
    ///Write operations disabled for the bank by the FMC
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WREN_A::Disabled)
    }
    ///Write operations enabled for the bank by the FMC
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WREN_A::Enabled)
    }
}
///Field `WAITEN` reader - WAITEN
pub type WAITEN_R = crate::BitReader<WAITEN_A>;
///WAITEN
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITEN_A {
    ///0: Values inside the FMC_BWTR are taken into account
    Disabled = 0,
    ///1: NWAIT signal enabled
    Enabled = 1,
}
impl From<WAITEN_A> for bool {
    #[inline(always)]
    fn from(variant: WAITEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WAITEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WAITEN_A {
        match self.bits {
            false => WAITEN_A::Disabled,
            true => WAITEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAITEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAITEN_A::Enabled
    }
}
///Field `WAITEN` writer - WAITEN
pub type WAITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, WAITEN_A, O>;
impl<'a, const O: u8> WAITEN_W<'a, O> {
    ///Values inside the FMC_BWTR are taken into account
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAITEN_A::Disabled)
    }
    ///NWAIT signal enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAITEN_A::Enabled)
    }
}
///Field `EXTMOD` reader - EXTMOD
pub type EXTMOD_R = crate::BitReader<EXTMOD_A>;
///EXTMOD
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTMOD_A {
    ///0: Values inside the FMC_BWTR are not taken into account
    Disabled = 0,
    ///1: Values inside the FMC_BWTR are taken into account
    Enabled = 1,
}
impl From<EXTMOD_A> for bool {
    #[inline(always)]
    fn from(variant: EXTMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EXTMOD_A {
        match self.bits {
            false => EXTMOD_A::Disabled,
            true => EXTMOD_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTMOD_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTMOD_A::Enabled
    }
}
///Field `EXTMOD` writer - EXTMOD
pub type EXTMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, EXTMOD_A, O>;
impl<'a, const O: u8> EXTMOD_W<'a, O> {
    ///Values inside the FMC_BWTR are not taken into account
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTMOD_A::Disabled)
    }
    ///Values inside the FMC_BWTR are taken into account
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXTMOD_A::Enabled)
    }
}
///Field `ASYNCWAIT` reader - ASYNCWAIT
pub type ASYNCWAIT_R = crate::BitReader<ASYNCWAIT_A>;
///ASYNCWAIT
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASYNCWAIT_A {
    ///0: Wait signal not used in asynchronous mode
    Disabled = 0,
    ///1: Wait signal used even in asynchronous mode
    Enabled = 1,
}
impl From<ASYNCWAIT_A> for bool {
    #[inline(always)]
    fn from(variant: ASYNCWAIT_A) -> Self {
        variant as u8 != 0
    }
}
impl ASYNCWAIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ASYNCWAIT_A {
        match self.bits {
            false => ASYNCWAIT_A::Disabled,
            true => ASYNCWAIT_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ASYNCWAIT_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ASYNCWAIT_A::Enabled
    }
}
///Field `ASYNCWAIT` writer - ASYNCWAIT
pub type ASYNCWAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, ASYNCWAIT_A, O>;
impl<'a, const O: u8> ASYNCWAIT_W<'a, O> {
    ///Wait signal not used in asynchronous mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ASYNCWAIT_A::Disabled)
    }
    ///Wait signal used even in asynchronous mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ASYNCWAIT_A::Enabled)
    }
}
///Field `CPSIZE` reader - CRAM page size
pub type CPSIZE_R = crate::FieldReader<u8, CPSIZE_A>;
///CRAM page size
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPSIZE_A {
    ///0: No burst split when crossing page boundary
    NoBurstSplit = 0,
    ///1: 128 bytes CRAM page size
    Bytes128 = 1,
    ///2: 256 bytes CRAM page size
    Bytes256 = 2,
    ///3: 512 bytes CRAM page size
    Bytes512 = 3,
    ///4: 1024 bytes CRAM page size
    Bytes1024 = 4,
}
impl From<CPSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CPSIZE_A) -> Self {
        variant as _
    }
}
impl CPSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CPSIZE_A> {
        match self.bits {
            0 => Some(CPSIZE_A::NoBurstSplit),
            1 => Some(CPSIZE_A::Bytes128),
            2 => Some(CPSIZE_A::Bytes256),
            3 => Some(CPSIZE_A::Bytes512),
            4 => Some(CPSIZE_A::Bytes1024),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NoBurstSplit`
    #[inline(always)]
    pub fn is_no_burst_split(&self) -> bool {
        *self == CPSIZE_A::NoBurstSplit
    }
    ///Checks if the value of the field is `Bytes128`
    #[inline(always)]
    pub fn is_bytes128(&self) -> bool {
        *self == CPSIZE_A::Bytes128
    }
    ///Checks if the value of the field is `Bytes256`
    #[inline(always)]
    pub fn is_bytes256(&self) -> bool {
        *self == CPSIZE_A::Bytes256
    }
    ///Checks if the value of the field is `Bytes512`
    #[inline(always)]
    pub fn is_bytes512(&self) -> bool {
        *self == CPSIZE_A::Bytes512
    }
    ///Checks if the value of the field is `Bytes1024`
    #[inline(always)]
    pub fn is_bytes1024(&self) -> bool {
        *self == CPSIZE_A::Bytes1024
    }
}
///Field `CPSIZE` writer - CRAM page size
pub type CPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR1_SPEC, u8, CPSIZE_A, 3, O>;
impl<'a, const O: u8> CPSIZE_W<'a, O> {
    ///No burst split when crossing page boundary
    #[inline(always)]
    pub fn no_burst_split(self) -> &'a mut W {
        self.variant(CPSIZE_A::NoBurstSplit)
    }
    ///128 bytes CRAM page size
    #[inline(always)]
    pub fn bytes128(self) -> &'a mut W {
        self.variant(CPSIZE_A::Bytes128)
    }
    ///256 bytes CRAM page size
    #[inline(always)]
    pub fn bytes256(self) -> &'a mut W {
        self.variant(CPSIZE_A::Bytes256)
    }
    ///512 bytes CRAM page size
    #[inline(always)]
    pub fn bytes512(self) -> &'a mut W {
        self.variant(CPSIZE_A::Bytes512)
    }
    ///1024 bytes CRAM page size
    #[inline(always)]
    pub fn bytes1024(self) -> &'a mut W {
        self.variant(CPSIZE_A::Bytes1024)
    }
}
///Field `CBURSTRW` reader - CBURSTRW
pub type CBURSTRW_R = crate::BitReader<CBURSTRW_A>;
///CBURSTRW
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBURSTRW_A {
    ///0: Write operations are always performed in asynchronous mode
    Disabled = 0,
    ///1: Write operations are performed in synchronous mode
    Enabled = 1,
}
impl From<CBURSTRW_A> for bool {
    #[inline(always)]
    fn from(variant: CBURSTRW_A) -> Self {
        variant as u8 != 0
    }
}
impl CBURSTRW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CBURSTRW_A {
        match self.bits {
            false => CBURSTRW_A::Disabled,
            true => CBURSTRW_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CBURSTRW_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CBURSTRW_A::Enabled
    }
}
///Field `CBURSTRW` writer - CBURSTRW
pub type CBURSTRW_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, CBURSTRW_A, O>;
impl<'a, const O: u8> CBURSTRW_W<'a, O> {
    ///Write operations are always performed in asynchronous mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CBURSTRW_A::Disabled)
    }
    ///Write operations are performed in synchronous mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CBURSTRW_A::Enabled)
    }
}
///Field `CCLKEN` reader - CCLKEN
pub type CCLKEN_R = crate::BitReader<CCLKEN_A>;
///CCLKEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLKEN_A {
    ///0: The FMC_CLK is generated continuously during asynchronous and synchronous access. The FMC_CLK clock is activated when the CCLKEN is set
    Disabled = 0,
    ///1: The FMC_CLK is only generated during the synchronous memory access (read/write transaction)
    Enabled = 1,
}
impl From<CCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CCLKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CCLKEN_A {
        match self.bits {
            false => CCLKEN_A::Disabled,
            true => CCLKEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCLKEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCLKEN_A::Enabled
    }
}
///Field `CCLKEN` writer - CCLKEN
pub type CCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, CCLKEN_A, O>;
impl<'a, const O: u8> CCLKEN_W<'a, O> {
    ///The FMC_CLK is generated continuously during asynchronous and synchronous access. The FMC_CLK clock is activated when the CCLKEN is set
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CCLKEN_A::Disabled)
    }
    ///The FMC_CLK is only generated during the synchronous memory access (read/write transaction)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CCLKEN_A::Enabled)
    }
}
///Field `WFDIS` reader - Write FIFO Disable
pub type WFDIS_R = crate::BitReader<WFDIS_A>;
///Write FIFO Disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WFDIS_A {
    ///0: Write FIFO enabled
    Enabled = 0,
    ///1: Write FIFO disabled
    Disabled = 1,
}
impl From<WFDIS_A> for bool {
    #[inline(always)]
    fn from(variant: WFDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl WFDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WFDIS_A {
        match self.bits {
            false => WFDIS_A::Enabled,
            true => WFDIS_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WFDIS_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WFDIS_A::Disabled
    }
}
///Field `WFDIS` writer - Write FIFO Disable
pub type WFDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, WFDIS_A, O>;
impl<'a, const O: u8> WFDIS_W<'a, O> {
    ///Write FIFO enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WFDIS_A::Enabled)
    }
    ///Write FIFO disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WFDIS_A::Disabled)
    }
}
impl R {
    ///Bit 0 - MBKEN
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MUXEN
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - MTYP
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - MWID
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - FACCEN
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - BURSTEN
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - WAITPOL
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - WAITCFG
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - WREN
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - WAITEN
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - EXTMOD
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ASYNCWAIT
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - CRAM page size
    #[inline(always)]
    pub fn cpsize(&self) -> CPSIZE_R {
        CPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 19 - CBURSTRW
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - CCLKEN
    #[inline(always)]
    pub fn cclken(&self) -> CCLKEN_R {
        CCLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Write FIFO Disable
    #[inline(always)]
    pub fn wfdis(&self) -> WFDIS_R {
        WFDIS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MBKEN
    #[inline(always)]
    #[must_use]
    pub fn mbken(&mut self) -> MBKEN_W<0> {
        MBKEN_W::new(self)
    }
    ///Bit 1 - MUXEN
    #[inline(always)]
    #[must_use]
    pub fn muxen(&mut self) -> MUXEN_W<1> {
        MUXEN_W::new(self)
    }
    ///Bits 2:3 - MTYP
    #[inline(always)]
    #[must_use]
    pub fn mtyp(&mut self) -> MTYP_W<2> {
        MTYP_W::new(self)
    }
    ///Bits 4:5 - MWID
    #[inline(always)]
    #[must_use]
    pub fn mwid(&mut self) -> MWID_W<4> {
        MWID_W::new(self)
    }
    ///Bit 6 - FACCEN
    #[inline(always)]
    #[must_use]
    pub fn faccen(&mut self) -> FACCEN_W<6> {
        FACCEN_W::new(self)
    }
    ///Bit 8 - BURSTEN
    #[inline(always)]
    #[must_use]
    pub fn bursten(&mut self) -> BURSTEN_W<8> {
        BURSTEN_W::new(self)
    }
    ///Bit 9 - WAITPOL
    #[inline(always)]
    #[must_use]
    pub fn waitpol(&mut self) -> WAITPOL_W<9> {
        WAITPOL_W::new(self)
    }
    ///Bit 11 - WAITCFG
    #[inline(always)]
    #[must_use]
    pub fn waitcfg(&mut self) -> WAITCFG_W<11> {
        WAITCFG_W::new(self)
    }
    ///Bit 12 - WREN
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WREN_W<12> {
        WREN_W::new(self)
    }
    ///Bit 13 - WAITEN
    #[inline(always)]
    #[must_use]
    pub fn waiten(&mut self) -> WAITEN_W<13> {
        WAITEN_W::new(self)
    }
    ///Bit 14 - EXTMOD
    #[inline(always)]
    #[must_use]
    pub fn extmod(&mut self) -> EXTMOD_W<14> {
        EXTMOD_W::new(self)
    }
    ///Bit 15 - ASYNCWAIT
    #[inline(always)]
    #[must_use]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<15> {
        ASYNCWAIT_W::new(self)
    }
    ///Bits 16:18 - CRAM page size
    #[inline(always)]
    #[must_use]
    pub fn cpsize(&mut self) -> CPSIZE_W<16> {
        CPSIZE_W::new(self)
    }
    ///Bit 19 - CBURSTRW
    #[inline(always)]
    #[must_use]
    pub fn cburstrw(&mut self) -> CBURSTRW_W<19> {
        CBURSTRW_W::new(self)
    }
    ///Bit 20 - CCLKEN
    #[inline(always)]
    #[must_use]
    pub fn cclken(&mut self) -> CCLKEN_W<20> {
        CCLKEN_W::new(self)
    }
    ///Bit 21 - Write FIFO Disable
    #[inline(always)]
    #[must_use]
    pub fn wfdis(&mut self) -> WFDIS_W<21> {
        WFDIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SRAM/NOR-Flash chip-select control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bcr1](index.html) module
pub struct BCR1_SPEC;
impl crate::RegisterSpec for BCR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [bcr1::R](R) reader structure
impl crate::Readable for BCR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bcr1::W](W) writer structure
impl crate::Writable for BCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BCR1 to value 0x30d0
impl crate::Resettable for BCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x30d0;
}
