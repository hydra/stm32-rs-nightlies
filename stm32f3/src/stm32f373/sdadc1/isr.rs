///Register `ISR` reader
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EOCALF` reader - End of calibration flag
pub type EOCALF_R = crate::BitReader<bool>;
///Field `JEOCF` reader - End of injected conversion flag
pub type JEOCF_R = crate::BitReader<bool>;
///Field `JOVRF` reader - Injected conversion overrun flag
pub type JOVRF_R = crate::BitReader<bool>;
///Field `REOCF` reader - End of regular conversion flag
pub type REOCF_R = crate::BitReader<bool>;
///Field `ROVRF` reader - Regular conversion overrun flag
pub type ROVRF_R = crate::BitReader<bool>;
///Field `CALIBIP` reader - Calibration in progress status
pub type CALIBIP_R = crate::BitReader<bool>;
///Field `JCIP` reader - Injected conversion in progress status
pub type JCIP_R = crate::BitReader<bool>;
///Field `RCIP` reader - Regular conversion in progress status
pub type RCIP_R = crate::BitReader<bool>;
///Field `STABIP` reader - Stabilization in progress status
pub type STABIP_R = crate::BitReader<bool>;
///Field `INITRDY` reader - Initialization mode is ready
pub type INITRDY_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - End of calibration flag
    #[inline(always)]
    pub fn eocalf(&self) -> EOCALF_R {
        EOCALF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of injected conversion flag
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected conversion overrun flag
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of regular conversion flag
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Regular conversion overrun flag
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 12 - Calibration in progress status
    #[inline(always)]
    pub fn calibip(&self) -> CALIBIP_R {
        CALIBIP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Injected conversion in progress status
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Regular conversion in progress status
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Stabilization in progress status
    #[inline(always)]
    pub fn stabip(&self) -> STABIP_R {
        STABIP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 31 - Initialization mode is ready
    #[inline(always)]
    pub fn initrdy(&self) -> INITRDY_R {
        INITRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
///interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr](index.html) module
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [isr::R](R) reader structure
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
