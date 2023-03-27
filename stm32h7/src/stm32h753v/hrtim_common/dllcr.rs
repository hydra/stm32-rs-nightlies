///Register `DLLCR` reader
pub struct R(crate::R<DLLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLLCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DLLCR` writer
pub struct W(crate::W<DLLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLLCR_SPEC>;
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
impl From<crate::W<DLLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLLCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CAL` reader - DLL Calibration Start
pub type CAL_R = crate::BitReader<bool>;
///Field `CAL` writer - DLL Calibration Start
pub type CAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLLCR_SPEC, bool, O>;
///Field `CALEN` reader - DLL Calibration Enable
pub type CALEN_R = crate::BitReader<bool>;
///Field `CALEN` writer - DLL Calibration Enable
pub type CALEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLLCR_SPEC, bool, O>;
///Field `CALRTE` reader - DLL Calibration rate
pub type CALRTE_R = crate::FieldReader<u8, u8>;
///Field `CALRTE` writer - DLL Calibration rate
pub type CALRTE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLLCR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - DLL Calibration Start
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DLL Calibration Enable
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - DLL Calibration rate
    #[inline(always)]
    pub fn calrte(&self) -> CALRTE_R {
        CALRTE_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - DLL Calibration Start
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<0> {
        CAL_W::new(self)
    }
    ///Bit 1 - DLL Calibration Enable
    #[inline(always)]
    #[must_use]
    pub fn calen(&mut self) -> CALEN_W<1> {
        CALEN_W::new(self)
    }
    ///Bits 2:3 - DLL Calibration rate
    #[inline(always)]
    #[must_use]
    pub fn calrte(&mut self) -> CALRTE_W<2> {
        CALRTE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DLL Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dllcr](index.html) module
pub struct DLLCR_SPEC;
impl crate::RegisterSpec for DLLCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dllcr::R](R) reader structure
impl crate::Readable for DLLCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dllcr::W](W) writer structure
impl crate::Writable for DLLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DLLCR to value 0
impl crate::Resettable for DLLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
