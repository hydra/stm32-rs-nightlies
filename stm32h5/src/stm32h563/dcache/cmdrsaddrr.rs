///Register `CMDRSADDRR` reader
pub struct R(crate::R<CMDRSADDRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDRSADDRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDRSADDRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDRSADDRR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CMDRSADDRR` writer
pub struct W(crate::W<CMDRSADDRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDRSADDRR_SPEC>;
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
impl From<crate::W<CMDRSADDRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDRSADDRR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CMDSTARTADDR` reader - start address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written. .
pub type CMDSTARTADDR_R = crate::FieldReader<u32, u32>;
///Field `CMDSTARTADDR` writer - start address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written. .
pub type CMDSTARTADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMDRSADDRR_SPEC, u32, u32, 28, O>;
impl R {
    ///Bits 4:31 - start address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written. .
    #[inline(always)]
    pub fn cmdstartaddr(&self) -> CMDSTARTADDR_R {
        CMDSTARTADDR_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    ///Bits 4:31 - start address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written. .
    #[inline(always)]
    #[must_use]
    pub fn cmdstartaddr(&mut self) -> CMDSTARTADDR_W<4> {
        CMDSTARTADDR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DCACHE command range start address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmdrsaddrr](index.html) module
pub struct CMDRSADDRR_SPEC;
impl crate::RegisterSpec for CMDRSADDRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cmdrsaddrr::R](R) reader structure
impl crate::Readable for CMDRSADDRR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cmdrsaddrr::W](W) writer structure
impl crate::Writable for CMDRSADDRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CMDRSADDRR to value 0
impl crate::Resettable for CMDRSADDRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
