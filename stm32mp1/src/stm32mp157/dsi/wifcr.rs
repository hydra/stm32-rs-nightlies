///Register `WIFCR` writer
pub struct W(crate::W<WIFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFCR_SPEC>;
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
impl From<crate::W<WIFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CTEIF` writer - CTEIF
pub type CTEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFCR_SPEC, bool, O>;
///Field `CERIF` writer - CERIF
pub type CERIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFCR_SPEC, bool, O>;
///Field `CPLLLIF` writer - CPLLLIF
pub type CPLLLIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFCR_SPEC, bool, O>;
///Field `CPLLUIF` writer - CPLLUIF
pub type CPLLUIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFCR_SPEC, bool, O>;
///Field `CRRIF` writer - CRRIF
pub type CRRIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFCR_SPEC, bool, O>;
impl W {
    ///Bit 0 - CTEIF
    #[inline(always)]
    #[must_use]
    pub fn cteif(&mut self) -> CTEIF_W<0> {
        CTEIF_W::new(self)
    }
    ///Bit 1 - CERIF
    #[inline(always)]
    #[must_use]
    pub fn cerif(&mut self) -> CERIF_W<1> {
        CERIF_W::new(self)
    }
    ///Bit 9 - CPLLLIF
    #[inline(always)]
    #[must_use]
    pub fn cplllif(&mut self) -> CPLLLIF_W<9> {
        CPLLLIF_W::new(self)
    }
    ///Bit 10 - CPLLUIF
    #[inline(always)]
    #[must_use]
    pub fn cplluif(&mut self) -> CPLLUIF_W<10> {
        CPLLUIF_W::new(self)
    }
    ///Bit 13 - CRRIF
    #[inline(always)]
    #[must_use]
    pub fn crrif(&mut self) -> CRRIF_W<13> {
        CRRIF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI wrapper interrupt flag clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wifcr](index.html) module
pub struct WIFCR_SPEC;
impl crate::RegisterSpec for WIFCR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [wifcr::W](W) writer structure
impl crate::Writable for WIFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WIFCR to value 0
impl crate::Resettable for WIFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
