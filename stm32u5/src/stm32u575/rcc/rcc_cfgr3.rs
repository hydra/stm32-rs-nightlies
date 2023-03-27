///Register `RCC_CFGR3` reader
pub struct R(crate::R<RCC_CFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CFGR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_CFGR3` writer
pub struct W(crate::W<RCC_CFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CFGR3_SPEC>;
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
impl From<crate::W<RCC_CFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CFGR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PPRE3` reader - APB3 prescaler Set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided
pub type PPRE3_R = crate::FieldReader<u8, u8>;
///Field `PPRE3` writer - APB3 prescaler Set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided
pub type PPRE3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CFGR3_SPEC, u8, u8, 3, O>;
///Field `AHB3DIS` reader - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4.
pub type AHB3DIS_R = crate::BitReader<bool>;
///Field `AHB3DIS` writer - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4.
pub type AHB3DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CFGR3_SPEC, bool, O>;
///Field `APB3DIS` reader - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off.
pub type APB3DIS_R = crate::BitReader<bool>;
///Field `APB3DIS` writer - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off.
pub type APB3DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CFGR3_SPEC, bool, O>;
impl R {
    ///Bits 4:6 - APB3 prescaler Set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided
    #[inline(always)]
    pub fn ppre3(&self) -> PPRE3_R {
        PPRE3_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 16 - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4.
    #[inline(always)]
    pub fn ahb3dis(&self) -> AHB3DIS_R {
        AHB3DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off.
    #[inline(always)]
    pub fn apb3dis(&self) -> APB3DIS_R {
        APB3DIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    ///Bits 4:6 - APB3 prescaler Set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided
    #[inline(always)]
    #[must_use]
    pub fn ppre3(&mut self) -> PPRE3_W<4> {
        PPRE3_W::new(self)
    }
    ///Bit 16 - AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4.
    #[inline(always)]
    #[must_use]
    pub fn ahb3dis(&mut self) -> AHB3DIS_W<16> {
        AHB3DIS_W::new(self)
    }
    ///Bit 17 - APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off.
    #[inline(always)]
    #[must_use]
    pub fn apb3dis(&mut self) -> APB3DIS_W<17> {
        APB3DIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC clock configuration register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_cfgr3](index.html) module
pub struct RCC_CFGR3_SPEC;
impl crate::RegisterSpec for RCC_CFGR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_cfgr3::R](R) reader structure
impl crate::Readable for RCC_CFGR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_cfgr3::W](W) writer structure
impl crate::Writable for RCC_CFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_CFGR3 to value 0
impl crate::Resettable for RCC_CFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
