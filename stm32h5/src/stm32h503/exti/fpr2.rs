///Register `FPR2` reader
pub struct R(crate::R<FPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FPR2` writer
pub struct W(crate::W<FPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPR2_SPEC>;
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
impl From<crate::W<FPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FPIF50` reader - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub type FPIF50_R = crate::BitReader<bool>;
///Field `FPIF50` writer - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub type FPIF50_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR2_SPEC, bool, O>;
///Field `FPIF53` reader - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub type FPIF53_R = crate::BitReader<bool>;
///Field `FPIF53` writer - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub type FPIF53_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR2_SPEC, bool, O>;
impl R {
    ///Bit 18 - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    pub fn fpif50(&self) -> FPIF50_R {
        FPIF50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    pub fn fpif53(&self) -> FPIF53_R {
        FPIF53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 18 - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    #[must_use]
    pub fn fpif50(&mut self) -> FPIF50_W<18> {
        FPIF50_W::new(self)
    }
    ///Bit 21 - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    #[must_use]
    pub fn fpif53(&mut self) -> FPIF53_W<21> {
        FPIF53_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI falling edge pending register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fpr2](index.html) module
pub struct FPR2_SPEC;
impl crate::RegisterSpec for FPR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [fpr2::R](R) reader structure
impl crate::Readable for FPR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fpr2::W](W) writer structure
impl crate::Writable for FPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FPR2 to value 0
impl crate::Resettable for FPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
