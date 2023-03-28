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
///Field `PECF` writer - Parity error clear flag
pub type PECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `FECF` writer - Framing error clear flag
pub type FECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `NCEF` writer - Noise detected clear flag
pub type NCEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `ORECF` writer - Overrun error clear flag
pub type ORECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `IDLECF` writer - Idle line detected clear flag
pub type IDLECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `TCCF` writer - Transmission complete clear flag
pub type TCCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `CTSCF` writer - CTS clear flag
pub type CTSCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `CMCF` writer - Character match clear flag
pub type CMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `WUCF` writer - Wakeup from Stop mode clear flag
pub type WUCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
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
    pub fn ncef(&mut self) -> NCEF_W<2> {
        NCEF_W::new(self)
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
    ///Bit 6 - Transmission complete clear flag
    #[inline(always)]
    #[must_use]
    pub fn tccf(&mut self) -> TCCF_W<6> {
        TCCF_W::new(self)
    }
    ///Bit 9 - CTS clear flag
    #[inline(always)]
    #[must_use]
    pub fn ctscf(&mut self) -> CTSCF_W<9> {
        CTSCF_W::new(self)
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
