///Register `TIM5_CR1` reader
pub struct R(crate::R<TIM5_CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM5_CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM5_CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM5_CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM5_CR1` writer
pub struct W(crate::W<TIM5_CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM5_CR1_SPEC>;
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
impl From<crate::W<TIM5_CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM5_CR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CEN` reader - Counter enable Note: External clock, gated mode and encoder mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware. CEN is cleared automatically in one-pulse mode, when an update event occurs.
pub type CEN_R = crate::BitReader<bool>;
///Field `CEN` writer - Counter enable Note: External clock, gated mode and encoder mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware. CEN is cleared automatically in one-pulse mode, when an update event occurs.
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_CR1_SPEC, bool, O>;
///Field `UDIS` reader - Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values.
pub type UDIS_R = crate::BitReader<bool>;
///Field `UDIS` writer - Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values.
pub type UDIS_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_CR1_SPEC, bool, O>;
///Field `URS` reader - Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller
pub type URS_R = crate::BitReader<bool>;
///Field `URS` writer - Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller
pub type URS_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_CR1_SPEC, bool, O>;
///Field `OPM` reader - One-pulse mode
pub type OPM_R = crate::BitReader<bool>;
///Field `OPM` writer - One-pulse mode
pub type OPM_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_CR1_SPEC, bool, O>;
///Field `DIR` reader - Direction Note: This bit is read only when the timer is configured in Center-aligned mode or Encoder mode.
pub type DIR_R = crate::BitReader<bool>;
///Field `DIR` writer - Direction Note: This bit is read only when the timer is configured in Center-aligned mode or Encoder mode.
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_CR1_SPEC, bool, O>;
///Field `CMS` reader - Center-aligned mode selection Note: It is not allowed to switch from edge-aligned mode to center-aligned mode as long as the counter is enabled (CEN=1)
pub type CMS_R = crate::FieldReader<u8, u8>;
///Field `CMS` writer - Center-aligned mode selection Note: It is not allowed to switch from edge-aligned mode to center-aligned mode as long as the counter is enabled (CEN=1)
pub type CMS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM5_CR1_SPEC, u8, u8, 2, O>;
///Field `ARPE` reader - Auto-reload preload enable
pub type ARPE_R = crate::BitReader<bool>;
///Field `ARPE` writer - Auto-reload preload enable
pub type ARPE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_CR1_SPEC, bool, O>;
///Field `CKD` reader - Clock division This bit-field indicates the division ratio between the timer clock (tim_ker_ck) frequency and sampling clock used by the digital filters (tim_etr_in, tim_tix),
pub type CKD_R = crate::FieldReader<u8, u8>;
///Field `CKD` writer - Clock division This bit-field indicates the division ratio between the timer clock (tim_ker_ck) frequency and sampling clock used by the digital filters (tim_etr_in, tim_tix),
pub type CKD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM5_CR1_SPEC, u8, u8, 2, O>;
///Field `UIFREMAP` reader - UIF status bit remapping
pub type UIFREMAP_R = crate::BitReader<bool>;
///Field `UIFREMAP` writer - UIF status bit remapping
pub type UIFREMAP_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_CR1_SPEC, bool, O>;
///Field `DITHEN` reader - Dithering Enable Note: The DITHEN bit can only be modified when CEN bit is reset.
pub type DITHEN_R = crate::BitReader<bool>;
///Field `DITHEN` writer - Dithering Enable Note: The DITHEN bit can only be modified when CEN bit is reset.
pub type DITHEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_CR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - Counter enable Note: External clock, gated mode and encoder mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware. CEN is cleared automatically in one-pulse mode, when an update event occurs.
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values.
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - One-pulse mode
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Direction Note: This bit is read only when the timer is configured in Center-aligned mode or Encoder mode.
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Center-aligned mode selection Note: It is not allowed to switch from edge-aligned mode to center-aligned mode as long as the counter is enabled (CEN=1)
    #[inline(always)]
    pub fn cms(&self) -> CMS_R {
        CMS_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Auto-reload preload enable
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Clock division This bit-field indicates the division ratio between the timer clock (tim_ker_ck) frequency and sampling clock used by the digital filters (tim_etr_in, tim_tix),
    #[inline(always)]
    pub fn ckd(&self) -> CKD_R {
        CKD_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - UIF status bit remapping
    #[inline(always)]
    pub fn uifremap(&self) -> UIFREMAP_R {
        UIFREMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Dithering Enable Note: The DITHEN bit can only be modified when CEN bit is reset.
    #[inline(always)]
    pub fn dithen(&self) -> DITHEN_R {
        DITHEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Counter enable Note: External clock, gated mode and encoder mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware. CEN is cleared automatically in one-pulse mode, when an update event occurs.
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<0> {
        CEN_W::new(self)
    }
    ///Bit 1 - Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values.
    #[inline(always)]
    #[must_use]
    pub fn udis(&mut self) -> UDIS_W<1> {
        UDIS_W::new(self)
    }
    ///Bit 2 - Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller
    #[inline(always)]
    #[must_use]
    pub fn urs(&mut self) -> URS_W<2> {
        URS_W::new(self)
    }
    ///Bit 3 - One-pulse mode
    #[inline(always)]
    #[must_use]
    pub fn opm(&mut self) -> OPM_W<3> {
        OPM_W::new(self)
    }
    ///Bit 4 - Direction Note: This bit is read only when the timer is configured in Center-aligned mode or Encoder mode.
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<4> {
        DIR_W::new(self)
    }
    ///Bits 5:6 - Center-aligned mode selection Note: It is not allowed to switch from edge-aligned mode to center-aligned mode as long as the counter is enabled (CEN=1)
    #[inline(always)]
    #[must_use]
    pub fn cms(&mut self) -> CMS_W<5> {
        CMS_W::new(self)
    }
    ///Bit 7 - Auto-reload preload enable
    #[inline(always)]
    #[must_use]
    pub fn arpe(&mut self) -> ARPE_W<7> {
        ARPE_W::new(self)
    }
    ///Bits 8:9 - Clock division This bit-field indicates the division ratio between the timer clock (tim_ker_ck) frequency and sampling clock used by the digital filters (tim_etr_in, tim_tix),
    #[inline(always)]
    #[must_use]
    pub fn ckd(&mut self) -> CKD_W<8> {
        CKD_W::new(self)
    }
    ///Bit 11 - UIF status bit remapping
    #[inline(always)]
    #[must_use]
    pub fn uifremap(&mut self) -> UIFREMAP_W<11> {
        UIFREMAP_W::new(self)
    }
    ///Bit 12 - Dithering Enable Note: The DITHEN bit can only be modified when CEN bit is reset.
    #[inline(always)]
    #[must_use]
    pub fn dithen(&mut self) -> DITHEN_W<12> {
        DITHEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM5 control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim5_cr1](index.html) module
pub struct TIM5_CR1_SPEC;
impl crate::RegisterSpec for TIM5_CR1_SPEC {
    type Ux = u16;
}
///`read()` method returns [tim5_cr1::R](R) reader structure
impl crate::Readable for TIM5_CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim5_cr1::W](W) writer structure
impl crate::Writable for TIM5_CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM5_CR1 to value 0
impl crate::Resettable for TIM5_CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
