///Register `ITENR` reader
pub struct R(crate::R<ITENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ITENR` writer
pub struct W(crate::W<ITENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITENR_SPEC>;
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
impl From<crate::W<ITENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TS1_ITEEN` reader - Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement.
pub type TS1_ITEEN_R = crate::BitReader<bool>;
///Field `TS1_ITEEN` writer - Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement.
pub type TS1_ITEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ITENR_SPEC, bool, O>;
///Field `TS1_ITLEN` reader - Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold.
pub type TS1_ITLEN_R = crate::BitReader<bool>;
///Field `TS1_ITLEN` writer - Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold.
pub type TS1_ITLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ITENR_SPEC, bool, O>;
///Field `TS1_ITHEN` reader - Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold.
pub type TS1_ITHEN_R = crate::BitReader<bool>;
///Field `TS1_ITHEN` writer - Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold.
pub type TS1_ITHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ITENR_SPEC, bool, O>;
///Field `TS1_AITEEN` reader - Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1).
pub type TS1_AITEEN_R = crate::BitReader<bool>;
///Field `TS1_AITEEN` writer - Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1).
pub type TS1_AITEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ITENR_SPEC, bool, O>;
///Field `TS1_AITLEN` reader - Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1)
pub type TS1_AITLEN_R = crate::BitReader<bool>;
///Field `TS1_AITLEN` writer - Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1)
pub type TS1_AITLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ITENR_SPEC, bool, O>;
///Field `TS1_AITHEN` reader - Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1’’)
pub type TS1_AITHEN_R = crate::BitReader<bool>;
///Field `TS1_AITHEN` writer - Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1’’)
pub type TS1_AITHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ITENR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement.
    #[inline(always)]
    pub fn ts1_iteen(&self) -> TS1_ITEEN_R {
        TS1_ITEEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold.
    #[inline(always)]
    pub fn ts1_itlen(&self) -> TS1_ITLEN_R {
        TS1_ITLEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold.
    #[inline(always)]
    pub fn ts1_ithen(&self) -> TS1_ITHEN_R {
        TS1_ITHEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1).
    #[inline(always)]
    pub fn ts1_aiteen(&self) -> TS1_AITEEN_R {
        TS1_AITEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1)
    #[inline(always)]
    pub fn ts1_aitlen(&self) -> TS1_AITLEN_R {
        TS1_AITLEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1’’)
    #[inline(always)]
    pub fn ts1_aithen(&self) -> TS1_AITHEN_R {
        TS1_AITHEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Interrupt enable flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt for end of measurement.
    #[inline(always)]
    #[must_use]
    pub fn ts1_iteen(&mut self) -> TS1_ITEEN_W<0> {
        TS1_ITEEN_W::new(self)
    }
    ///Bit 1 - Interrupt enable flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the synchronous interrupt when the measure reaches or is below the low threshold.
    #[inline(always)]
    #[must_use]
    pub fn ts1_itlen(&mut self) -> TS1_ITLEN_W<1> {
        TS1_ITLEN_W::new(self)
    }
    ///Bit 2 - Interrupt enable flag for high threshold on temperature sensor 1, synchronized on PCLK. This bit are set and cleared by software. It enables the interrupt when the measure reaches or is above the high threshold.
    #[inline(always)]
    #[must_use]
    pub fn ts1_ithen(&mut self) -> TS1_ITHEN_W<2> {
        TS1_ITHEN_W::new(self)
    }
    ///Bit 4 - Asynchronous interrupt enable flag for end of measurement on temperature sensor 1 This bit are set and cleared by software. It enables the asynchronous interrupt for end of measurement (only when REFCLK_SEL = 1).
    #[inline(always)]
    #[must_use]
    pub fn ts1_aiteen(&mut self) -> TS1_AITEEN_W<4> {
        TS1_AITEEN_W::new(self)
    }
    ///Bit 5 - Asynchronous interrupt enable flag for low threshold on temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is below the low threshold (only when REFCLK_SEL= 1)
    #[inline(always)]
    #[must_use]
    pub fn ts1_aitlen(&mut self) -> TS1_AITLEN_W<5> {
        TS1_AITLEN_W::new(self)
    }
    ///Bit 6 - Asynchronous interrupt enable flag on high threshold for temperature sensor 1. This bit are set and cleared by software. It enables the asynchronous interrupt when the temperature is above the high threshold (only when REFCLK_SEL= 1’’)
    #[inline(always)]
    #[must_use]
    pub fn ts1_aithen(&mut self) -> TS1_AITHEN_W<6> {
        TS1_AITHEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Temperature sensor interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itenr](index.html) module
pub struct ITENR_SPEC;
impl crate::RegisterSpec for ITENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [itenr::R](R) reader structure
impl crate::Readable for ITENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [itenr::W](W) writer structure
impl crate::Writable for ITENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ITENR to value 0
impl crate::Resettable for ITENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
