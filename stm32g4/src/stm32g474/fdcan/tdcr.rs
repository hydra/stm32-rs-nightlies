///Register `TDCR` reader
pub struct R(crate::R<TDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TDCR` writer
pub struct W(crate::W<TDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDCR_SPEC>;
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
impl From<crate::W<TDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TDCF` reader - TDCF
pub type TDCF_R = crate::FieldReader<u8, u8>;
///Field `TDCF` writer - TDCF
pub type TDCF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDCR_SPEC, u8, u8, 7, O>;
///Field `TDCO` reader - TDCO
pub type TDCO_R = crate::FieldReader<u8, u8>;
///Field `TDCO` writer - TDCO
pub type TDCO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDCR_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:6 - TDCF
    #[inline(always)]
    pub fn tdcf(&self) -> TDCF_R {
        TDCF_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:14 - TDCO
    #[inline(always)]
    pub fn tdco(&self) -> TDCO_R {
        TDCO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - TDCF
    #[inline(always)]
    #[must_use]
    pub fn tdcf(&mut self) -> TDCF_W<0> {
        TDCF_W::new(self)
    }
    ///Bits 8:14 - TDCO
    #[inline(always)]
    #[must_use]
    pub fn tdco(&mut self) -> TDCO_W<8> {
        TDCO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Transmitter Delay Compensation Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tdcr](index.html) module
pub struct TDCR_SPEC;
impl crate::RegisterSpec for TDCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tdcr::R](R) reader structure
impl crate::Readable for TDCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tdcr::W](W) writer structure
impl crate::Writable for TDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TDCR to value 0
impl crate::Resettable for TDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
