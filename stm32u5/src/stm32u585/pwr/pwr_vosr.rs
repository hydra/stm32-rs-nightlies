///Register `PWR_VOSR` reader
pub struct R(crate::R<PWR_VOSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_VOSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_VOSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_VOSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PWR_VOSR` writer
pub struct W(crate::W<PWR_VOSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_VOSR_SPEC>;
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
impl From<crate::W<PWR_VOSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_VOSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BOOSTRDY` reader - EPOD booster ready This bit is set to 1 by hardware when the power booster startup time is reached. The system clock frequency can be switched higher than 50 MHz only after this bit is set.
pub type BOOSTRDY_R = crate::BitReader<bool>;
///Field `VOSRDY` reader - Ready bit for VCORE voltage scaling output selection
pub type VOSRDY_R = crate::BitReader<bool>;
///Field `VOS` reader - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
pub type VOS_R = crate::FieldReader<u8, u8>;
///Field `VOS` writer - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
pub type VOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_VOSR_SPEC, u8, u8, 2, O>;
///Field `BOOSTEN` reader - EPOD booster enable
pub type BOOSTEN_R = crate::BitReader<bool>;
///Field `BOOSTEN` writer - EPOD booster enable
pub type BOOSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_VOSR_SPEC, bool, O>;
impl R {
    ///Bit 14 - EPOD booster ready This bit is set to 1 by hardware when the power booster startup time is reached. The system clock frequency can be switched higher than 50 MHz only after this bit is set.
    #[inline(always)]
    pub fn boostrdy(&self) -> BOOSTRDY_R {
        BOOSTRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Ready bit for VCORE voltage scaling output selection
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - EPOD booster enable
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bits 16:17 - Voltage scaling range selection This field is protected against non-secure access when SYSCLKSEC = 1 in RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<16> {
        VOS_W::new(self)
    }
    ///Bit 18 - EPOD booster enable
    #[inline(always)]
    #[must_use]
    pub fn boosten(&mut self) -> BOOSTEN_W<18> {
        BOOSTEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR voltage scaling register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_vosr](index.html) module
pub struct PWR_VOSR_SPEC;
impl crate::RegisterSpec for PWR_VOSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_vosr::R](R) reader structure
impl crate::Readable for PWR_VOSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pwr_vosr::W](W) writer structure
impl crate::Writable for PWR_VOSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PWR_VOSR to value 0x8000
impl crate::Resettable for PWR_VOSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
