///Register `FTSR2` reader
pub struct R(crate::R<FTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTSR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FTSR2` writer
pub struct W(crate::W<FTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTSR2_SPEC>;
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
impl From<crate::W<FTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTSR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FT50` reader - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.
pub type FT50_R = crate::BitReader<bool>;
///Field `FT50` writer - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.
pub type FT50_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR2_SPEC, bool, O>;
///Field `FT53` reader - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.
pub type FT53_R = crate::BitReader<bool>;
///Field `FT53` writer - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.
pub type FT53_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR2_SPEC, bool, O>;
impl R {
    ///Bit 18 - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.
    #[inline(always)]
    pub fn ft50(&self) -> FT50_R {
        FT50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.
    #[inline(always)]
    pub fn ft53(&self) -> FT53_R {
        FT53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 18 - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.
    #[inline(always)]
    #[must_use]
    pub fn ft50(&mut self) -> FT50_W<18> {
        FT50_W::new(self)
    }
    ///Bit 21 - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.
    #[inline(always)]
    #[must_use]
    pub fn ft53(&mut self) -> FT53_W<21> {
        FT53_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI falling trigger selection register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ftsr2](index.html) module
pub struct FTSR2_SPEC;
impl crate::RegisterSpec for FTSR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [ftsr2::R](R) reader structure
impl crate::Readable for FTSR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ftsr2::W](W) writer structure
impl crate::Writable for FTSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FTSR2 to value 0
impl crate::Resettable for FTSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
