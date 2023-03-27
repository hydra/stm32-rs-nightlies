///Register `ICR` reader
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
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
///Field `CLRJOVRF` reader - Clear the injected conversion overrun flag
pub type CLRJOVRF_R = crate::BitReader<bool>;
///Field `CLRJOVRF` writer - Clear the injected conversion overrun flag
pub type CLRJOVRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `CLRROVRF` reader - Clear the regular conversion overrun flag
pub type CLRROVRF_R = crate::BitReader<bool>;
///Field `CLRROVRF` writer - Clear the regular conversion overrun flag
pub type CLRROVRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `CLRCKABF` reader - Clear the clock absence flag CLRCKABF\[y\]=0: Writing '0â has no effect CLRCKABF\[y\]=1: Writing '1â to position y clears the corresponding CKABF\[y\]
///bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\[y\]. Note: CLRCKABF\[7:0\]
///is present only in DFSDM_FLT0ICR register (filter x=0)
pub type CLRCKABF_R = crate::FieldReader<u8, u8>;
///Field `CLRCKABF` writer - Clear the clock absence flag CLRCKABF\[y\]=0: Writing '0â has no effect CLRCKABF\[y\]=1: Writing '1â to position y clears the corresponding CKABF\[y\]
///bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\[y\]. Note: CLRCKABF\[7:0\]
///is present only in DFSDM_FLT0ICR register (filter x=0)
pub type CLRCKABF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICR_SPEC, u8, u8, 8, O>;
///Field `CLRSCDF` reader - Clear the short-circuit detector flag CLRSCDF\[y\]=0: Writing '0â has no effect CLRSCDF\[y\]=1: Writing '1â to position y clears the corresponding SCDF\[y\]
///bit in the DFSDM_FLTxISR register Note: CLRSCDF\[7:0\]
///is present only in DFSDM_FLT0ICR register (filter x=0)
pub type CLRSCDF_R = crate::FieldReader<u8, u8>;
///Field `CLRSCDF` writer - Clear the short-circuit detector flag CLRSCDF\[y\]=0: Writing '0â has no effect CLRSCDF\[y\]=1: Writing '1â to position y clears the corresponding SCDF\[y\]
///bit in the DFSDM_FLTxISR register Note: CLRSCDF\[7:0\]
///is present only in DFSDM_FLT0ICR register (filter x=0)
pub type CLRSCDF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bit 2 - Clear the injected conversion overrun flag
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear the regular conversion overrun flag
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 16:23 - Clear the clock absence flag CLRCKABF\[y\]=0: Writing '0â has no effect CLRCKABF\[y\]=1: Writing '1â to position y clears the corresponding CKABF\[y\]
    ///bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\[y\]. Note: CLRCKABF\[7:0\]
    ///is present only in DFSDM_FLT0ICR register (filter x=0)
    #[inline(always)]
    pub fn clrckabf(&self) -> CLRCKABF_R {
        CLRCKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Clear the short-circuit detector flag CLRSCDF\[y\]=0: Writing '0â has no effect CLRSCDF\[y\]=1: Writing '1â to position y clears the corresponding SCDF\[y\]
    ///bit in the DFSDM_FLTxISR register Note: CLRSCDF\[7:0\]
    ///is present only in DFSDM_FLT0ICR register (filter x=0)
    #[inline(always)]
    pub fn clrscdf(&self) -> CLRSCDF_R {
        CLRSCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bit 2 - Clear the injected conversion overrun flag
    #[inline(always)]
    #[must_use]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W<2> {
        CLRJOVRF_W::new(self)
    }
    ///Bit 3 - Clear the regular conversion overrun flag
    #[inline(always)]
    #[must_use]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W<3> {
        CLRROVRF_W::new(self)
    }
    ///Bits 16:23 - Clear the clock absence flag CLRCKABF\[y\]=0: Writing '0â has no effect CLRCKABF\[y\]=1: Writing '1â to position y clears the corresponding CKABF\[y\]
    ///bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\[y\]. Note: CLRCKABF\[7:0\]
    ///is present only in DFSDM_FLT0ICR register (filter x=0)
    #[inline(always)]
    #[must_use]
    pub fn clrckabf(&mut self) -> CLRCKABF_W<16> {
        CLRCKABF_W::new(self)
    }
    ///Bits 24:31 - Clear the short-circuit detector flag CLRSCDF\[y\]=0: Writing '0â has no effect CLRSCDF\[y\]=1: Writing '1â to position y clears the corresponding SCDF\[y\]
    ///bit in the DFSDM_FLTxISR register Note: CLRSCDF\[7:0\]
    ///is present only in DFSDM_FLT0ICR register (filter x=0)
    #[inline(always)]
    #[must_use]
    pub fn clrscdf(&mut self) -> CLRSCDF_W<24> {
        CLRSCDF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](index.html) module
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [icr::R](R) reader structure
impl crate::Readable for ICR_SPEC {
    type Reader = R;
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
