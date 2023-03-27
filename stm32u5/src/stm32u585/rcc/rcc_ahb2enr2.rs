///Register `RCC_AHB2ENR2` reader
pub struct R(crate::R<RCC_AHB2ENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB2ENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB2ENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB2ENR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_AHB2ENR2` writer
pub struct W(crate::W<RCC_AHB2ENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB2ENR2_SPEC>;
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
impl From<crate::W<RCC_AHB2ENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB2ENR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FSMCEN` reader - FSMC clock enable Set and cleared by software.
pub type FSMCEN_R = crate::BitReader<bool>;
///Field `FSMCEN` writer - FSMC clock enable Set and cleared by software.
pub type FSMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR2_SPEC, bool, O>;
///Field `OCTOSPI1EN` reader - OCTOSPI1 clock enable Set and cleared by software.
pub type OCTOSPI1EN_R = crate::BitReader<bool>;
///Field `OCTOSPI1EN` writer - OCTOSPI1 clock enable Set and cleared by software.
pub type OCTOSPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR2_SPEC, bool, O>;
///Field `OCTOSPI2EN` reader - OCTOSPI2 clock enable Set and cleared by software.
pub type OCTOSPI2EN_R = crate::BitReader<bool>;
///Field `OCTOSPI2EN` writer - OCTOSPI2 clock enable Set and cleared by software.
pub type OCTOSPI2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2ENR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - FSMC clock enable Set and cleared by software.
    #[inline(always)]
    pub fn fsmcen(&self) -> FSMCEN_R {
        FSMCEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - OCTOSPI1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn octospi1en(&self) -> OCTOSPI1EN_R {
        OCTOSPI1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - OCTOSPI2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn octospi2en(&self) -> OCTOSPI2EN_R {
        OCTOSPI2EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - FSMC clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn fsmcen(&mut self) -> FSMCEN_W<0> {
        FSMCEN_W::new(self)
    }
    ///Bit 4 - OCTOSPI1 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn octospi1en(&mut self) -> OCTOSPI1EN_W<4> {
        OCTOSPI1EN_W::new(self)
    }
    ///Bit 8 - OCTOSPI2 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn octospi2en(&mut self) -> OCTOSPI2EN_W<8> {
        OCTOSPI2EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB2 peripheral clock enable register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ahb2enr2](index.html) module
pub struct RCC_AHB2ENR2_SPEC;
impl crate::RegisterSpec for RCC_AHB2ENR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ahb2enr2::R](R) reader structure
impl crate::Readable for RCC_AHB2ENR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_ahb2enr2::W](W) writer structure
impl crate::Writable for RCC_AHB2ENR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_AHB2ENR2 to value 0
impl crate::Resettable for RCC_AHB2ENR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
