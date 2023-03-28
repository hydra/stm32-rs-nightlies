///Register `FDCAN_RWD` reader
pub struct R(crate::R<FDCAN_RWD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RWD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RWD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RWD_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FDCAN_RWD` writer
pub struct W(crate::W<FDCAN_RWD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_RWD_SPEC>;
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
impl From<crate::W<FDCAN_RWD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_RWD_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WDC` reader - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of FDCAN_CCCR register are set to 1.
pub type WDC_R = crate::FieldReader<u8, u8>;
///Field `WDC` writer - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of FDCAN_CCCR register are set to 1.
pub type WDC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_RWD_SPEC, u8, u8, 8, O>;
///Field `WDV` reader - Watchdog value Actual message RAM watchdog counter value.
pub type WDV_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of FDCAN_CCCR register are set to 1.
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Watchdog value Actual message RAM watchdog counter value.
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of FDCAN_CCCR register are set to 1.
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
///FDCAN RAM watchdog register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_rwd](index.html) module
pub struct FDCAN_RWD_SPEC;
impl crate::RegisterSpec for FDCAN_RWD_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_rwd::R](R) reader structure
impl crate::Readable for FDCAN_RWD_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fdcan_rwd::W](W) writer structure
impl crate::Writable for FDCAN_RWD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FDCAN_RWD to value 0
impl crate::Resettable for FDCAN_RWD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
