///Register `ICR` writer
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
///Parity error clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECF_AW {
    ///1: Clears the PE flag in the ISR register
    Clear = 1,
}
impl From<PECF_AW> for bool {
    #[inline(always)]
    fn from(variant: PECF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PECF` writer - Parity error clear flag
pub type PECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, PECF_AW, O>;
impl<'a, const O: u8> PECF_W<'a, O> {
    ///Clears the PE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PECF_AW::Clear)
    }
}
///Framing error clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FECF_AW {
    ///1: Clears the FE flag in the ISR register
    Clear = 1,
}
impl From<FECF_AW> for bool {
    #[inline(always)]
    fn from(variant: FECF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `FECF` writer - Framing error clear flag
pub type FECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, FECF_AW, O>;
impl<'a, const O: u8> FECF_W<'a, O> {
    ///Clears the FE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FECF_AW::Clear)
    }
}
///Noise detected clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCF_AW {
    ///1: Clears the NF flag in the ISR register
    Clear = 1,
}
impl From<NCF_AW> for bool {
    #[inline(always)]
    fn from(variant: NCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `NCF` writer - Noise detected clear flag
pub type NCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, NCF_AW, O>;
impl<'a, const O: u8> NCF_W<'a, O> {
    ///Clears the NF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NCF_AW::Clear)
    }
}
///Overrun error clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORECF_AW {
    ///1: Clears the ORE flag in the ISR register
    Clear = 1,
}
impl From<ORECF_AW> for bool {
    #[inline(always)]
    fn from(variant: ORECF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ORECF` writer - Overrun error clear flag
pub type ORECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, ORECF_AW, O>;
impl<'a, const O: u8> ORECF_W<'a, O> {
    ///Clears the ORE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ORECF_AW::Clear)
    }
}
///Idle line detected clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLECF_AW {
    ///1: Clears the IDLE flag in the ISR register
    Clear = 1,
}
impl From<IDLECF_AW> for bool {
    #[inline(always)]
    fn from(variant: IDLECF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLECF` writer - Idle line detected clear flag
pub type IDLECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, IDLECF_AW, O>;
impl<'a, const O: u8> IDLECF_W<'a, O> {
    ///Clears the IDLE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IDLECF_AW::Clear)
    }
}
///Field `TXFECF` writer - TXFIFO empty clear flag
pub type TXFECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Transmission complete clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCCF_AW {
    ///1: Clears the TC flag in the ISR register
    Clear = 1,
}
impl From<TCCF_AW> for bool {
    #[inline(always)]
    fn from(variant: TCCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `TCCF` writer - Transmission complete clear flag
pub type TCCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, TCCF_AW, O>;
impl<'a, const O: u8> TCCF_W<'a, O> {
    ///Clears the TC flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCCF_AW::Clear)
    }
}
///Field `TCBGTC` writer - Transmission complete before Guard time clear flag
pub type TCBGTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///LIN break detection clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDCF_AW {
    ///1: Clears the LBDF flag in the ISR register
    Clear = 1,
}
impl From<LBDCF_AW> for bool {
    #[inline(always)]
    fn from(variant: LBDCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `LBDCF` writer - LIN break detection clear flag
pub type LBDCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, LBDCF_AW, O>;
impl<'a, const O: u8> LBDCF_W<'a, O> {
    ///Clears the LBDF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LBDCF_AW::Clear)
    }
}
///CTS clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSCF_AW {
    ///1: Clears the CTSIF flag in the ISR register
    Clear = 1,
}
impl From<CTSCF_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSCF` writer - CTS clear flag
pub type CTSCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, CTSCF_AW, O>;
impl<'a, const O: u8> CTSCF_W<'a, O> {
    ///Clears the CTSIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTSCF_AW::Clear)
    }
}
///Receiver timeout clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOCF_AW {
    ///1: Clears the RTOF flag in the ISR register
    Clear = 1,
}
impl From<RTOCF_AW> for bool {
    #[inline(always)]
    fn from(variant: RTOCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `RTOCF` writer - Receiver timeout clear flag
pub type RTOCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, RTOCF_AW, O>;
impl<'a, const O: u8> RTOCF_W<'a, O> {
    ///Clears the RTOF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RTOCF_AW::Clear)
    }
}
///End of block clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBCF_AW {
    ///1: Clears the EOBF flag in the ISR register
    Clear = 1,
}
impl From<EOBCF_AW> for bool {
    #[inline(always)]
    fn from(variant: EOBCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOBCF` writer - End of block clear flag
pub type EOBCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, EOBCF_AW, O>;
impl<'a, const O: u8> EOBCF_W<'a, O> {
    ///Clears the EOBF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOBCF_AW::Clear)
    }
}
///Field `UDRCF` writer - SPI slave underrun clear flag
pub type UDRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Character match clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMCF_AW {
    ///1: Clears the CMF flag in the ISR register
    Clear = 1,
}
impl From<CMCF_AW> for bool {
    #[inline(always)]
    fn from(variant: CMCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CMCF` writer - Character match clear flag
pub type CMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, CMCF_AW, O>;
impl<'a, const O: u8> CMCF_W<'a, O> {
    ///Clears the CMF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMCF_AW::Clear)
    }
}
///Wakeup from Stop mode clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUCF_AW {
    ///1: Clears the WUF flag in the ISR register
    Clear = 1,
}
impl From<WUCF_AW> for bool {
    #[inline(always)]
    fn from(variant: WUCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `WUCF` writer - Wakeup from Stop mode clear flag
pub type WUCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, WUCF_AW, O>;
impl<'a, const O: u8> WUCF_W<'a, O> {
    ///Clears the WUF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WUCF_AW::Clear)
    }
}
impl W {
    ///Bit 0 - Parity error clear flag
    #[inline(always)]
    #[must_use]
    pub fn pecf(&mut self) -> PECF_W<0> {
        PECF_W::new(self)
    }
    ///Bit 1 - Framing error clear flag
    #[inline(always)]
    #[must_use]
    pub fn fecf(&mut self) -> FECF_W<1> {
        FECF_W::new(self)
    }
    ///Bit 2 - Noise detected clear flag
    #[inline(always)]
    #[must_use]
    pub fn ncf(&mut self) -> NCF_W<2> {
        NCF_W::new(self)
    }
    ///Bit 3 - Overrun error clear flag
    #[inline(always)]
    #[must_use]
    pub fn orecf(&mut self) -> ORECF_W<3> {
        ORECF_W::new(self)
    }
    ///Bit 4 - Idle line detected clear flag
    #[inline(always)]
    #[must_use]
    pub fn idlecf(&mut self) -> IDLECF_W<4> {
        IDLECF_W::new(self)
    }
    ///Bit 5 - TXFIFO empty clear flag
    #[inline(always)]
    #[must_use]
    pub fn txfecf(&mut self) -> TXFECF_W<5> {
        TXFECF_W::new(self)
    }
    ///Bit 6 - Transmission complete clear flag
    #[inline(always)]
    #[must_use]
    pub fn tccf(&mut self) -> TCCF_W<6> {
        TCCF_W::new(self)
    }
    ///Bit 7 - Transmission complete before Guard time clear flag
    #[inline(always)]
    #[must_use]
    pub fn tcbgtc(&mut self) -> TCBGTC_W<7> {
        TCBGTC_W::new(self)
    }
    ///Bit 8 - LIN break detection clear flag
    #[inline(always)]
    #[must_use]
    pub fn lbdcf(&mut self) -> LBDCF_W<8> {
        LBDCF_W::new(self)
    }
    ///Bit 9 - CTS clear flag
    #[inline(always)]
    #[must_use]
    pub fn ctscf(&mut self) -> CTSCF_W<9> {
        CTSCF_W::new(self)
    }
    ///Bit 11 - Receiver timeout clear flag
    #[inline(always)]
    #[must_use]
    pub fn rtocf(&mut self) -> RTOCF_W<11> {
        RTOCF_W::new(self)
    }
    ///Bit 12 - End of block clear flag
    #[inline(always)]
    #[must_use]
    pub fn eobcf(&mut self) -> EOBCF_W<12> {
        EOBCF_W::new(self)
    }
    ///Bit 13 - SPI slave underrun clear flag
    #[inline(always)]
    #[must_use]
    pub fn udrcf(&mut self) -> UDRCF_W<13> {
        UDRCF_W::new(self)
    }
    ///Bit 17 - Character match clear flag
    #[inline(always)]
    #[must_use]
    pub fn cmcf(&mut self) -> CMCF_W<17> {
        CMCF_W::new(self)
    }
    ///Bit 20 - Wakeup from Stop mode clear flag
    #[inline(always)]
    #[must_use]
    pub fn wucf(&mut self) -> WUCF_W<20> {
        WUCF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Interrupt flag clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](index.html) module
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [icr::W](W) writer structure
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
