///Register `IFCR` writer
pub struct W(crate::W<IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCR_SPEC>;
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
impl From<crate::W<IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PERRCF` writer - Clears the Parity error flag
pub type PERRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
///Field `OVRCF` writer - Clears the Overrun error flag
pub type OVRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
///Field `SBDCF` writer - Clears the Synchronization Block Detected flag
pub type SBDCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
///Field `SYNCDCF` writer - Clears the Synchronization Done flag
pub type SYNCDCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
impl W {
    ///Bit 2 - Clears the Parity error flag
    #[inline(always)]
    #[must_use]
    pub fn perrcf(&mut self) -> PERRCF_W<2> {
        PERRCF_W::new(self)
    }
    ///Bit 3 - Clears the Overrun error flag
    #[inline(always)]
    #[must_use]
    pub fn ovrcf(&mut self) -> OVRCF_W<3> {
        OVRCF_W::new(self)
    }
    ///Bit 4 - Clears the Synchronization Block Detected flag
    #[inline(always)]
    #[must_use]
    pub fn sbdcf(&mut self) -> SBDCF_W<4> {
        SBDCF_W::new(self)
    }
    ///Bit 5 - Clears the Synchronization Done flag
    #[inline(always)]
    #[must_use]
    pub fn syncdcf(&mut self) -> SYNCDCF_W<5> {
        SYNCDCF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Interrupt Flag Clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ifcr](index.html) module
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [ifcr::W](W) writer structure
impl crate::Writable for IFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
