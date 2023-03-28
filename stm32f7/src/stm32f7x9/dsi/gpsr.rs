///Register `GPSR` reader
pub struct R(crate::R<GPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CMDFE` reader - Command FIFO Empty
pub type CMDFE_R = crate::BitReader<bool>;
///Field `CMDFF` reader - Command FIFO Full
pub type CMDFF_R = crate::BitReader<bool>;
///Field `PWRFE` reader - Payload Write FIFO Empty
pub type PWRFE_R = crate::BitReader<bool>;
///Field `PWRFF` reader - Payload Write FIFO Full
pub type PWRFF_R = crate::BitReader<bool>;
///Field `PRDFE` reader - Payload Read FIFO Empty
pub type PRDFE_R = crate::BitReader<bool>;
///Field `PRDFF` reader - Payload Read FIFO Full
pub type PRDFF_R = crate::BitReader<bool>;
///Field `RCB` reader - Read Command Busy
pub type RCB_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Command FIFO Empty
    #[inline(always)]
    pub fn cmdfe(&self) -> CMDFE_R {
        CMDFE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Command FIFO Full
    #[inline(always)]
    pub fn cmdff(&self) -> CMDFF_R {
        CMDFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Payload Write FIFO Empty
    #[inline(always)]
    pub fn pwrfe(&self) -> PWRFE_R {
        PWRFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Payload Write FIFO Full
    #[inline(always)]
    pub fn pwrff(&self) -> PWRFF_R {
        PWRFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Payload Read FIFO Empty
    #[inline(always)]
    pub fn prdfe(&self) -> PRDFE_R {
        PRDFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Payload Read FIFO Full
    #[inline(always)]
    pub fn prdff(&self) -> PRDFF_R {
        PRDFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Read Command Busy
    #[inline(always)]
    pub fn rcb(&self) -> RCB_R {
        RCB_R::new(((self.bits >> 6) & 1) != 0)
    }
}
///DSI Host Generic Packet Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpsr](index.html) module
pub struct GPSR_SPEC;
impl crate::RegisterSpec for GPSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpsr::R](R) reader structure
impl crate::Readable for GPSR_SPEC {
    type Reader = R;
}
///`reset()` method sets GPSR to value 0
impl crate::Resettable for GPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
