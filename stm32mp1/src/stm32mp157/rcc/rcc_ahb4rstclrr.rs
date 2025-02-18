///Register `RCC_AHB4RSTCLRR` reader
pub struct R(crate::R<RCC_AHB4RSTCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB4RSTCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB4RSTCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB4RSTCLRR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_AHB4RSTCLRR` writer
pub struct W(crate::W<RCC_AHB4RSTCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB4RSTCLRR_SPEC>;
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
impl From<crate::W<RCC_AHB4RSTCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB4RSTCLRR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOARST` reader - GPIOARST
pub type GPIOARST_R = crate::BitReader<bool>;
///Field `GPIOARST` writer - GPIOARST
pub type GPIOARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB4RSTCLRR_SPEC, bool, O>;
///Field `GPIOBRST` reader - GPIOBRST
pub type GPIOBRST_R = crate::BitReader<bool>;
///Field `GPIOBRST` writer - GPIOBRST
pub type GPIOBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB4RSTCLRR_SPEC, bool, O>;
///Field `GPIOCRST` reader - GPIOCRST
pub type GPIOCRST_R = crate::BitReader<bool>;
///Field `GPIOCRST` writer - GPIOCRST
pub type GPIOCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB4RSTCLRR_SPEC, bool, O>;
///Field `GPIODRST` reader - GPIODRST
pub type GPIODRST_R = crate::BitReader<bool>;
///Field `GPIODRST` writer - GPIODRST
pub type GPIODRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB4RSTCLRR_SPEC, bool, O>;
///Field `GPIOERST` reader - GPIOERST
pub type GPIOERST_R = crate::BitReader<bool>;
///Field `GPIOERST` writer - GPIOERST
pub type GPIOERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB4RSTCLRR_SPEC, bool, O>;
///Field `GPIOFRST` reader - GPIOFRST
pub type GPIOFRST_R = crate::BitReader<bool>;
///Field `GPIOFRST` writer - GPIOFRST
pub type GPIOFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB4RSTCLRR_SPEC, bool, O>;
///Field `GPIOGRST` reader - GPIOGRST
pub type GPIOGRST_R = crate::BitReader<bool>;
///Field `GPIOGRST` writer - GPIOGRST
pub type GPIOGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB4RSTCLRR_SPEC, bool, O>;
///Field `GPIOHRST` reader - GPIOHRST
pub type GPIOHRST_R = crate::BitReader<bool>;
///Field `GPIOHRST` writer - GPIOHRST
pub type GPIOHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB4RSTCLRR_SPEC, bool, O>;
///Field `GPIOIRST` reader - GPIOIRST
pub type GPIOIRST_R = crate::BitReader<bool>;
///Field `GPIOIRST` writer - GPIOIRST
pub type GPIOIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB4RSTCLRR_SPEC, bool, O>;
///Field `GPIOJRST` reader - GPIOJRST
pub type GPIOJRST_R = crate::BitReader<bool>;
///Field `GPIOJRST` writer - GPIOJRST
pub type GPIOJRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB4RSTCLRR_SPEC, bool, O>;
///Field `GPIOKRST` reader - GPIOKRST
pub type GPIOKRST_R = crate::BitReader<bool>;
///Field `GPIOKRST` writer - GPIOKRST
pub type GPIOKRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB4RSTCLRR_SPEC, bool, O>;
impl R {
    ///Bit 0 - GPIOARST
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOBRST
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOCRST
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIODRST
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOERST
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOFRST
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOGRST
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOHRST
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIOIRST
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPIOJRST
    #[inline(always)]
    pub fn gpiojrst(&self) -> GPIOJRST_R {
        GPIOJRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GPIOKRST
    #[inline(always)]
    pub fn gpiokrst(&self) -> GPIOKRST_R {
        GPIOKRST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPIOARST
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<0> {
        GPIOARST_W::new(self)
    }
    ///Bit 1 - GPIOBRST
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<1> {
        GPIOBRST_W::new(self)
    }
    ///Bit 2 - GPIOCRST
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<2> {
        GPIOCRST_W::new(self)
    }
    ///Bit 3 - GPIODRST
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<3> {
        GPIODRST_W::new(self)
    }
    ///Bit 4 - GPIOERST
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<4> {
        GPIOERST_W::new(self)
    }
    ///Bit 5 - GPIOFRST
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<5> {
        GPIOFRST_W::new(self)
    }
    ///Bit 6 - GPIOGRST
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<6> {
        GPIOGRST_W::new(self)
    }
    ///Bit 7 - GPIOHRST
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<7> {
        GPIOHRST_W::new(self)
    }
    ///Bit 8 - GPIOIRST
    #[inline(always)]
    #[must_use]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<8> {
        GPIOIRST_W::new(self)
    }
    ///Bit 9 - GPIOJRST
    #[inline(always)]
    #[must_use]
    pub fn gpiojrst(&mut self) -> GPIOJRST_W<9> {
        GPIOJRST_W::new(self)
    }
    ///Bit 10 - GPIOKRST
    #[inline(always)]
    #[must_use]
    pub fn gpiokrst(&mut self) -> GPIOKRST_W<10> {
        GPIOKRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to release the reset of the corresponding peripheral.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ahb4rstclrr](index.html) module
pub struct RCC_AHB4RSTCLRR_SPEC;
impl crate::RegisterSpec for RCC_AHB4RSTCLRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ahb4rstclrr::R](R) reader structure
impl crate::Readable for RCC_AHB4RSTCLRR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_ahb4rstclrr::W](W) writer structure
impl crate::Writable for RCC_AHB4RSTCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_AHB4RSTCLRR to value 0
impl crate::Resettable for RCC_AHB4RSTCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
