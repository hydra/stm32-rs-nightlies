///Register `CONFR0` writer
pub struct W(crate::W<CONFR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFR0_SPEC>;
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
impl From<crate::W<CONFR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `START` writer - Start This bit start or stop the encoding or decoding process. Read this register always return 0.
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFR0_SPEC, bool, O>;
impl W {
    ///Bit 0 - Start This bit start or stop the encoding or decoding process. Read this register always return 0.
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///JPEG codec control register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [confr0](index.html) module
pub struct CONFR0_SPEC;
impl crate::RegisterSpec for CONFR0_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [confr0::W](W) writer structure
impl crate::Writable for CONFR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CONFR0 to value 0
impl crate::Resettable for CONFR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
