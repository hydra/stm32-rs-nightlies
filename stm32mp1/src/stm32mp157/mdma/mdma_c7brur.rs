///Register `MDMA_C7BRUR` reader
pub struct R(crate::R<MDMA_C7BRUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C7BRUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C7BRUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C7BRUR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDMA_C7BRUR` writer
pub struct W(crate::W<MDMA_C7BRUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMA_C7BRUR_SPEC>;
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
impl From<crate::W<MDMA_C7BRUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMA_C7BRUR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SUV` reader - SUV
pub type SUV_R = crate::FieldReader<u16, u16>;
///Field `SUV` writer - SUV
pub type SUV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C7BRUR_SPEC, u16, u16, 16, O>;
///Field `DUV` reader - DUV
pub type DUV_R = crate::FieldReader<u16, u16>;
///Field `DUV` writer - DUV
pub type DUV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C7BRUR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - SUV
    #[inline(always)]
    pub fn suv(&self) -> SUV_R {
        SUV_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - DUV
    #[inline(always)]
    pub fn duv(&self) -> DUV_R {
        DUV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - SUV
    #[inline(always)]
    #[must_use]
    pub fn suv(&mut self) -> SUV_W<0> {
        SUV_W::new(self)
    }
    ///Bits 16:31 - DUV
    #[inline(always)]
    #[must_use]
    pub fn duv(&mut self) -> DUV_W<16> {
        DUV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
///+ 0x10).
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdma_c7brur](index.html) module
pub struct MDMA_C7BRUR_SPEC;
impl crate::RegisterSpec for MDMA_C7BRUR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdma_c7brur::R](R) reader structure
impl crate::Readable for MDMA_C7BRUR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdma_c7brur::W](W) writer structure
impl crate::Writable for MDMA_C7BRUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDMA_C7BRUR to value 0
impl crate::Resettable for MDMA_C7BRUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
