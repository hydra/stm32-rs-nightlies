///Register `DCACHE_CMDRSADDRR` reader
pub struct R(crate::R<DCACHE_CMDRSADDRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_CMDRSADDRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_CMDRSADDRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_CMDRSADDRR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DCACHE_CMDRSADDRR` writer
pub struct W(crate::W<DCACHE_CMDRSADDRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_CMDRSADDRR_SPEC>;
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
impl From<crate::W<DCACHE_CMDRSADDRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_CMDRSADDRR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CMDSTARTADDR` reader - CMDSTARTADDR
pub type CMDSTARTADDR_R = crate::FieldReader<u32, u32>;
///Field `CMDSTARTADDR` writer - CMDSTARTADDR
pub type CMDSTARTADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCACHE_CMDRSADDRR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CMDSTARTADDR
    #[inline(always)]
    pub fn cmdstartaddr(&self) -> CMDSTARTADDR_R {
        CMDSTARTADDR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CMDSTARTADDR
    #[inline(always)]
    #[must_use]
    pub fn cmdstartaddr(&mut self) -> CMDSTARTADDR_W<0> {
        CMDSTARTADDR_W::new(self)
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
///For information about available fields see [dcache_cmdrsaddrr](index.html) module
pub struct DCACHE_CMDRSADDRR_SPEC;
impl crate::RegisterSpec for DCACHE_CMDRSADDRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dcache_cmdrsaddrr::R](R) reader structure
impl crate::Readable for DCACHE_CMDRSADDRR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dcache_cmdrsaddrr::W](W) writer structure
impl crate::Writable for DCACHE_CMDRSADDRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DCACHE_CMDRSADDRR to value 0
impl crate::Resettable for DCACHE_CMDRSADDRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
