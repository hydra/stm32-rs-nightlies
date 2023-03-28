///Register `DMATPDR` reader
pub struct R(crate::R<DMATPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATPDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMATPDR` writer
pub struct W(crate::W<DMATPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATPDR_SPEC>;
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
impl From<crate::W<DMATPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATPDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TPD` reader - Transmit poll demand
pub type TPD_R = crate::FieldReader<u32, u32>;
///Field `TPD` writer - Transmit poll demand
pub type TPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMATPDR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Transmit poll demand
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Transmit poll demand
    #[inline(always)]
    #[must_use]
    pub fn tpd(&mut self) -> TPD_W<0> {
        TPD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet DMA transmit poll demand register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmatpdr](index.html) module
pub struct DMATPDR_SPEC;
impl crate::RegisterSpec for DMATPDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmatpdr::R](R) reader structure
impl crate::Readable for DMATPDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmatpdr::W](W) writer structure
impl crate::Writable for DMATPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMATPDR to value 0
impl crate::Resettable for DMATPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
