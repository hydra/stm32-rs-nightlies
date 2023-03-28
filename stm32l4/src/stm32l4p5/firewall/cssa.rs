///Register `CSSA` reader
pub struct R(crate::R<CSSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSSA_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSSA` writer
pub struct W(crate::W<CSSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSSA_SPEC>;
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
impl From<crate::W<CSSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSSA_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADD` reader - code segment start address
pub type ADD_R = crate::FieldReader<u16, u16>;
///Field `ADD` writer - code segment start address
pub type ADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSSA_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 8:23 - code segment start address
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 8:23 - code segment start address
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
///Code segment start address
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cssa](index.html) module
pub struct CSSA_SPEC;
impl crate::RegisterSpec for CSSA_SPEC {
    type Ux = u32;
}
///`read()` method returns [cssa::R](R) reader structure
impl crate::Readable for CSSA_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cssa::W](W) writer structure
impl crate::Writable for CSSA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSSA to value 0
impl crate::Resettable for CSSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
