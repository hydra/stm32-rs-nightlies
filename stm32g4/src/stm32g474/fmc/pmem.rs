///Register `PMEM` reader
pub struct R(crate::R<PMEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMEM_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PMEM` writer
pub struct W(crate::W<PMEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMEM_SPEC>;
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
impl From<crate::W<PMEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMEM_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MEMSETx` reader - MEMSETx
pub type MEMSETX_R = crate::FieldReader<u8, u8>;
///Field `MEMSETx` writer - MEMSETx
pub type MEMSETX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM_SPEC, u8, u8, 8, O>;
///Field `MEMWAITx` reader - MEMWAITx
pub type MEMWAITX_R = crate::FieldReader<u8, u8>;
///Field `MEMWAITx` writer - MEMWAITx
pub type MEMWAITX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM_SPEC, u8, u8, 8, O>;
///Field `MEMHOLDx` reader - MEMHOLDx
pub type MEMHOLDX_R = crate::FieldReader<u8, u8>;
///Field `MEMHOLDx` writer - MEMHOLDx
pub type MEMHOLDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM_SPEC, u8, u8, 8, O>;
///Field `MEMHIZx` reader - MEMHIZx
pub type MEMHIZX_R = crate::FieldReader<u8, u8>;
///Field `MEMHIZx` writer - MEMHIZx
pub type MEMHIZX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - MEMSETx
    #[inline(always)]
    pub fn memsetx(&self) -> MEMSETX_R {
        MEMSETX_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - MEMWAITx
    #[inline(always)]
    pub fn memwaitx(&self) -> MEMWAITX_R {
        MEMWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - MEMHOLDx
    #[inline(always)]
    pub fn memholdx(&self) -> MEMHOLDX_R {
        MEMHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - MEMHIZx
    #[inline(always)]
    pub fn memhizx(&self) -> MEMHIZX_R {
        MEMHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - MEMSETx
    #[inline(always)]
    #[must_use]
    pub fn memsetx(&mut self) -> MEMSETX_W<0> {
        MEMSETX_W::new(self)
    }
    ///Bits 8:15 - MEMWAITx
    #[inline(always)]
    #[must_use]
    pub fn memwaitx(&mut self) -> MEMWAITX_W<8> {
        MEMWAITX_W::new(self)
    }
    ///Bits 16:23 - MEMHOLDx
    #[inline(always)]
    #[must_use]
    pub fn memholdx(&mut self) -> MEMHOLDX_W<16> {
        MEMHOLDX_W::new(self)
    }
    ///Bits 24:31 - MEMHIZx
    #[inline(always)]
    #[must_use]
    pub fn memhizx(&mut self) -> MEMHIZX_W<24> {
        MEMHIZX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Common memory space timing register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pmem](index.html) module
pub struct PMEM_SPEC;
impl crate::RegisterSpec for PMEM_SPEC {
    type Ux = u32;
}
///`read()` method returns [pmem::R](R) reader structure
impl crate::Readable for PMEM_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pmem::W](W) writer structure
impl crate::Writable for PMEM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PMEM to value 0xfcfc_fcfc
impl crate::Resettable for PMEM_SPEC {
    const RESET_VALUE: Self::Ux = 0xfcfc_fcfc;
}
