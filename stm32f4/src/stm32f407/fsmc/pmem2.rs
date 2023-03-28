///Register `PMEM2` reader
pub struct R(crate::R<PMEM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMEM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMEM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMEM2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PMEM2` writer
pub struct W(crate::W<PMEM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMEM2_SPEC>;
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
impl From<crate::W<PMEM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMEM2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MEMSET` reader - MEMSETx
pub type MEMSET_R = crate::FieldReader<u8, u8>;
///Field `MEMSET` writer - MEMSETx
pub type MEMSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM2_SPEC, u8, u8, 8, O>;
///Field `MEMWAIT` reader - MEMWAITx
pub type MEMWAIT_R = crate::FieldReader<u8, u8>;
///Field `MEMWAIT` writer - MEMWAITx
pub type MEMWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM2_SPEC, u8, u8, 8, O>;
///Field `MEMHOLD` reader - MEMHOLDx
pub type MEMHOLD_R = crate::FieldReader<u8, u8>;
///Field `MEMHOLD` writer - MEMHOLDx
pub type MEMHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM2_SPEC, u8, u8, 8, O>;
///Field `MEMHIZ` reader - MEMHIZx
pub type MEMHIZ_R = crate::FieldReader<u8, u8>;
///Field `MEMHIZ` writer - MEMHIZx
pub type MEMHIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM2_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - MEMSETx
    #[inline(always)]
    pub fn memset(&self) -> MEMSET_R {
        MEMSET_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - MEMWAITx
    #[inline(always)]
    pub fn memwait(&self) -> MEMWAIT_R {
        MEMWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - MEMHOLDx
    #[inline(always)]
    pub fn memhold(&self) -> MEMHOLD_R {
        MEMHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - MEMHIZx
    #[inline(always)]
    pub fn memhiz(&self) -> MEMHIZ_R {
        MEMHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - MEMSETx
    #[inline(always)]
    #[must_use]
    pub fn memset(&mut self) -> MEMSET_W<0> {
        MEMSET_W::new(self)
    }
    ///Bits 8:15 - MEMWAITx
    #[inline(always)]
    #[must_use]
    pub fn memwait(&mut self) -> MEMWAIT_W<8> {
        MEMWAIT_W::new(self)
    }
    ///Bits 16:23 - MEMHOLDx
    #[inline(always)]
    #[must_use]
    pub fn memhold(&mut self) -> MEMHOLD_W<16> {
        MEMHOLD_W::new(self)
    }
    ///Bits 24:31 - MEMHIZx
    #[inline(always)]
    #[must_use]
    pub fn memhiz(&mut self) -> MEMHIZ_W<24> {
        MEMHIZ_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Common memory space timing register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pmem2](index.html) module
pub struct PMEM2_SPEC;
impl crate::RegisterSpec for PMEM2_SPEC {
    type Ux = u32;
}
///`read()` method returns [pmem2::R](R) reader structure
impl crate::Readable for PMEM2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pmem2::W](W) writer structure
impl crate::Writable for PMEM2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PMEM2 to value 0xfcfc_fcfc
impl crate::Resettable for PMEM2_SPEC {
    const RESET_VALUE: Self::Ux = 0xfcfc_fcfc;
}
