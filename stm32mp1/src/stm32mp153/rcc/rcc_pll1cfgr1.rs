///Register `RCC_PLL1CFGR1` reader
pub struct R(crate::R<RCC_PLL1CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL1CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL1CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL1CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_PLL1CFGR1` writer
pub struct W(crate::W<RCC_PLL1CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL1CFGR1_SPEC>;
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
impl From<crate::W<RCC_PLL1CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL1CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DIVN` reader - DIVN
pub type DIVN_R = crate::FieldReader<u16, u16>;
///Field `DIVN` writer - DIVN
pub type DIVN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL1CFGR1_SPEC, u16, u16, 9, O>;
///Field `DIVM1` reader - DIVM1
pub type DIVM1_R = crate::FieldReader<u8, u8>;
///Field `DIVM1` writer - DIVM1
pub type DIVM1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL1CFGR1_SPEC, u8, u8, 6, O>;
impl R {
    ///Bits 0:8 - DIVN
    #[inline(always)]
    pub fn divn(&self) -> DIVN_R {
        DIVN_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:21 - DIVM1
    #[inline(always)]
    pub fn divm1(&self) -> DIVM1_R {
        DIVM1_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:8 - DIVN
    #[inline(always)]
    #[must_use]
    pub fn divn(&mut self) -> DIVN_W<0> {
        DIVN_W::new(self)
    }
    ///Bits 16:21 - DIVM1
    #[inline(always)]
    #[must_use]
    pub fn divm1(&mut self) -> DIVM1_W<16> {
        DIVM1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_pll1cfgr1](index.html) module
pub struct RCC_PLL1CFGR1_SPEC;
impl crate::RegisterSpec for RCC_PLL1CFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_pll1cfgr1::R](R) reader structure
impl crate::Readable for RCC_PLL1CFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_pll1cfgr1::W](W) writer structure
impl crate::Writable for RCC_PLL1CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_PLL1CFGR1 to value 0x0001_0031
impl crate::Resettable for RCC_PLL1CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0031;
}
