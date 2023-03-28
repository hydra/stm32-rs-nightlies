///Register `CCR4` reader
pub struct R(crate::R<CCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR4` writer
pub struct W(crate::W<CCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR4_SPEC>;
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
impl From<crate::W<CCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - channel enable
pub type EN_R = crate::BitReader<EN_A>;
///channel enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::Disabled,
            true => EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::Enabled
    }
}
///Field `EN` writer - channel enable
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::Enabled)
    }
}
///Field `TCIE` reader - transfer complete interrupt enable
pub type TCIE_R = crate::BitReader<TCIE_A>;
///transfer complete interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE_A {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::Disabled,
            true => TCIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE_A::Enabled
    }
}
///Field `TCIE` writer - transfer complete interrupt enable
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, TCIE_A, O>;
impl<'a, const O: u8> TCIE_W<'a, O> {
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::Enabled)
    }
}
///Field `HTIE` reader - half transfer interrupt enable
pub type HTIE_R = crate::BitReader<HTIE_A>;
///half transfer interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIE_A {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<HTIE_A> for bool {
    #[inline(always)]
    fn from(variant: HTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HTIE_A {
        match self.bits {
            false => HTIE_A::Disabled,
            true => HTIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HTIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HTIE_A::Enabled
    }
}
///Field `HTIE` writer - half transfer interrupt enable
pub type HTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, HTIE_A, O>;
impl<'a, const O: u8> HTIE_W<'a, O> {
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HTIE_A::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HTIE_A::Enabled)
    }
}
///Field `TEIE` reader - transfer error interrupt enable
pub type TEIE_R = crate::BitReader<TEIE_A>;
///transfer error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE_A {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::Disabled,
            true => TEIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEIE_A::Enabled
    }
}
///Field `TEIE` writer - transfer error interrupt enable
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, TEIE_A, O>;
impl<'a, const O: u8> TEIE_W<'a, O> {
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEIE_A::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEIE_A::Enabled)
    }
}
///Field `DIR` reader - data transfer direction
pub type DIR_R = crate::BitReader<DIR_A>;
///data transfer direction
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    ///0: Read from peripheral
    Peripheral = 0,
    ///1: Read from memory
    Memory = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::Peripheral,
            true => DIR_A::Memory,
        }
    }
    ///Checks if the value of the field is `Peripheral`
    #[inline(always)]
    pub fn is_peripheral(&self) -> bool {
        *self == DIR_A::Peripheral
    }
    ///Checks if the value of the field is `Memory`
    #[inline(always)]
    pub fn is_memory(&self) -> bool {
        *self == DIR_A::Memory
    }
}
///Field `DIR` writer - data transfer direction
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, DIR_A, O>;
impl<'a, const O: u8> DIR_W<'a, O> {
    ///Read from peripheral
    #[inline(always)]
    pub fn peripheral(self) -> &'a mut W {
        self.variant(DIR_A::Peripheral)
    }
    ///Read from memory
    #[inline(always)]
    pub fn memory(self) -> &'a mut W {
        self.variant(DIR_A::Memory)
    }
}
///Field `CIRC` reader - circular mode
pub type CIRC_R = crate::BitReader<CIRC_A>;
///circular mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIRC_A {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<CIRC_A> for bool {
    #[inline(always)]
    fn from(variant: CIRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CIRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CIRC_A {
        match self.bits {
            false => CIRC_A::Disabled,
            true => CIRC_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CIRC_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CIRC_A::Enabled
    }
}
///Field `CIRC` writer - circular mode
pub type CIRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, CIRC_A, O>;
impl<'a, const O: u8> CIRC_W<'a, O> {
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CIRC_A::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CIRC_A::Enabled)
    }
}
///Field `PINC` reader - peripheral increment mode
pub type PINC_R = crate::BitReader<PINC_A>;
///peripheral increment mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINC_A {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<PINC_A> for bool {
    #[inline(always)]
    fn from(variant: PINC_A) -> Self {
        variant as u8 != 0
    }
}
impl PINC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PINC_A {
        match self.bits {
            false => PINC_A::Disabled,
            true => PINC_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINC_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINC_A::Enabled
    }
}
///Field `PINC` writer - peripheral increment mode
pub type PINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, PINC_A, O>;
impl<'a, const O: u8> PINC_W<'a, O> {
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINC_A::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINC_A::Enabled)
    }
}
///Field `MINC` reader - memory increment mode
pub type MINC_R = crate::BitReader<MINC_A>;
///memory increment mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINC_A {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<MINC_A> for bool {
    #[inline(always)]
    fn from(variant: MINC_A) -> Self {
        variant as u8 != 0
    }
}
impl MINC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MINC_A {
        match self.bits {
            false => MINC_A::Disabled,
            true => MINC_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MINC_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MINC_A::Enabled
    }
}
///Field `MINC` writer - memory increment mode
pub type MINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, MINC_A, O>;
impl<'a, const O: u8> MINC_W<'a, O> {
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MINC_A::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MINC_A::Enabled)
    }
}
///Field `PSIZE` reader - peripheral size
pub type PSIZE_R = crate::FieldReader<u8, PSIZE_A>;
///peripheral size
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSIZE_A {
    ///0: 8 bits
    Bits8 = 0,
    ///1: 16 bits
    Bits16 = 1,
    ///2: 32 bits
    Bits32 = 2,
}
impl From<PSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PSIZE_A) -> Self {
        variant as _
    }
}
impl PSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PSIZE_A> {
        match self.bits {
            0 => Some(PSIZE_A::Bits8),
            1 => Some(PSIZE_A::Bits16),
            2 => Some(PSIZE_A::Bits32),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Bits8`
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == PSIZE_A::Bits8
    }
    ///Checks if the value of the field is `Bits16`
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == PSIZE_A::Bits16
    }
    ///Checks if the value of the field is `Bits32`
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == PSIZE_A::Bits32
    }
}
///Field `PSIZE` writer - peripheral size
pub type PSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR4_SPEC, u8, PSIZE_A, 2, O>;
impl<'a, const O: u8> PSIZE_W<'a, O> {
    ///8 bits
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(PSIZE_A::Bits8)
    }
    ///16 bits
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(PSIZE_A::Bits16)
    }
    ///32 bits
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(PSIZE_A::Bits32)
    }
}
///Field `MSIZE` reader - memory size
pub type MSIZE_R = crate::FieldReader<u8, MSIZE_A>;
///memory size
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSIZE_A {
    ///0: 8 bits
    Bits8 = 0,
    ///1: 16 bits
    Bits16 = 1,
    ///2: 32 bits
    Bits32 = 2,
}
impl From<MSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSIZE_A) -> Self {
        variant as _
    }
}
impl MSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MSIZE_A> {
        match self.bits {
            0 => Some(MSIZE_A::Bits8),
            1 => Some(MSIZE_A::Bits16),
            2 => Some(MSIZE_A::Bits32),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Bits8`
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == MSIZE_A::Bits8
    }
    ///Checks if the value of the field is `Bits16`
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == MSIZE_A::Bits16
    }
    ///Checks if the value of the field is `Bits32`
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == MSIZE_A::Bits32
    }
}
///Field `MSIZE` writer - memory size
pub type MSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR4_SPEC, u8, MSIZE_A, 2, O>;
impl<'a, const O: u8> MSIZE_W<'a, O> {
    ///8 bits
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(MSIZE_A::Bits8)
    }
    ///16 bits
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(MSIZE_A::Bits16)
    }
    ///32 bits
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(MSIZE_A::Bits32)
    }
}
///Field `PL` reader - priority level
pub type PL_R = crate::FieldReader<u8, PL_A>;
///priority level
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PL_A {
    ///0: Low
    Low = 0,
    ///1: Medium
    Medium = 1,
    ///2: High
    High = 2,
    ///3: Very high
    VeryHigh = 3,
}
impl From<PL_A> for u8 {
    #[inline(always)]
    fn from(variant: PL_A) -> Self {
        variant as _
    }
}
impl PL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PL_A {
        match self.bits {
            0 => PL_A::Low,
            1 => PL_A::Medium,
            2 => PL_A::High,
            3 => PL_A::VeryHigh,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PL_A::Low
    }
    ///Checks if the value of the field is `Medium`
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == PL_A::Medium
    }
    ///Checks if the value of the field is `High`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PL_A::High
    }
    ///Checks if the value of the field is `VeryHigh`
    #[inline(always)]
    pub fn is_very_high(&self) -> bool {
        *self == PL_A::VeryHigh
    }
}
///Field `PL` writer - priority level
pub type PL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR4_SPEC, u8, PL_A, 2, O>;
impl<'a, const O: u8> PL_W<'a, O> {
    ///Low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PL_A::Low)
    }
    ///Medium
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(PL_A::Medium)
    }
    ///High
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PL_A::High)
    }
    ///Very high
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(PL_A::VeryHigh)
    }
}
///Field `MEM2MEM` reader - memory-to-memory mode
pub type MEM2MEM_R = crate::BitReader<MEM2MEM_A>;
///memory-to-memory mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM2MEM_A {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<MEM2MEM_A> for bool {
    #[inline(always)]
    fn from(variant: MEM2MEM_A) -> Self {
        variant as u8 != 0
    }
}
impl MEM2MEM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MEM2MEM_A {
        match self.bits {
            false => MEM2MEM_A::Disabled,
            true => MEM2MEM_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MEM2MEM_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MEM2MEM_A::Enabled
    }
}
///Field `MEM2MEM` writer - memory-to-memory mode
pub type MEM2MEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, MEM2MEM_A, O>;
impl<'a, const O: u8> MEM2MEM_W<'a, O> {
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MEM2MEM_A::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MEM2MEM_A::Enabled)
    }
}
///Field `SECM` reader - ecure mode
pub type SECM_R = crate::BitReader<bool>;
///Field `SECM` writer - ecure mode
pub type SECM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, bool, O>;
///Field `SSEC` reader - ecurity of the DMA transfer from the source
pub type SSEC_R = crate::BitReader<bool>;
///Field `SSEC` writer - ecurity of the DMA transfer from the source
pub type SSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, bool, O>;
///Field `DSEC` reader - ecurity of the DMA transfer to the destination
pub type DSEC_R = crate::BitReader<bool>;
///Field `DSEC` writer - ecurity of the DMA transfer to the destination
pub type DSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, bool, O>;
///Field `PRIV` reader - rivileged mode
pub type PRIV_R = crate::BitReader<PRIV_A>;
///rivileged mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV_A {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV_A) -> Self {
        variant as u8 != 0
    }
}
impl PRIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PRIV_A {
        match self.bits {
            false => PRIV_A::Disabled,
            true => PRIV_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRIV_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRIV_A::Enabled
    }
}
///Field `PRIV` writer - rivileged mode
pub type PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, PRIV_A, O>;
impl<'a, const O: u8> PRIV_W<'a, O> {
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRIV_A::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRIV_A::Enabled)
    }
}
impl R {
    ///Bit 0 - channel enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - half transfer interrupt enable
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - data transfer direction
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - circular mode
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - peripheral increment mode
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - memory increment mode
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - peripheral size
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - memory size
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - priority level
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - memory-to-memory mode
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - ecure mode
    #[inline(always)]
    pub fn secm(&self) -> SECM_R {
        SECM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ecurity of the DMA transfer from the source
    #[inline(always)]
    pub fn ssec(&self) -> SSEC_R {
        SSEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ecurity of the DMA transfer to the destination
    #[inline(always)]
    pub fn dsec(&self) -> DSEC_R {
        DSEC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - rivileged mode
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - channel enable
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bit 1 - transfer complete interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<1> {
        TCIE_W::new(self)
    }
    ///Bit 2 - half transfer interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<2> {
        HTIE_W::new(self)
    }
    ///Bit 3 - transfer error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<3> {
        TEIE_W::new(self)
    }
    ///Bit 4 - data transfer direction
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<4> {
        DIR_W::new(self)
    }
    ///Bit 5 - circular mode
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CIRC_W<5> {
        CIRC_W::new(self)
    }
    ///Bit 6 - peripheral increment mode
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PINC_W<6> {
        PINC_W::new(self)
    }
    ///Bit 7 - memory increment mode
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MINC_W<7> {
        MINC_W::new(self)
    }
    ///Bits 8:9 - peripheral size
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<8> {
        PSIZE_W::new(self)
    }
    ///Bits 10:11 - memory size
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MSIZE_W<10> {
        MSIZE_W::new(self)
    }
    ///Bits 12:13 - priority level
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<12> {
        PL_W::new(self)
    }
    ///Bit 14 - memory-to-memory mode
    #[inline(always)]
    #[must_use]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<14> {
        MEM2MEM_W::new(self)
    }
    ///Bit 17 - ecure mode
    #[inline(always)]
    #[must_use]
    pub fn secm(&mut self) -> SECM_W<17> {
        SECM_W::new(self)
    }
    ///Bit 18 - ecurity of the DMA transfer from the source
    #[inline(always)]
    #[must_use]
    pub fn ssec(&mut self) -> SSEC_W<18> {
        SSEC_W::new(self)
    }
    ///Bit 19 - ecurity of the DMA transfer to the destination
    #[inline(always)]
    #[must_use]
    pub fn dsec(&mut self) -> DSEC_W<19> {
        DSEC_W::new(self)
    }
    ///Bit 20 - rivileged mode
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PRIV_W<20> {
        PRIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///channel x configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr4](index.html) module
pub struct CCR4_SPEC;
impl crate::RegisterSpec for CCR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr4::R](R) reader structure
impl crate::Readable for CCR4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr4::W](W) writer structure
impl crate::Writable for CCR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCR4 to value 0
impl crate::Resettable for CCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
