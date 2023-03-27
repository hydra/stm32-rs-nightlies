///Register `RCC_AHB2SMENR2` reader
pub struct R(crate::R<RCC_AHB2SMENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB2SMENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB2SMENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB2SMENR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_AHB2SMENR2` writer
pub struct W(crate::W<RCC_AHB2SMENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB2SMENR2_SPEC>;
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
impl From<crate::W<RCC_AHB2SMENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB2SMENR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FSMCSMEN` reader - FSMC clocks enable during Sleep and Stop modes Set and cleared by software.
pub type FSMCSMEN_R = crate::BitReader<bool>;
///Field `FSMCSMEN` writer - FSMC clocks enable during Sleep and Stop modes Set and cleared by software.
pub type FSMCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR2_SPEC, bool, O>;
///Field `OCTOSPI1SMEN` reader - OCTOSPI1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type OCTOSPI1SMEN_R = crate::BitReader<bool>;
///Field `OCTOSPI1SMEN` writer - OCTOSPI1 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type OCTOSPI1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR2_SPEC, bool, O>;
///Field `OCTOSPI2SMEN` reader - OCTOSPI2 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type OCTOSPI2SMEN_R = crate::BitReader<bool>;
///Field `OCTOSPI2SMEN` writer - OCTOSPI2 clocks enable during Sleep and Stop modes Set and cleared by software.
pub type OCTOSPI2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB2SMENR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - FSMC clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn fsmcsmen(&self) -> FSMCSMEN_R {
        FSMCSMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - OCTOSPI1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn octospi1smen(&self) -> OCTOSPI1SMEN_R {
        OCTOSPI1SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - OCTOSPI2 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn octospi2smen(&self) -> OCTOSPI2SMEN_R {
        OCTOSPI2SMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - FSMC clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn fsmcsmen(&mut self) -> FSMCSMEN_W<0> {
        FSMCSMEN_W::new(self)
    }
    ///Bit 4 - OCTOSPI1 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn octospi1smen(&mut self) -> OCTOSPI1SMEN_W<4> {
        OCTOSPI1SMEN_W::new(self)
    }
    ///Bit 8 - OCTOSPI2 clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn octospi2smen(&mut self) -> OCTOSPI2SMEN_W<8> {
        OCTOSPI2SMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB2 peripheral clocks enable in Sleep and Stop modes register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ahb2smenr2](index.html) module
pub struct RCC_AHB2SMENR2_SPEC;
impl crate::RegisterSpec for RCC_AHB2SMENR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ahb2smenr2::R](R) reader structure
impl crate::Readable for RCC_AHB2SMENR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_ahb2smenr2::W](W) writer structure
impl crate::Writable for RCC_AHB2SMENR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_AHB2SMENR2 to value 0xffff_ffff
impl crate::Resettable for RCC_AHB2SMENR2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
