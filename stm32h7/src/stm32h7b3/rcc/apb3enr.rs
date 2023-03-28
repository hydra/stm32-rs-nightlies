///Register `APB3ENR` reader
pub struct R(crate::R<APB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB3ENR` writer
pub struct W(crate::W<APB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB3ENR_SPEC>;
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
impl From<crate::W<APB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LTDCEN` reader - LTDC clock enable Provides the clock (ltdc_pclk, ltdc_aclk, ltdc_ker_ck) to the LTDC block. Set and reset by software.
pub type LTDCEN_R = crate::BitReader<LTDCEN_A>;
///LTDC clock enable Provides the clock (ltdc_pclk, ltdc_aclk, ltdc_ker_ck) to the LTDC block. Set and reset by software.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCEN_A {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<LTDCEN_A> for bool {
    #[inline(always)]
    fn from(variant: LTDCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LTDCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LTDCEN_A {
        match self.bits {
            false => LTDCEN_A::Disabled,
            true => LTDCEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCEN_A::Enabled
    }
}
///Field `LTDCEN` writer - LTDC clock enable Provides the clock (ltdc_pclk, ltdc_aclk, ltdc_ker_ck) to the LTDC block. Set and reset by software.
pub type LTDCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, LTDCEN_A, O>;
impl<'a, const O: u8> LTDCEN_W<'a, O> {
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LTDCEN_A::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LTDCEN_A::Enabled)
    }
}
///Field `WWDGEN` reader - WWDG clock enable Set by software, and reset by hardware when a system reset occurs. Note that in order to work properly, before enabling the WWDG, the bit WW1RSC must be set to 1.
pub use LTDCEN_R as WWDGEN_R;
///Field `WWDGEN` writer - WWDG clock enable Set by software, and reset by hardware when a system reset occurs. Note that in order to work properly, before enabling the WWDG, the bit WW1RSC must be set to 1.
pub use LTDCEN_W as WWDGEN_W;
impl R {
    ///Bit 3 - LTDC clock enable Provides the clock (ltdc_pclk, ltdc_aclk, ltdc_ker_ck) to the LTDC block. Set and reset by software.
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - WWDG clock enable Set by software, and reset by hardware when a system reset occurs. Note that in order to work properly, before enabling the WWDG, the bit WW1RSC must be set to 1.
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - LTDC clock enable Provides the clock (ltdc_pclk, ltdc_aclk, ltdc_ker_ck) to the LTDC block. Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<3> {
        LTDCEN_W::new(self)
    }
    ///Bit 6 - WWDG clock enable Set by software, and reset by hardware when a system reset occurs. Note that in order to work properly, before enabling the WWDG, the bit WW1RSC must be set to 1.
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<6> {
        WWDGEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb3enr](index.html) module
pub struct APB3ENR_SPEC;
impl crate::RegisterSpec for APB3ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb3enr::R](R) reader structure
impl crate::Readable for APB3ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb3enr::W](W) writer structure
impl crate::Writable for APB3ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB3ENR to value 0
impl crate::Resettable for APB3ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
