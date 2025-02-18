///Register `RCC_PLL4CFGR1` reader
pub struct R(crate::R<RCC_PLL4CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL4CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL4CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL4CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_PLL4CFGR1` writer
pub struct W(crate::W<RCC_PLL4CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL4CFGR1_SPEC>;
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
impl From<crate::W<RCC_PLL4CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL4CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DIVN` reader - DIVN
pub type DIVN_R = crate::FieldReader<u16, u16>;
///Field `DIVN` writer - DIVN
pub type DIVN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL4CFGR1_SPEC, u16, u16, 9, O>;
///Field `DIVM4` reader - DIVM4
pub type DIVM4_R = crate::FieldReader<u8, u8>;
///Field `DIVM4` writer - DIVM4
pub type DIVM4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL4CFGR1_SPEC, u8, u8, 6, O>;
///Field `IFRGE` reader - IFRGE
pub type IFRGE_R = crate::FieldReader<u8, u8>;
///Field `IFRGE` writer - IFRGE
pub type IFRGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL4CFGR1_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:8 - DIVN
    #[inline(always)]
    pub fn divn(&self) -> DIVN_R {
        DIVN_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:21 - DIVM4
    #[inline(always)]
    pub fn divm4(&self) -> DIVM4_R {
        DIVM4_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:25 - IFRGE
    #[inline(always)]
    pub fn ifrge(&self) -> IFRGE_R {
        IFRGE_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    ///Bits 0:8 - DIVN
    #[inline(always)]
    #[must_use]
    pub fn divn(&mut self) -> DIVN_W<0> {
        DIVN_W::new(self)
    }
    ///Bits 16:21 - DIVM4
    #[inline(always)]
    #[must_use]
    pub fn divm4(&mut self) -> DIVM4_W<16> {
        DIVM4_W::new(self)
    }
    ///Bits 24:25 - IFRGE
    #[inline(always)]
    #[must_use]
    pub fn ifrge(&mut self) -> IFRGE_W<24> {
        IFRGE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to configure the PLL4.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_pll4cfgr1](index.html) module
pub struct RCC_PLL4CFGR1_SPEC;
impl crate::RegisterSpec for RCC_PLL4CFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_pll4cfgr1::R](R) reader structure
impl crate::Readable for RCC_PLL4CFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_pll4cfgr1::W](W) writer structure
impl crate::Writable for RCC_PLL4CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_PLL4CFGR1 to value 0x0001_0031
impl crate::Resettable for RCC_PLL4CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0031;
}
