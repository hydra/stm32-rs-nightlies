///Register `MMCCR` reader
pub struct R(crate::R<MMCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MMCCR` writer
pub struct W(crate::W<MMCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCCR_SPEC>;
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
impl From<crate::W<MMCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CR` reader - Counter reset
pub type CR_R = crate::BitReader<CR_A>;
///Counter reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CR_A {
    ///1: Reset all counters. Cleared automatically
    Reset = 1,
}
impl From<CR_A> for bool {
    #[inline(always)]
    fn from(variant: CR_A) -> Self {
        variant as u8 != 0
    }
}
impl CR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CR_A> {
        match self.bits {
            true => Some(CR_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CR_A::Reset
    }
}
///Field `CR` writer - Counter reset
pub type CR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, CR_A, O>;
impl<'a, const O: u8> CR_W<'a, O> {
    ///Reset all counters. Cleared automatically
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CR_A::Reset)
    }
}
///Field `CSR` reader - Counter stop rollover
pub type CSR_R = crate::BitReader<CSR_A>;
///Counter stop rollover
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSR_A {
    ///0: Counters roll over to zero after reaching the maximum value
    Disabled = 0,
    ///1: Counters do not roll over to zero after reaching the maximum value
    Enabled = 1,
}
impl From<CSR_A> for bool {
    #[inline(always)]
    fn from(variant: CSR_A) -> Self {
        variant as u8 != 0
    }
}
impl CSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSR_A {
        match self.bits {
            false => CSR_A::Disabled,
            true => CSR_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSR_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSR_A::Enabled
    }
}
///Field `CSR` writer - Counter stop rollover
pub type CSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, CSR_A, O>;
impl<'a, const O: u8> CSR_W<'a, O> {
    ///Counters roll over to zero after reaching the maximum value
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSR_A::Disabled)
    }
    ///Counters do not roll over to zero after reaching the maximum value
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSR_A::Enabled)
    }
}
///Field `ROR` reader - Reset on read
pub type ROR_R = crate::BitReader<ROR_A>;
///Reset on read
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROR_A {
    ///0: MMC counters do not reset on read
    Disabled = 0,
    ///1: MMC counters reset to zero after read
    Enabled = 1,
}
impl From<ROR_A> for bool {
    #[inline(always)]
    fn from(variant: ROR_A) -> Self {
        variant as u8 != 0
    }
}
impl ROR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ROR_A {
        match self.bits {
            false => ROR_A::Disabled,
            true => ROR_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROR_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROR_A::Enabled
    }
}
///Field `ROR` writer - Reset on read
pub type ROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, ROR_A, O>;
impl<'a, const O: u8> ROR_W<'a, O> {
    ///MMC counters do not reset on read
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROR_A::Disabled)
    }
    ///MMC counters reset to zero after read
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROR_A::Enabled)
    }
}
///Field `MCF` reader - MMC counter freeze
pub type MCF_R = crate::BitReader<MCF_A>;
///MMC counter freeze
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCF_A {
    ///0: All MMC counters update normally
    Unfrozen = 0,
    ///1: All MMC counters frozen to their current value
    Frozen = 1,
}
impl From<MCF_A> for bool {
    #[inline(always)]
    fn from(variant: MCF_A) -> Self {
        variant as u8 != 0
    }
}
impl MCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MCF_A {
        match self.bits {
            false => MCF_A::Unfrozen,
            true => MCF_A::Frozen,
        }
    }
    ///Checks if the value of the field is `Unfrozen`
    #[inline(always)]
    pub fn is_unfrozen(&self) -> bool {
        *self == MCF_A::Unfrozen
    }
    ///Checks if the value of the field is `Frozen`
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == MCF_A::Frozen
    }
}
///Field `MCF` writer - MMC counter freeze
pub type MCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, MCF_A, O>;
impl<'a, const O: u8> MCF_W<'a, O> {
    ///All MMC counters update normally
    #[inline(always)]
    pub fn unfrozen(self) -> &'a mut W {
        self.variant(MCF_A::Unfrozen)
    }
    ///All MMC counters frozen to their current value
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(MCF_A::Frozen)
    }
}
///Field `MCP` reader - MMC counter preset
pub type MCP_R = crate::BitReader<MCP_A>;
///MMC counter preset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCP_A {
    ///1: MMC counters will be preset to almost full or almost half. Cleared automatically
    Preset = 1,
}
impl From<MCP_A> for bool {
    #[inline(always)]
    fn from(variant: MCP_A) -> Self {
        variant as u8 != 0
    }
}
impl MCP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MCP_A> {
        match self.bits {
            true => Some(MCP_A::Preset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Preset`
    #[inline(always)]
    pub fn is_preset(&self) -> bool {
        *self == MCP_A::Preset
    }
}
///Field `MCP` writer - MMC counter preset
pub type MCP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, MCP_A, O>;
impl<'a, const O: u8> MCP_W<'a, O> {
    ///MMC counters will be preset to almost full or almost half. Cleared automatically
    #[inline(always)]
    pub fn preset(self) -> &'a mut W {
        self.variant(MCP_A::Preset)
    }
}
///Field `MCFHP` reader - MMC counter Full-Half preset
pub type MCFHP_R = crate::BitReader<MCFHP_A>;
///MMC counter Full-Half preset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCFHP_A {
    ///0: When MCP is set, MMC counters are preset to almost-half value 0x7FFF_FFF0
    AlmostHalf = 0,
    ///1: When MCP is set, MMC counters are preset to almost-full value 0xFFFF_FFF0
    AlmostFull = 1,
}
impl From<MCFHP_A> for bool {
    #[inline(always)]
    fn from(variant: MCFHP_A) -> Self {
        variant as u8 != 0
    }
}
impl MCFHP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MCFHP_A {
        match self.bits {
            false => MCFHP_A::AlmostHalf,
            true => MCFHP_A::AlmostFull,
        }
    }
    ///Checks if the value of the field is `AlmostHalf`
    #[inline(always)]
    pub fn is_almost_half(&self) -> bool {
        *self == MCFHP_A::AlmostHalf
    }
    ///Checks if the value of the field is `AlmostFull`
    #[inline(always)]
    pub fn is_almost_full(&self) -> bool {
        *self == MCFHP_A::AlmostFull
    }
}
///Field `MCFHP` writer - MMC counter Full-Half preset
pub type MCFHP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, MCFHP_A, O>;
impl<'a, const O: u8> MCFHP_W<'a, O> {
    ///When MCP is set, MMC counters are preset to almost-half value 0x7FFF_FFF0
    #[inline(always)]
    pub fn almost_half(self) -> &'a mut W {
        self.variant(MCFHP_A::AlmostHalf)
    }
    ///When MCP is set, MMC counters are preset to almost-full value 0xFFFF_FFF0
    #[inline(always)]
    pub fn almost_full(self) -> &'a mut W {
        self.variant(MCFHP_A::AlmostFull)
    }
}
impl R {
    ///Bit 0 - Counter reset
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Counter stop rollover
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Reset on read
    #[inline(always)]
    pub fn ror(&self) -> ROR_R {
        ROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MMC counter freeze
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MMC counter preset
    #[inline(always)]
    pub fn mcp(&self) -> MCP_R {
        MCP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MMC counter Full-Half preset
    #[inline(always)]
    pub fn mcfhp(&self) -> MCFHP_R {
        MCFHP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Counter reset
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<0> {
        CR_W::new(self)
    }
    ///Bit 1 - Counter stop rollover
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<1> {
        CSR_W::new(self)
    }
    ///Bit 2 - Reset on read
    #[inline(always)]
    #[must_use]
    pub fn ror(&mut self) -> ROR_W<2> {
        ROR_W::new(self)
    }
    ///Bit 3 - MMC counter freeze
    #[inline(always)]
    #[must_use]
    pub fn mcf(&mut self) -> MCF_W<3> {
        MCF_W::new(self)
    }
    ///Bit 4 - MMC counter preset
    #[inline(always)]
    #[must_use]
    pub fn mcp(&mut self) -> MCP_W<4> {
        MCP_W::new(self)
    }
    ///Bit 5 - MMC counter Full-Half preset
    #[inline(always)]
    #[must_use]
    pub fn mcfhp(&mut self) -> MCFHP_W<5> {
        MCFHP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MMC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mmccr](index.html) module
pub struct MMCCR_SPEC;
impl crate::RegisterSpec for MMCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mmccr::R](R) reader structure
impl crate::Readable for MMCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mmccr::W](W) writer structure
impl crate::Writable for MMCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MMCCR to value 0
impl crate::Resettable for MMCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
