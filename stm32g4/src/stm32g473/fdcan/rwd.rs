///Register `RWD` reader
pub struct R(crate::R<RWD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWD_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RWD` writer
pub struct W(crate::W<RWD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RWD_SPEC>;
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
impl From<crate::W<RWD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RWD_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WDC` reader - WDC
pub type WDC_R = crate::FieldReader<u8, u8>;
///Field `WDC` writer - WDC
pub type WDC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RWD_SPEC, u8, u8, 8, O>;
///Field `WDV` reader - WDV
pub type WDV_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - WDC
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - WDV
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - WDC
    #[inline(always)]
    #[must_use]
    pub fn wdc(&mut self) -> WDC_W<0> {
        WDC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD\[WDC\]
///bits. The counter is reloaded with RWD\[WDC\]
///bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR\[WDI\]
///bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rwd](index.html) module
pub struct RWD_SPEC;
impl crate::RegisterSpec for RWD_SPEC {
    type Ux = u32;
}
///`read()` method returns [rwd::R](R) reader structure
impl crate::Readable for RWD_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rwd::W](W) writer structure
impl crate::Writable for RWD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RWD to value 0
impl crate::Resettable for RWD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
