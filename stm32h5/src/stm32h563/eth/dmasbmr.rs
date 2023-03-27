///Register `DMASBMR` reader
pub struct R(crate::R<DMASBMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASBMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASBMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASBMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMASBMR` writer
pub struct W(crate::W<DMASBMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASBMR_SPEC>;
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
impl From<crate::W<DMASBMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASBMR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FB` reader - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE). When this bit is set to 0, the AHB master will initiate transfers of unspecified length (INCR) or SINGLE transfers.
pub type FB_R = crate::BitReader<bool>;
///Field `FB` writer - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE). When this bit is set to 0, the AHB master will initiate transfers of unspecified length (INCR) or SINGLE transfers.
pub type FB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASBMR_SPEC, bool, O>;
///Field `AAL` reader - Address-Aligned Beats When this bit is set to 1, the master performs address-aligned burst transfers on Read and Write channels.
pub type AAL_R = crate::BitReader<bool>;
///Field `AAL` writer - Address-Aligned Beats When this bit is set to 1, the master performs address-aligned burst transfers on Read and Write channels.
pub type AAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMASBMR_SPEC, bool, O>;
///Field `MB` reader - Mixed Burst When this bit is set high and the FB bit is low, the AHB master performs undefined bursts transfers (INCR) for burst length of 16 or more. For burst length of 16 or less, the AHB master performs fixed burst transfers (INCRx and SINGLE).
pub type MB_R = crate::BitReader<bool>;
///Field `RB` reader - Rebuild INCRx Burst When this bit is set high and the AHB master gets SPLIT, RETRY, or Early Burst Termination (EBT) response, the AHB master interface rebuilds the pending beats of any initiated burst transfer with INCRx and SINGLE transfers. By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified (INCR) burst.
pub type RB_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE). When this bit is set to 0, the AHB master will initiate transfers of unspecified length (INCR) or SINGLE transfers.
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new((self.bits & 1) != 0)
    }
    ///Bit 12 - Address-Aligned Beats When this bit is set to 1, the master performs address-aligned burst transfers on Read and Write channels.
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Mixed Burst When this bit is set high and the FB bit is low, the AHB master performs undefined bursts transfers (INCR) for burst length of 16 or more. For burst length of 16 or less, the AHB master performs fixed burst transfers (INCRx and SINGLE).
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Rebuild INCRx Burst When this bit is set high and the AHB master gets SPLIT, RETRY, or Early Burst Termination (EBT) response, the AHB master interface rebuilds the pending beats of any initiated burst transfer with INCRx and SINGLE transfers. By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified (INCR) burst.
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE). When this bit is set to 0, the AHB master will initiate transfers of unspecified length (INCR) or SINGLE transfers.
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<0> {
        FB_W::new(self)
    }
    ///Bit 12 - Address-Aligned Beats When this bit is set to 1, the master performs address-aligned burst transfers on Read and Write channels.
    #[inline(always)]
    #[must_use]
    pub fn aal(&mut self) -> AAL_W<12> {
        AAL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///System bus mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmasbmr](index.html) module
pub struct DMASBMR_SPEC;
impl crate::RegisterSpec for DMASBMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmasbmr::R](R) reader structure
impl crate::Readable for DMASBMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmasbmr::W](W) writer structure
impl crate::Writable for DMASBMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMASBMR to value 0
impl crate::Resettable for DMASBMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
