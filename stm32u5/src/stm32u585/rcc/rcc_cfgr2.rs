///Register `RCC_CFGR2` reader
pub struct R(crate::R<RCC_CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_CFGR2` writer
pub struct W(crate::W<RCC_CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CFGR2_SPEC>;
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
impl From<crate::W<RCC_CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HPRE` reader - AHB prescaler Set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to ). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided
pub type HPRE_R = crate::FieldReader<u8, u8>;
///Field `HPRE` writer - AHB prescaler Set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to ). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided
pub type HPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CFGR2_SPEC, u8, u8, 4, O>;
///Field `PPRE1` reader - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided
pub type PPRE1_R = crate::FieldReader<u8, u8>;
///Field `PPRE1` writer - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided
pub type PPRE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CFGR2_SPEC, u8, u8, 3, O>;
///Field `PPRE2` reader - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided
pub type PPRE2_R = crate::FieldReader<u8, u8>;
///Field `PPRE2` writer - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided
pub type PPRE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CFGR2_SPEC, u8, u8, 3, O>;
///Field `AHB1DIS` reader - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1.
pub type AHB1DIS_R = crate::BitReader<bool>;
///Field `AHB1DIS` writer - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1.
pub type AHB1DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CFGR2_SPEC, bool, O>;
///Field `AHB2DIS1` reader - AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3.
pub type AHB2DIS1_R = crate::BitReader<bool>;
///Field `AHB2DIS1` writer - AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3.
pub type AHB2DIS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CFGR2_SPEC, bool, O>;
///Field `AHB2DIS2` reader - AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2EBNR2 are off.
pub type AHB2DIS2_R = crate::BitReader<bool>;
///Field `AHB2DIS2` writer - AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2EBNR2 are off.
pub type AHB2DIS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CFGR2_SPEC, bool, O>;
///Field `APB1DIS` reader - APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.
pub type APB1DIS_R = crate::BitReader<bool>;
///Field `APB1DIS` writer - APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.
pub type APB1DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CFGR2_SPEC, bool, O>;
///Field `APB2DIS` reader - APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off.
pub type APB2DIS_R = crate::BitReader<bool>;
///Field `APB2DIS` writer - APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off.
pub type APB2DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CFGR2_SPEC, bool, O>;
impl R {
    ///Bits 0:3 - AHB prescaler Set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to ). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 16 - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1.
    #[inline(always)]
    pub fn ahb1dis(&self) -> AHB1DIS_R {
        AHB1DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3.
    #[inline(always)]
    pub fn ahb2dis1(&self) -> AHB2DIS1_R {
        AHB2DIS1_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2EBNR2 are off.
    #[inline(always)]
    pub fn ahb2dis2(&self) -> AHB2DIS2_R {
        AHB2DIS2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.
    #[inline(always)]
    pub fn apb1dis(&self) -> APB1DIS_R {
        APB1DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off.
    #[inline(always)]
    pub fn apb2dis(&self) -> APB2DIS_R {
        APB2DIS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - AHB prescaler Set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to ). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<0> {
        HPRE_W::new(self)
    }
    ///Bits 4:6 - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<4> {
        PPRE1_W::new(self)
    }
    ///Bits 8:10 - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<8> {
        PPRE2_W::new(self)
    }
    ///Bit 16 - AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1.
    #[inline(always)]
    #[must_use]
    pub fn ahb1dis(&mut self) -> AHB1DIS_W<16> {
        AHB1DIS_W::new(self)
    }
    ///Bit 17 - AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3.
    #[inline(always)]
    #[must_use]
    pub fn ahb2dis1(&mut self) -> AHB2DIS1_W<17> {
        AHB2DIS1_W::new(self)
    }
    ///Bit 18 - AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2EBNR2 are off.
    #[inline(always)]
    #[must_use]
    pub fn ahb2dis2(&mut self) -> AHB2DIS2_W<18> {
        AHB2DIS2_W::new(self)
    }
    ///Bit 19 - APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.
    #[inline(always)]
    #[must_use]
    pub fn apb1dis(&mut self) -> APB1DIS_W<19> {
        APB1DIS_W::new(self)
    }
    ///Bit 20 - APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off.
    #[inline(always)]
    #[must_use]
    pub fn apb2dis(&mut self) -> APB2DIS_W<20> {
        APB2DIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC clock configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_cfgr2](index.html) module
pub struct RCC_CFGR2_SPEC;
impl crate::RegisterSpec for RCC_CFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_cfgr2::R](R) reader structure
impl crate::Readable for RCC_CFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_cfgr2::W](W) writer structure
impl crate::Writable for RCC_CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_CFGR2 to value 0
impl crate::Resettable for RCC_CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
