///Register `PWR_UCPDR` reader
pub struct R(crate::R<PWR_UCPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_UCPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_UCPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_UCPDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PWR_UCPDR` writer
pub struct W(crate::W<PWR_UCPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_UCPDR_SPEC>;
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
impl From<crate::W<PWR_UCPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_UCPDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `UCPD_DBDIS` reader - UCPD dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable).
pub type UCPD_DBDIS_R = crate::BitReader<bool>;
///Field `UCPD_DBDIS` writer - UCPD dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable).
pub type UCPD_DBDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_UCPDR_SPEC, bool, O>;
///Field `UCPD_STBY` reader - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers.
pub type UCPD_STBY_R = crate::BitReader<bool>;
///Field `UCPD_STBY` writer - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers.
pub type UCPD_STBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_UCPDR_SPEC, bool, O>;
impl R {
    ///Bit 0 - UCPD dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable).
    #[inline(always)]
    pub fn ucpd_dbdis(&self) -> UCPD_DBDIS_R {
        UCPD_DBDIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers.
    #[inline(always)]
    pub fn ucpd_stby(&self) -> UCPD_STBY_R {
        UCPD_STBY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - UCPD dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable).
    #[inline(always)]
    #[must_use]
    pub fn ucpd_dbdis(&mut self) -> UCPD_DBDIS_W<0> {
        UCPD_DBDIS_W::new(self)
    }
    ///Bit 1 - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers.
    #[inline(always)]
    #[must_use]
    pub fn ucpd_stby(&mut self) -> UCPD_STBY_W<1> {
        UCPD_STBY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR USB Type-C™ and Power Delivery register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_ucpdr](index.html) module
pub struct PWR_UCPDR_SPEC;
impl crate::RegisterSpec for PWR_UCPDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_ucpdr::R](R) reader structure
impl crate::Readable for PWR_UCPDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pwr_ucpdr::W](W) writer structure
impl crate::Writable for PWR_UCPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PWR_UCPDR to value 0
impl crate::Resettable for PWR_UCPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
