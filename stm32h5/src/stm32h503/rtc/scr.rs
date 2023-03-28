///Register `SCR` writer
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CALRAF` writer - Clear alarm A flag Writing 1 in this bit clears the ALRAF bit in the RTC_SR register.
pub type CALRAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `CALRBF` writer - Clear alarm B flag Writing 1 in this bit clears the ALRBF bit in the RTC_SR register.
pub type CALRBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `CWUTF` writer - Clear wakeup timer flag Writing 1 in this bit clears the WUTF bit in the RTC_SR register.
pub type CWUTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `CTSF` writer - Clear timestamp flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. If ITSF flag is set, TSF must be cleared together with ITSF by setting CRSF and CITSF.
pub type CTSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `CTSOVF` writer - Clear timestamp overflow flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
pub type CTSOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `CITSF` writer - Clear internal timestamp flag Writing 1 in this bit clears the ITSF bit in the RTC_SR register.
pub type CITSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `CSSRUF` writer - Clear SSR underflow flag Writing ‘1’ in this bit clears the SSRUF in the RTC_SR register.
pub type CSSRUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
impl W {
    ///Bit 0 - Clear alarm A flag Writing 1 in this bit clears the ALRAF bit in the RTC_SR register.
    #[inline(always)]
    #[must_use]
    pub fn calraf(&mut self) -> CALRAF_W<0> {
        CALRAF_W::new(self)
    }
    ///Bit 1 - Clear alarm B flag Writing 1 in this bit clears the ALRBF bit in the RTC_SR register.
    #[inline(always)]
    #[must_use]
    pub fn calrbf(&mut self) -> CALRBF_W<1> {
        CALRBF_W::new(self)
    }
    ///Bit 2 - Clear wakeup timer flag Writing 1 in this bit clears the WUTF bit in the RTC_SR register.
    #[inline(always)]
    #[must_use]
    pub fn cwutf(&mut self) -> CWUTF_W<2> {
        CWUTF_W::new(self)
    }
    ///Bit 3 - Clear timestamp flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. If ITSF flag is set, TSF must be cleared together with ITSF by setting CRSF and CITSF.
    #[inline(always)]
    #[must_use]
    pub fn ctsf(&mut self) -> CTSF_W<3> {
        CTSF_W::new(self)
    }
    ///Bit 4 - Clear timestamp overflow flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
    #[inline(always)]
    #[must_use]
    pub fn ctsovf(&mut self) -> CTSOVF_W<4> {
        CTSOVF_W::new(self)
    }
    ///Bit 5 - Clear internal timestamp flag Writing 1 in this bit clears the ITSF bit in the RTC_SR register.
    #[inline(always)]
    #[must_use]
    pub fn citsf(&mut self) -> CITSF_W<5> {
        CITSF_W::new(self)
    }
    ///Bit 6 - Clear SSR underflow flag Writing ‘1’ in this bit clears the SSRUF in the RTC_SR register.
    #[inline(always)]
    #[must_use]
    pub fn cssruf(&mut self) -> CSSRUF_W<6> {
        CSSRUF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC status clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [scr](index.html) module
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [scr::W](W) writer structure
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
