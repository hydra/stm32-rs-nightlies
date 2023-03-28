///Register `CCIPR3` reader
pub struct R(crate::R<CCIPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCIPR3` writer
pub struct W(crate::W<CCIPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR3_SPEC>;
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
impl From<crate::W<CCIPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SPI1SEL` reader - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI1SEL_R = crate::FieldReader<u8, u8>;
///Field `SPI1SEL` writer - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR3_SPEC, u8, u8, 3, O>;
///Field `SPI2SEL` reader - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI2SEL_R = crate::FieldReader<u8, u8>;
///Field `SPI2SEL` writer - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR3_SPEC, u8, u8, 3, O>;
///Field `SPI3SEL` reader - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI3SEL_R = crate::FieldReader<u8, u8>;
///Field `SPI3SEL` writer - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI3SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR3_SPEC, u8, u8, 3, O>;
///Field `LPUART1SEL` reader - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPUART1SEL_R = crate::FieldReader<u8, u8>;
///Field `LPUART1SEL` writer - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPUART1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR3_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:2 - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi1sel(&self) -> SPI1SEL_R {
        SPI1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi2sel(&self) -> SPI2SEL_R {
        SPI2SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi3sel(&self) -> SPI3SEL_R {
        SPI3SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 24:26 - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn spi1sel(&mut self) -> SPI1SEL_W<0> {
        SPI1SEL_W::new(self)
    }
    ///Bits 3:5 - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn spi2sel(&mut self) -> SPI2SEL_W<3> {
        SPI2SEL_W::new(self)
    }
    ///Bits 6:8 - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn spi3sel(&mut self) -> SPI3SEL_W<6> {
        SPI3SEL_W::new(self)
    }
    ///Bits 24:26 - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<24> {
        LPUART1SEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC kernel clock configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccipr3](index.html) module
pub struct CCIPR3_SPEC;
impl crate::RegisterSpec for CCIPR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccipr3::R](R) reader structure
impl crate::Readable for CCIPR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccipr3::W](W) writer structure
impl crate::Writable for CCIPR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCIPR3 to value 0
impl crate::Resettable for CCIPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
