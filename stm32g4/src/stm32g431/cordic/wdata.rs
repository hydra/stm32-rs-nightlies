///Register `WDATA` reader
pub struct R(crate::R<WDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDATA_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WDATA` writer
pub struct W(crate::W<WDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDATA_SPEC>;
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
impl From<crate::W<WDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDATA_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ARG` reader - ARG
pub type ARG_R = crate::FieldReader<u32, u32>;
///Field `ARG` writer - ARG
pub type ARG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, WDATA_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - ARG
    #[inline(always)]
    pub fn arg(&self) -> ARG_R {
        ARG_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - ARG
    #[inline(always)]
    #[must_use]
    pub fn arg(&mut self) -> ARG_W<0> {
        ARG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///CORDIC argument register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wdata](index.html) module
pub struct WDATA_SPEC;
impl crate::RegisterSpec for WDATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [wdata::R](R) reader structure
impl crate::Readable for WDATA_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [wdata::W](W) writer structure
impl crate::Writable for WDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WDATA to value 0
impl crate::Resettable for WDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
