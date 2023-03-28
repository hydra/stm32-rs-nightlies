///Register `RQR` writer
pub struct W(crate::W<RQR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RQR_SPEC>;
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
impl From<crate::W<RQR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RQR_SPEC>) -> Self {
        W(writer)
    }
}
///Send break request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBKRQ_AW {
    ///1: sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available
    Break = 1,
}
impl From<SBKRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: SBKRQ_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `SBKRQ` writer - Send break request
pub type SBKRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, SBKRQ_AW, O>;
impl<'a, const O: u8> SBKRQ_W<'a, O> {
    ///sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available
    #[inline(always)]
    pub fn break_(self) -> &'a mut W {
        self.variant(SBKRQ_AW::Break)
    }
}
///Mute mode request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMRQ_AW {
    ///1: Puts the USART in mute mode and sets the RWU flag
    Mute = 1,
}
impl From<MMRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: MMRQ_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `MMRQ` writer - Mute mode request
pub type MMRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, MMRQ_AW, O>;
impl<'a, const O: u8> MMRQ_W<'a, O> {
    ///Puts the USART in mute mode and sets the RWU flag
    #[inline(always)]
    pub fn mute(self) -> &'a mut W {
        self.variant(MMRQ_AW::Mute)
    }
}
///Receive data flush request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFRQ_AW {
    ///1: clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition
    Discard = 1,
}
impl From<RXFRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFRQ_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFRQ` writer - Receive data flush request
pub type RXFRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, RXFRQ_AW, O>;
impl<'a, const O: u8> RXFRQ_W<'a, O> {
    ///clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition
    #[inline(always)]
    pub fn discard(self) -> &'a mut W {
        self.variant(RXFRQ_AW::Discard)
    }
}
impl W {
    ///Bit 1 - Send break request
    #[inline(always)]
    #[must_use]
    pub fn sbkrq(&mut self) -> SBKRQ_W<1> {
        SBKRQ_W::new(self)
    }
    ///Bit 2 - Mute mode request
    #[inline(always)]
    #[must_use]
    pub fn mmrq(&mut self) -> MMRQ_W<2> {
        MMRQ_W::new(self)
    }
    ///Bit 3 - Receive data flush request
    #[inline(always)]
    #[must_use]
    pub fn rxfrq(&mut self) -> RXFRQ_W<3> {
        RXFRQ_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Request register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rqr](index.html) module
pub struct RQR_SPEC;
impl crate::RegisterSpec for RQR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [rqr::W](W) writer structure
impl crate::Writable for RQR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RQR to value 0
impl crate::Resettable for RQR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
