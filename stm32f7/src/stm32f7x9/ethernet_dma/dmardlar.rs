///Register `DMARDLAR` reader
pub struct R(crate::R<DMARDLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMARDLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMARDLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMARDLAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMARDLAR` writer
pub struct W(crate::W<DMARDLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMARDLAR_SPEC>;
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
impl From<crate::W<DMARDLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMARDLAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SRL` reader - Start of receive list
pub type SRL_R = crate::FieldReader<u32, u32>;
///Field `SRL` writer - Start of receive list
pub type SRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMARDLAR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Start of receive list
    #[inline(always)]
    pub fn srl(&self) -> SRL_R {
        SRL_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Start of receive list
    #[inline(always)]
    #[must_use]
    pub fn srl(&mut self) -> SRL_W<0> {
        SRL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet DMA receive descriptor list address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmardlar](index.html) module
pub struct DMARDLAR_SPEC;
impl crate::RegisterSpec for DMARDLAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmardlar::R](R) reader structure
impl crate::Readable for DMARDLAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmardlar::W](W) writer structure
impl crate::Writable for DMARDLAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMARDLAR to value 0
impl crate::Resettable for DMARDLAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
