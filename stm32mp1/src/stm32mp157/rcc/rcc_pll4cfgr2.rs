///Register `RCC_PLL4CFGR2` reader
pub struct R(crate::R<RCC_PLL4CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL4CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL4CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL4CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_PLL4CFGR2` writer
pub struct W(crate::W<RCC_PLL4CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL4CFGR2_SPEC>;
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
impl From<crate::W<RCC_PLL4CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL4CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DIVP` reader - DIVP
pub type DIVP_R = crate::FieldReader<u8, u8>;
///Field `DIVP` writer - DIVP
pub type DIVP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL4CFGR2_SPEC, u8, u8, 7, O>;
///Field `DIVQ` reader - DIVQ
pub type DIVQ_R = crate::FieldReader<u8, u8>;
///Field `DIVQ` writer - DIVQ
pub type DIVQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL4CFGR2_SPEC, u8, u8, 7, O>;
///Field `DIVR` reader - DIVR
pub type DIVR_R = crate::FieldReader<u8, u8>;
///Field `DIVR` writer - DIVR
pub type DIVR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL4CFGR2_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:6 - DIVP
    #[inline(always)]
    pub fn divp(&self) -> DIVP_R {
        DIVP_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:14 - DIVQ
    #[inline(always)]
    pub fn divq(&self) -> DIVQ_R {
        DIVQ_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bits 16:22 - DIVR
    #[inline(always)]
    pub fn divr(&self) -> DIVR_R {
        DIVR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - DIVP
    #[inline(always)]
    #[must_use]
    pub fn divp(&mut self) -> DIVP_W<0> {
        DIVP_W::new(self)
    }
    ///Bits 8:14 - DIVQ
    #[inline(always)]
    #[must_use]
    pub fn divq(&mut self) -> DIVQ_W<8> {
        DIVQ_W::new(self)
    }
    ///Bits 16:22 - DIVR
    #[inline(always)]
    #[must_use]
    pub fn divr(&mut self) -> DIVR_W<16> {
        DIVR_W::new(self)
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
///For information about available fields see [rcc_pll4cfgr2](index.html) module
pub struct RCC_PLL4CFGR2_SPEC;
impl crate::RegisterSpec for RCC_PLL4CFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_pll4cfgr2::R](R) reader structure
impl crate::Readable for RCC_PLL4CFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_pll4cfgr2::W](W) writer structure
impl crate::Writable for RCC_PLL4CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_PLL4CFGR2 to value 0
impl crate::Resettable for RCC_PLL4CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
