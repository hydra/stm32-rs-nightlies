///Register `DCACHE_FCR` writer
pub struct W(crate::W<DCACHE_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_FCR_SPEC>;
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
impl From<crate::W<DCACHE_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_FCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CBSYENDF` writer - CBSYENDF
pub type CBSYENDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_FCR_SPEC, bool, O>;
///Field `CERRF` writer - CERRF
pub type CERRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_FCR_SPEC, bool, O>;
///Field `CCMDENDF` writer - CCMDENDF
pub type CCMDENDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_FCR_SPEC, bool, O>;
impl W {
    ///Bit 1 - CBSYENDF
    #[inline(always)]
    #[must_use]
    pub fn cbsyendf(&mut self) -> CBSYENDF_W<1> {
        CBSYENDF_W::new(self)
    }
    ///Bit 2 - CERRF
    #[inline(always)]
    #[must_use]
    pub fn cerrf(&mut self) -> CERRF_W<2> {
        CERRF_W::new(self)
    }
    ///Bit 4 - CCMDENDF
    #[inline(always)]
    #[must_use]
    pub fn ccmdendf(&mut self) -> CCMDENDF_W<4> {
        CCMDENDF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DCACHE flag clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcache_fcr](index.html) module
pub struct DCACHE_FCR_SPEC;
impl crate::RegisterSpec for DCACHE_FCR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [dcache_fcr::W](W) writer structure
impl crate::Writable for DCACHE_FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DCACHE_FCR to value 0
impl crate::Resettable for DCACHE_FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
