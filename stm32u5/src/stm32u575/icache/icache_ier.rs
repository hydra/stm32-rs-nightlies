///Register `ICACHE_IER` reader
pub struct R(crate::R<ICACHE_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ICACHE_IER` writer
pub struct W(crate::W<ICACHE_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_IER_SPEC>;
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
impl From<crate::W<ICACHE_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BSYENDIE` reader - BSYENDIE
pub type BSYENDIE_R = crate::BitReader<bool>;
///Field `BSYENDIE` writer - BSYENDIE
pub type BSYENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_IER_SPEC, bool, O>;
///Field `ERRIE` reader - ERRIE
pub type ERRIE_R = crate::BitReader<bool>;
///Field `ERRIE` writer - ERRIE
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_IER_SPEC, bool, O>;
impl R {
    ///Bit 1 - BSYENDIE
    #[inline(always)]
    pub fn bsyendie(&self) -> BSYENDIE_R {
        BSYENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ERRIE
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - BSYENDIE
    #[inline(always)]
    #[must_use]
    pub fn bsyendie(&mut self) -> BSYENDIE_W<1> {
        BSYENDIE_W::new(self)
    }
    ///Bit 2 - ERRIE
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<2> {
        ERRIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ICACHE interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icache_ier](index.html) module
pub struct ICACHE_IER_SPEC;
impl crate::RegisterSpec for ICACHE_IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [icache_ier::R](R) reader structure
impl crate::Readable for ICACHE_IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [icache_ier::W](W) writer structure
impl crate::Writable for ICACHE_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ICACHE_IER to value 0
impl crate::Resettable for ICACHE_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
