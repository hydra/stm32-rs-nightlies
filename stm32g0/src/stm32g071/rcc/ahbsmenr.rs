///Register `AHBSMENR` reader
pub struct R(crate::R<AHBSMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBSMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBSMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBSMENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHBSMENR` writer
pub struct W(crate::W<AHBSMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBSMENR_SPEC>;
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
impl From<crate::W<AHBSMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBSMENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMASMEN` reader - DMA clock enable during Sleep mode
pub type DMASMEN_R = crate::BitReader<bool>;
///Field `DMASMEN` writer - DMA clock enable during Sleep mode
pub type DMASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBSMENR_SPEC, bool, O>;
///Field `FLASHSMEN` reader - Flash memory interface clock enable during Sleep mode
pub type FLASHSMEN_R = crate::BitReader<bool>;
///Field `FLASHSMEN` writer - Flash memory interface clock enable during Sleep mode
pub type FLASHSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBSMENR_SPEC, bool, O>;
///Field `SRAMSMEN` reader - SRAM clock enable during Sleep mode
pub type SRAMSMEN_R = crate::BitReader<bool>;
///Field `SRAMSMEN` writer - SRAM clock enable during Sleep mode
pub type SRAMSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBSMENR_SPEC, bool, O>;
///Field `CRCSMEN` reader - CRC clock enable during Sleep mode
pub type CRCSMEN_R = crate::BitReader<bool>;
///Field `CRCSMEN` writer - CRC clock enable during Sleep mode
pub type CRCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBSMENR_SPEC, bool, O>;
impl R {
    ///Bit 0 - DMA clock enable during Sleep mode
    #[inline(always)]
    pub fn dmasmen(&self) -> DMASMEN_R {
        DMASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clock enable during Sleep mode
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM clock enable during Sleep mode
    #[inline(always)]
    pub fn sramsmen(&self) -> SRAMSMEN_R {
        SRAMSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable during Sleep mode
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DMA clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn dmasmen(&mut self) -> DMASMEN_W<0> {
        DMASMEN_W::new(self)
    }
    ///Bit 8 - Flash memory interface clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<8> {
        FLASHSMEN_W::new(self)
    }
    ///Bit 9 - SRAM clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn sramsmen(&mut self) -> SRAMSMEN_W<9> {
        SRAMSMEN_W::new(self)
    }
    ///Bit 12 - CRC clock enable during Sleep mode
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<12> {
        CRCSMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB peripheral clock enable in Sleep mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahbsmenr](index.html) module
pub struct AHBSMENR_SPEC;
impl crate::RegisterSpec for AHBSMENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahbsmenr::R](R) reader structure
impl crate::Readable for AHBSMENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahbsmenr::W](W) writer structure
impl crate::Writable for AHBSMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHBSMENR to value 0
impl crate::Resettable for AHBSMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
