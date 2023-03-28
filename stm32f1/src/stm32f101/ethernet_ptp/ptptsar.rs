///Register `PTPTSAR` reader
pub struct R(crate::R<PTPTSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTSAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PTPTSAR` writer
pub struct W(crate::W<PTPTSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPTSAR_SPEC>;
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
impl From<crate::W<PTPTSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPTSAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSA` reader - Time stamp addend
pub type TSA_R = crate::FieldReader<u32, u32>;
///Field `TSA` writer - Time stamp addend
pub type TSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTPTSAR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Time stamp addend
    #[inline(always)]
    pub fn tsa(&self) -> TSA_R {
        TSA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Time stamp addend
    #[inline(always)]
    #[must_use]
    pub fn tsa(&mut self) -> TSA_W<0> {
        TSA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet PTP time stamp addend register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ptptsar](index.html) module
pub struct PTPTSAR_SPEC;
impl crate::RegisterSpec for PTPTSAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ptptsar::R](R) reader structure
impl crate::Readable for PTPTSAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ptptsar::W](W) writer structure
impl crate::Writable for PTPTSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PTPTSAR to value 0
impl crate::Resettable for PTPTSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
