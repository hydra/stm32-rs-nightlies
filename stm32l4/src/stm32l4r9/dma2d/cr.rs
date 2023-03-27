///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `START` reader - Start
pub type START_R = crate::BitReader<START_A>;
///Start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    ///1: Launch the DMA2D
    Start = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<START_A> {
        match self.bits {
            true => Some(START_A::Start),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Start`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::Start
    }
}
///Field `START` writer - Start
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, START_A, O>;
impl<'a, const O: u8> START_W<'a, O> {
    ///Launch the DMA2D
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::Start)
    }
}
///Field `SUSP` reader - Suspend
pub type SUSP_R = crate::BitReader<SUSP_A>;
///Suspend
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSP_A {
    ///0: Transfer not suspended
    NotSuspended = 0,
    ///1: Transfer suspended
    Suspended = 1,
}
impl From<SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: SUSP_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SUSP_A {
        match self.bits {
            false => SUSP_A::NotSuspended,
            true => SUSP_A::Suspended,
        }
    }
    ///Checks if the value of the field is `NotSuspended`
    #[inline(always)]
    pub fn is_not_suspended(&self) -> bool {
        *self == SUSP_A::NotSuspended
    }
    ///Checks if the value of the field is `Suspended`
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == SUSP_A::Suspended
    }
}
///Field `SUSP` writer - Suspend
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, SUSP_A, O>;
impl<'a, const O: u8> SUSP_W<'a, O> {
    ///Transfer not suspended
    #[inline(always)]
    pub fn not_suspended(self) -> &'a mut W {
        self.variant(SUSP_A::NotSuspended)
    }
    ///Transfer suspended
    #[inline(always)]
    pub fn suspended(self) -> &'a mut W {
        self.variant(SUSP_A::Suspended)
    }
}
///Field `ABORT` reader - Abort
pub type ABORT_R = crate::BitReader<ABORT_A>;
///Abort
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT_A {
    ///1: Transfer abort requested
    AbortRequest = 1,
}
impl From<ABORT_A> for bool {
    #[inline(always)]
    fn from(variant: ABORT_A) -> Self {
        variant as u8 != 0
    }
}
impl ABORT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<ABORT_A> {
        match self.bits {
            true => Some(ABORT_A::AbortRequest),
            _ => None,
        }
    }
    ///Checks if the value of the field is `AbortRequest`
    #[inline(always)]
    pub fn is_abort_request(&self) -> bool {
        *self == ABORT_A::AbortRequest
    }
}
///Field `ABORT` writer - Abort
pub type ABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ABORT_A, O>;
impl<'a, const O: u8> ABORT_W<'a, O> {
    ///Transfer abort requested
    #[inline(always)]
    pub fn abort_request(self) -> &'a mut W {
        self.variant(ABORT_A::AbortRequest)
    }
}
///Field `TEIE` reader - Transfer error interrupt enable
pub type TEIE_R = crate::BitReader<TEIE_A>;
///Transfer error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE_A {
    ///0: TE interrupt disabled
    Disabled = 0,
    ///1: TE interrupt enabled
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
///Field `TEIE` writer - Transfer error interrupt enable
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TEIE_A, O>;
impl<'a, const O: u8> TEIE_W<'a, O> {
    ///TE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEIE_A::Disabled)
    }
    ///TE interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEIE_A::Enabled)
    }
}
///Field `TCIE` reader - Transfer complete interrupt enable
pub type TCIE_R = crate::BitReader<TCIE_A>;
///Transfer complete interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE_A {
    ///0: TC interrupt disabled
    Disabled = 0,
    ///1: TC interrupt enabled
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
///Field `TCIE` writer - Transfer complete interrupt enable
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TCIE_A, O>;
impl<'a, const O: u8> TCIE_W<'a, O> {
    ///TC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::Disabled)
    }
    ///TC interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::Enabled)
    }
}
///Field `TWIE` reader - Transfer watermark interrupt enable
pub type TWIE_R = crate::BitReader<TWIE_A>;
///Transfer watermark interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWIE_A {
    ///0: TW interrupt disabled
    Disabled = 0,
    ///1: TW interrupt enabled
    Enabled = 1,
}
impl From<TWIE_A> for bool {
    #[inline(always)]
    fn from(variant: TWIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TWIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TWIE_A {
        match self.bits {
            false => TWIE_A::Disabled,
            true => TWIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TWIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TWIE_A::Enabled
    }
}
///Field `TWIE` writer - Transfer watermark interrupt enable
pub type TWIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TWIE_A, O>;
impl<'a, const O: u8> TWIE_W<'a, O> {
    ///TW interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TWIE_A::Disabled)
    }
    ///TW interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TWIE_A::Enabled)
    }
}
///Field `CAEIE` reader - CLUT access error interrupt enable
pub type CAEIE_R = crate::BitReader<CAEIE_A>;
///CLUT access error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAEIE_A {
    ///0: CAE interrupt disabled
    Disabled = 0,
    ///1: CAE interrupt enabled
    Enabled = 1,
}
impl From<CAEIE_A> for bool {
    #[inline(always)]
    fn from(variant: CAEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CAEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CAEIE_A {
        match self.bits {
            false => CAEIE_A::Disabled,
            true => CAEIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAEIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAEIE_A::Enabled
    }
}
///Field `CAEIE` writer - CLUT access error interrupt enable
pub type CAEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CAEIE_A, O>;
impl<'a, const O: u8> CAEIE_W<'a, O> {
    ///CAE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAEIE_A::Disabled)
    }
    ///CAE interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAEIE_A::Enabled)
    }
}
///Field `CTCIE` reader - CLUT transfer complete interrupt enable
pub type CTCIE_R = crate::BitReader<CTCIE_A>;
///CLUT transfer complete interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCIE_A {
    ///0: CTC interrupt disabled
    Disabled = 0,
    ///1: CTC interrupt enabled
    Enabled = 1,
}
impl From<CTCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CTCIE_A {
        match self.bits {
            false => CTCIE_A::Disabled,
            true => CTCIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTCIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTCIE_A::Enabled
    }
}
///Field `CTCIE` writer - CLUT transfer complete interrupt enable
pub type CTCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CTCIE_A, O>;
impl<'a, const O: u8> CTCIE_W<'a, O> {
    ///CTC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTCIE_A::Disabled)
    }
    ///CTC interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTCIE_A::Enabled)
    }
}
///Field `CEIE` reader - Configuration Error Interrupt Enable
pub type CEIE_R = crate::BitReader<CEIE_A>;
///Configuration Error Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEIE_A {
    ///0: CE interrupt disabled
    Disabled = 0,
    ///1: CE interrupt enabled
    Enabled = 1,
}
impl From<CEIE_A> for bool {
    #[inline(always)]
    fn from(variant: CEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CEIE_A {
        match self.bits {
            false => CEIE_A::Disabled,
            true => CEIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEIE_A::Enabled
    }
}
///Field `CEIE` writer - Configuration Error Interrupt Enable
pub type CEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CEIE_A, O>;
impl<'a, const O: u8> CEIE_W<'a, O> {
    ///CE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEIE_A::Disabled)
    }
    ///CE interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEIE_A::Enabled)
    }
}
///Field `MODE` reader - DMA2D mode
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
///DMA2D mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    ///0: Memory-to-memory (FG fetch only)
    MemoryToMemory = 0,
    ///1: Memory-to-memory with PFC (FG fetch only with FG PFC active)
    MemoryToMemoryPfc = 1,
    ///2: Memory-to-memory with blending (FG and BG fetch with PFC and blending)
    MemoryToMemoryPfcblending = 2,
    ///3: Register-to-memory
    RegisterToMemory = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::MemoryToMemory,
            1 => MODE_A::MemoryToMemoryPfc,
            2 => MODE_A::MemoryToMemoryPfcblending,
            3 => MODE_A::RegisterToMemory,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `MemoryToMemory`
    #[inline(always)]
    pub fn is_memory_to_memory(&self) -> bool {
        *self == MODE_A::MemoryToMemory
    }
    ///Checks if the value of the field is `MemoryToMemoryPfc`
    #[inline(always)]
    pub fn is_memory_to_memory_pfc(&self) -> bool {
        *self == MODE_A::MemoryToMemoryPfc
    }
    ///Checks if the value of the field is `MemoryToMemoryPfcblending`
    #[inline(always)]
    pub fn is_memory_to_memory_pfcblending(&self) -> bool {
        *self == MODE_A::MemoryToMemoryPfcblending
    }
    ///Checks if the value of the field is `RegisterToMemory`
    #[inline(always)]
    pub fn is_register_to_memory(&self) -> bool {
        *self == MODE_A::RegisterToMemory
    }
}
///Field `MODE` writer - DMA2D mode
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    ///Memory-to-memory (FG fetch only)
    #[inline(always)]
    pub fn memory_to_memory(self) -> &'a mut W {
        self.variant(MODE_A::MemoryToMemory)
    }
    ///Memory-to-memory with PFC (FG fetch only with FG PFC active)
    #[inline(always)]
    pub fn memory_to_memory_pfc(self) -> &'a mut W {
        self.variant(MODE_A::MemoryToMemoryPfc)
    }
    ///Memory-to-memory with blending (FG and BG fetch with PFC and blending)
    #[inline(always)]
    pub fn memory_to_memory_pfcblending(self) -> &'a mut W {
        self.variant(MODE_A::MemoryToMemoryPfcblending)
    }
    ///Register-to-memory
    #[inline(always)]
    pub fn register_to_memory(self) -> &'a mut W {
        self.variant(MODE_A::RegisterToMemory)
    }
}
impl R {
    ///Bit 0 - Start
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Suspend
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Abort
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Transfer watermark interrupt enable
    #[inline(always)]
    pub fn twie(&self) -> TWIE_R {
        TWIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CLUT access error interrupt enable
    #[inline(always)]
    pub fn caeie(&self) -> CAEIE_R {
        CAEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CLUT transfer complete interrupt enable
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Configuration Error Interrupt Enable
    #[inline(always)]
    pub fn ceie(&self) -> CEIE_R {
        CEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 16:17 - DMA2D mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - Start
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    ///Bit 1 - Suspend
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<1> {
        SUSP_W::new(self)
    }
    ///Bit 2 - Abort
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<2> {
        ABORT_W::new(self)
    }
    ///Bit 8 - Transfer error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<8> {
        TEIE_W::new(self)
    }
    ///Bit 9 - Transfer complete interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<9> {
        TCIE_W::new(self)
    }
    ///Bit 10 - Transfer watermark interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn twie(&mut self) -> TWIE_W<10> {
        TWIE_W::new(self)
    }
    ///Bit 11 - CLUT access error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn caeie(&mut self) -> CAEIE_W<11> {
        CAEIE_W::new(self)
    }
    ///Bit 12 - CLUT transfer complete interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ctcie(&mut self) -> CTCIE_W<12> {
        CTCIE_W::new(self)
    }
    ///Bit 13 - Configuration Error Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn ceie(&mut self) -> CEIE_W<13> {
        CEIE_W::new(self)
    }
    ///Bits 16:17 - DMA2D mode
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<16> {
        MODE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
