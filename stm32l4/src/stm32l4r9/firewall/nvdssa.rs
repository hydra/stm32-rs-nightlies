///Register `NVDSSA` reader
pub struct R(crate::R<NVDSSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVDSSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVDSSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVDSSA_SPEC>) -> Self {
        R(reader)
    }
}
///Register `NVDSSA` writer
pub struct W(crate::W<NVDSSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVDSSA_SPEC>;
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
impl From<crate::W<NVDSSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVDSSA_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADD` reader - Non-volatile data segment start address
pub type ADD_R = crate::FieldReader<u16, u16>;
///Field `ADD` writer - Non-volatile data segment start address
pub type ADD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, NVDSSA_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 8:23 - Non-volatile data segment start address
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 8:23 - Non-volatile data segment start address
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<8> {
        ADD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Non-volatile data segment start address
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [nvdssa](index.html) module
pub struct NVDSSA_SPEC;
impl crate::RegisterSpec for NVDSSA_SPEC {
    type Ux = u32;
}
///`read()` method returns [nvdssa::R](R) reader structure
impl crate::Readable for NVDSSA_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [nvdssa::W](W) writer structure
impl crate::Writable for NVDSSA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets NVDSSA to value 0
impl crate::Resettable for NVDSSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
