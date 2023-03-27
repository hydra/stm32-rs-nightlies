///Register `DCACHE_CMDREADDRR` reader
pub struct R(crate::R<DCACHE_CMDREADDRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_CMDREADDRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_CMDREADDRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_CMDREADDRR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DCACHE_CMDREADDRR` writer
pub struct W(crate::W<DCACHE_CMDREADDRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_CMDREADDRR_SPEC>;
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
impl From<crate::W<DCACHE_CMDREADDRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_CMDREADDRR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CMDENDADDR` reader - CMDENDADDR
pub type CMDENDADDR_R = crate::FieldReader<u32, u32>;
///Field `CMDENDADDR` writer - CMDENDADDR
pub type CMDENDADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCACHE_CMDREADDRR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CMDENDADDR
    #[inline(always)]
    pub fn cmdendaddr(&self) -> CMDENDADDR_R {
        CMDENDADDR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CMDENDADDR
    #[inline(always)]
    #[must_use]
    pub fn cmdendaddr(&mut self) -> CMDENDADDR_W<0> {
        CMDENDADDR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///command range start address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcache_cmdreaddrr](index.html) module
pub struct DCACHE_CMDREADDRR_SPEC;
impl crate::RegisterSpec for DCACHE_CMDREADDRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dcache_cmdreaddrr::R](R) reader structure
impl crate::Readable for DCACHE_CMDREADDRR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dcache_cmdreaddrr::W](W) writer structure
impl crate::Writable for DCACHE_CMDREADDRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DCACHE_CMDREADDRR to value 0
impl crate::Resettable for DCACHE_CMDREADDRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
