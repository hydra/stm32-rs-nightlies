///Register `RCC_MP_AHB4LPENSETR` reader
pub struct R(crate::R<RCC_MP_AHB4LPENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_AHB4LPENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_AHB4LPENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_AHB4LPENSETR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MP_AHB4LPENSETR` writer
pub struct W(crate::W<RCC_MP_AHB4LPENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_AHB4LPENSETR_SPEC>;
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
impl From<crate::W<RCC_MP_AHB4LPENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_AHB4LPENSETR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOALPEN` reader - GPIOALPEN
pub type GPIOALPEN_R = crate::BitReader<bool>;
///Field `GPIOALPEN` writer - GPIOALPEN
pub type GPIOALPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB4LPENSETR_SPEC, bool, O>;
///Field `GPIOBLPEN` reader - GPIOBLPEN
pub type GPIOBLPEN_R = crate::BitReader<bool>;
///Field `GPIOBLPEN` writer - GPIOBLPEN
pub type GPIOBLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB4LPENSETR_SPEC, bool, O>;
///Field `GPIOCLPEN` reader - GPIOCLPEN
pub type GPIOCLPEN_R = crate::BitReader<bool>;
///Field `GPIOCLPEN` writer - GPIOCLPEN
pub type GPIOCLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB4LPENSETR_SPEC, bool, O>;
///Field `GPIODLPEN` reader - GPIODLPEN
pub type GPIODLPEN_R = crate::BitReader<bool>;
///Field `GPIODLPEN` writer - GPIODLPEN
pub type GPIODLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB4LPENSETR_SPEC, bool, O>;
///Field `GPIOELPEN` reader - GPIOELPEN
pub type GPIOELPEN_R = crate::BitReader<bool>;
///Field `GPIOELPEN` writer - GPIOELPEN
pub type GPIOELPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB4LPENSETR_SPEC, bool, O>;
///Field `GPIOFLPEN` reader - GPIOFLPEN
pub type GPIOFLPEN_R = crate::BitReader<bool>;
///Field `GPIOFLPEN` writer - GPIOFLPEN
pub type GPIOFLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB4LPENSETR_SPEC, bool, O>;
///Field `GPIOGLPEN` reader - GPIOGLPEN
pub type GPIOGLPEN_R = crate::BitReader<bool>;
///Field `GPIOGLPEN` writer - GPIOGLPEN
pub type GPIOGLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB4LPENSETR_SPEC, bool, O>;
///Field `GPIOHLPEN` reader - GPIOHLPEN
pub type GPIOHLPEN_R = crate::BitReader<bool>;
///Field `GPIOHLPEN` writer - GPIOHLPEN
pub type GPIOHLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB4LPENSETR_SPEC, bool, O>;
///Field `GPIOILPEN` reader - GPIOILPEN
pub type GPIOILPEN_R = crate::BitReader<bool>;
///Field `GPIOILPEN` writer - GPIOILPEN
pub type GPIOILPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB4LPENSETR_SPEC, bool, O>;
///Field `GPIOJLPEN` reader - GPIOJLPEN
pub type GPIOJLPEN_R = crate::BitReader<bool>;
///Field `GPIOJLPEN` writer - GPIOJLPEN
pub type GPIOJLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB4LPENSETR_SPEC, bool, O>;
///Field `GPIOKLPEN` reader - GPIOKLPEN
pub type GPIOKLPEN_R = crate::BitReader<bool>;
///Field `GPIOKLPEN` writer - GPIOKLPEN
pub type GPIOKLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_AHB4LPENSETR_SPEC, bool, O>;
impl R {
    ///Bit 0 - GPIOALPEN
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOBLPEN
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOCLPEN
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIODLPEN
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOELPEN
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOFLPEN
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOGLPEN
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOHLPEN
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIOILPEN
    #[inline(always)]
    pub fn gpioilpen(&self) -> GPIOILPEN_R {
        GPIOILPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPIOJLPEN
    #[inline(always)]
    pub fn gpiojlpen(&self) -> GPIOJLPEN_R {
        GPIOJLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GPIOKLPEN
    #[inline(always)]
    pub fn gpioklpen(&self) -> GPIOKLPEN_R {
        GPIOKLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPIOALPEN
    #[inline(always)]
    #[must_use]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<0> {
        GPIOALPEN_W::new(self)
    }
    ///Bit 1 - GPIOBLPEN
    #[inline(always)]
    #[must_use]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<1> {
        GPIOBLPEN_W::new(self)
    }
    ///Bit 2 - GPIOCLPEN
    #[inline(always)]
    #[must_use]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<2> {
        GPIOCLPEN_W::new(self)
    }
    ///Bit 3 - GPIODLPEN
    #[inline(always)]
    #[must_use]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<3> {
        GPIODLPEN_W::new(self)
    }
    ///Bit 4 - GPIOELPEN
    #[inline(always)]
    #[must_use]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<4> {
        GPIOELPEN_W::new(self)
    }
    ///Bit 5 - GPIOFLPEN
    #[inline(always)]
    #[must_use]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<5> {
        GPIOFLPEN_W::new(self)
    }
    ///Bit 6 - GPIOGLPEN
    #[inline(always)]
    #[must_use]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<6> {
        GPIOGLPEN_W::new(self)
    }
    ///Bit 7 - GPIOHLPEN
    #[inline(always)]
    #[must_use]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<7> {
        GPIOHLPEN_W::new(self)
    }
    ///Bit 8 - GPIOILPEN
    #[inline(always)]
    #[must_use]
    pub fn gpioilpen(&mut self) -> GPIOILPEN_W<8> {
        GPIOILPEN_W::new(self)
    }
    ///Bit 9 - GPIOJLPEN
    #[inline(always)]
    #[must_use]
    pub fn gpiojlpen(&mut self) -> GPIOJLPEN_W<9> {
        GPIOJLPEN_W::new(self)
    }
    ///Bit 10 - GPIOKLPEN
    #[inline(always)]
    #[must_use]
    pub fn gpioklpen(&mut self) -> GPIOKLPEN_W<10> {
        GPIOKLPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used by the MPU
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mp_ahb4lpensetr](index.html) module
pub struct RCC_MP_AHB4LPENSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_AHB4LPENSETR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mp_ahb4lpensetr::R](R) reader structure
impl crate::Readable for RCC_MP_AHB4LPENSETR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mp_ahb4lpensetr::W](W) writer structure
impl crate::Writable for RCC_MP_AHB4LPENSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MP_AHB4LPENSETR to value 0x07ff
impl crate::Resettable for RCC_MP_AHB4LPENSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07ff;
}
