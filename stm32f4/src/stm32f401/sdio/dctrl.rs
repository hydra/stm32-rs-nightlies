///Register `DCTRL` reader
pub struct R(crate::R<DCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DCTRL` writer
pub struct W(crate::W<DCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCTRL_SPEC>;
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
impl From<crate::W<DCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DTEN` reader - DTEN
pub type DTEN_R = crate::BitReader<DTEN_A>;
///DTEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEN_A {
    ///0: Disabled
    Disabled = 0,
    ///1: Start transfer
    Enabled = 1,
}
impl From<DTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DTEN_A {
        match self.bits {
            false => DTEN_A::Disabled,
            true => DTEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTEN_A::Enabled
    }
}
///Field `DTEN` writer - DTEN
pub type DTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, DTEN_A, O>;
impl<'a, const O: u8> DTEN_W<'a, O> {
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DTEN_A::Disabled)
    }
    ///Start transfer
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DTEN_A::Enabled)
    }
}
///Field `DTDIR` reader - Data transfer direction selection
pub type DTDIR_R = crate::BitReader<DTDIR_A>;
///Data transfer direction selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTDIR_A {
    ///0: From controller to card
    ControllerToCard = 0,
    ///1: From card to controller
    CardToController = 1,
}
impl From<DTDIR_A> for bool {
    #[inline(always)]
    fn from(variant: DTDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DTDIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DTDIR_A {
        match self.bits {
            false => DTDIR_A::ControllerToCard,
            true => DTDIR_A::CardToController,
        }
    }
    ///Checks if the value of the field is `ControllerToCard`
    #[inline(always)]
    pub fn is_controller_to_card(&self) -> bool {
        *self == DTDIR_A::ControllerToCard
    }
    ///Checks if the value of the field is `CardToController`
    #[inline(always)]
    pub fn is_card_to_controller(&self) -> bool {
        *self == DTDIR_A::CardToController
    }
}
///Field `DTDIR` writer - Data transfer direction selection
pub type DTDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, DTDIR_A, O>;
impl<'a, const O: u8> DTDIR_W<'a, O> {
    ///From controller to card
    #[inline(always)]
    pub fn controller_to_card(self) -> &'a mut W {
        self.variant(DTDIR_A::ControllerToCard)
    }
    ///From card to controller
    #[inline(always)]
    pub fn card_to_controller(self) -> &'a mut W {
        self.variant(DTDIR_A::CardToController)
    }
}
///Field `DTMODE` reader - Data transfer mode selection 1: Stream or SDIO multibyte data transfer.
pub type DTMODE_R = crate::BitReader<DTMODE_A>;
///Data transfer mode selection 1: Stream or SDIO multibyte data transfer.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTMODE_A {
    ///0: Bloack data transfer
    BlockMode = 0,
    ///1: Stream or SDIO multibyte data transfer
    StreamMode = 1,
}
impl From<DTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DTMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DTMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DTMODE_A {
        match self.bits {
            false => DTMODE_A::BlockMode,
            true => DTMODE_A::StreamMode,
        }
    }
    ///Checks if the value of the field is `BlockMode`
    #[inline(always)]
    pub fn is_block_mode(&self) -> bool {
        *self == DTMODE_A::BlockMode
    }
    ///Checks if the value of the field is `StreamMode`
    #[inline(always)]
    pub fn is_stream_mode(&self) -> bool {
        *self == DTMODE_A::StreamMode
    }
}
///Field `DTMODE` writer - Data transfer mode selection 1: Stream or SDIO multibyte data transfer.
pub type DTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, DTMODE_A, O>;
impl<'a, const O: u8> DTMODE_W<'a, O> {
    ///Bloack data transfer
    #[inline(always)]
    pub fn block_mode(self) -> &'a mut W {
        self.variant(DTMODE_A::BlockMode)
    }
    ///Stream or SDIO multibyte data transfer
    #[inline(always)]
    pub fn stream_mode(self) -> &'a mut W {
        self.variant(DTMODE_A::StreamMode)
    }
}
///Field `DMAEN` reader - DMA enable bit
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
///DMA enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    ///0: Dma disabled
    Disabled = 0,
    ///1: Dma enabled
    Enabled = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::Disabled,
            true => DMAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::Enabled
    }
}
///Field `DMAEN` writer - DMA enable bit
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    ///Dma disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Disabled)
    }
    ///Dma enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Enabled)
    }
}
///Field `DBLOCKSIZE` reader - Data block size
pub type DBLOCKSIZE_R = crate::FieldReader<u8, u8>;
///Field `DBLOCKSIZE` writer - Data block size
pub type DBLOCKSIZE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DCTRL_SPEC, u8, u8, 4, O>;
///Field `RWSTART` reader - Read wait start
pub type RWSTART_R = crate::BitReader<RWSTART_A>;
///Read wait start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWSTART_A {
    ///0: Don't start read wait operation
    Disabled = 0,
    ///1: Read wait operation starts
    Enabled = 1,
}
impl From<RWSTART_A> for bool {
    #[inline(always)]
    fn from(variant: RWSTART_A) -> Self {
        variant as u8 != 0
    }
}
impl RWSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RWSTART_A {
        match self.bits {
            false => RWSTART_A::Disabled,
            true => RWSTART_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RWSTART_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RWSTART_A::Enabled
    }
}
///Field `RWSTART` writer - Read wait start
pub type RWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, RWSTART_A, O>;
impl<'a, const O: u8> RWSTART_W<'a, O> {
    ///Don't start read wait operation
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RWSTART_A::Disabled)
    }
    ///Read wait operation starts
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RWSTART_A::Enabled)
    }
}
///Field `RWSTOP` reader - Read wait stop
pub type RWSTOP_R = crate::BitReader<RWSTOP_A>;
///Read wait stop
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWSTOP_A {
    ///0: Read wait in progress if RWSTART is enabled
    Disabled = 0,
    ///1: Enable for read wait stop if RWSTART is enabled
    Enabled = 1,
}
impl From<RWSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: RWSTOP_A) -> Self {
        variant as u8 != 0
    }
}
impl RWSTOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RWSTOP_A {
        match self.bits {
            false => RWSTOP_A::Disabled,
            true => RWSTOP_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RWSTOP_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RWSTOP_A::Enabled
    }
}
///Field `RWSTOP` writer - Read wait stop
pub type RWSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, RWSTOP_A, O>;
impl<'a, const O: u8> RWSTOP_W<'a, O> {
    ///Read wait in progress if RWSTART is enabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RWSTOP_A::Disabled)
    }
    ///Enable for read wait stop if RWSTART is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RWSTOP_A::Enabled)
    }
}
///Field `RWMOD` reader - Read wait mode
pub type RWMOD_R = crate::BitReader<RWMOD_A>;
///Read wait mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWMOD_A {
    ///0: Read wait control stopping using SDIO_D2
    D2 = 0,
    ///1: Read wait control using SDIO_CK
    Ck = 1,
}
impl From<RWMOD_A> for bool {
    #[inline(always)]
    fn from(variant: RWMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl RWMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RWMOD_A {
        match self.bits {
            false => RWMOD_A::D2,
            true => RWMOD_A::Ck,
        }
    }
    ///Checks if the value of the field is `D2`
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == RWMOD_A::D2
    }
    ///Checks if the value of the field is `Ck`
    #[inline(always)]
    pub fn is_ck(&self) -> bool {
        *self == RWMOD_A::Ck
    }
}
///Field `RWMOD` writer - Read wait mode
pub type RWMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, RWMOD_A, O>;
impl<'a, const O: u8> RWMOD_W<'a, O> {
    ///Read wait control stopping using SDIO_D2
    #[inline(always)]
    pub fn d2(self) -> &'a mut W {
        self.variant(RWMOD_A::D2)
    }
    ///Read wait control using SDIO_CK
    #[inline(always)]
    pub fn ck(self) -> &'a mut W {
        self.variant(RWMOD_A::Ck)
    }
}
///Field `SDIOEN` reader - SD I/O enable functions
pub type SDIOEN_R = crate::BitReader<SDIOEN_A>;
///SD I/O enable functions
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIOEN_A {
    ///0: SDIO operations disabled
    Disabled = 0,
    ///1: SDIO operations enabled
    Enabled = 1,
}
impl From<SDIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDIOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SDIOEN_A {
        match self.bits {
            false => SDIOEN_A::Disabled,
            true => SDIOEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDIOEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDIOEN_A::Enabled
    }
}
///Field `SDIOEN` writer - SD I/O enable functions
pub type SDIOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, SDIOEN_A, O>;
impl<'a, const O: u8> SDIOEN_W<'a, O> {
    ///SDIO operations disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDIOEN_A::Disabled)
    }
    ///SDIO operations enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDIOEN_A::Enabled)
    }
}
impl R {
    ///Bit 0 - DTEN
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data transfer direction selection
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer.
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DMA enable bit
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - Data block size
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Read wait start
    #[inline(always)]
    pub fn rwstart(&self) -> RWSTART_R {
        RWSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Read wait stop
    #[inline(always)]
    pub fn rwstop(&self) -> RWSTOP_R {
        RWSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Read wait mode
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SD I/O enable functions
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DTEN
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<0> {
        DTEN_W::new(self)
    }
    ///Bit 1 - Data transfer direction selection
    #[inline(always)]
    #[must_use]
    pub fn dtdir(&mut self) -> DTDIR_W<1> {
        DTDIR_W::new(self)
    }
    ///Bit 2 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer.
    #[inline(always)]
    #[must_use]
    pub fn dtmode(&mut self) -> DTMODE_W<2> {
        DTMODE_W::new(self)
    }
    ///Bit 3 - DMA enable bit
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<3> {
        DMAEN_W::new(self)
    }
    ///Bits 4:7 - Data block size
    #[inline(always)]
    #[must_use]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W<4> {
        DBLOCKSIZE_W::new(self)
    }
    ///Bit 8 - Read wait start
    #[inline(always)]
    #[must_use]
    pub fn rwstart(&mut self) -> RWSTART_W<8> {
        RWSTART_W::new(self)
    }
    ///Bit 9 - Read wait stop
    #[inline(always)]
    #[must_use]
    pub fn rwstop(&mut self) -> RWSTOP_W<9> {
        RWSTOP_W::new(self)
    }
    ///Bit 10 - Read wait mode
    #[inline(always)]
    #[must_use]
    pub fn rwmod(&mut self) -> RWMOD_W<10> {
        RWMOD_W::new(self)
    }
    ///Bit 11 - SD I/O enable functions
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SDIOEN_W<11> {
        SDIOEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///data control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dctrl](index.html) module
pub struct DCTRL_SPEC;
impl crate::RegisterSpec for DCTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [dctrl::R](R) reader structure
impl crate::Readable for DCTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dctrl::W](W) writer structure
impl crate::Writable for DCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DCTRL to value 0
impl crate::Resettable for DCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
