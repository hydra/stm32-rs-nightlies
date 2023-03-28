///Register `AHBRSTR` reader
pub struct R(crate::R<AHBRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBRSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHBRSTR` writer
pub struct W(crate::W<AHBRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBRSTR_SPEC>;
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
impl From<crate::W<AHBRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBRSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMA1RST` reader - DMA1 and DMAMUX reset Set and cleared by software.
pub type DMA1RST_R = crate::BitReader<bool>;
///Field `DMA1RST` writer - DMA1 and DMAMUX reset Set and cleared by software.
pub type DMA1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
///Field `FLASHRST` reader - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode.
pub type FLASHRST_R = crate::BitReader<bool>;
///Field `FLASHRST` writer - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode.
pub type FLASHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
///Field `CRCRST` reader - CRC reset Set and cleared by software.
pub type CRCRST_R = crate::BitReader<bool>;
///Field `CRCRST` writer - CRC reset Set and cleared by software.
pub type CRCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
impl R {
    ///Bit 0 - DMA1 and DMAMUX reset Set and cleared by software.
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode.
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC reset Set and cleared by software.
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DMA1 and DMAMUX reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<0> {
        DMA1RST_W::new(self)
    }
    ///Bit 8 - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode.
    #[inline(always)]
    #[must_use]
    pub fn flashrst(&mut self) -> FLASHRST_W<8> {
        FLASHRST_W::new(self)
    }
    ///Bit 12 - CRC reset Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<12> {
        CRCRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahbrstr](index.html) module
pub struct AHBRSTR_SPEC;
impl crate::RegisterSpec for AHBRSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahbrstr::R](R) reader structure
impl crate::Readable for AHBRSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahbrstr::W](W) writer structure
impl crate::Writable for AHBRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHBRSTR to value 0
impl crate::Resettable for AHBRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
