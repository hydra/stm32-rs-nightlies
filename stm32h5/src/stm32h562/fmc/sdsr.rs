///Register `SDSR` reader
pub struct R(crate::R<SDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RE` reader - Refresh error flag An interrupt is generated if REIE = 1 and RE = 1
pub type RE_R = crate::BitReader<bool>;
///Field `MODES1` reader - Status Mode for Bank 1 This bit defines the Status Mode of SDRAM Bank 1.
pub type MODES1_R = crate::FieldReader<u8, u8>;
///Field `MODES2` reader - Status Mode for Bank 2 This bit defines the Status Mode of SDRAM Bank 2.
pub type MODES2_R = crate::FieldReader<u8, u8>;
///Field `BUSY` reader - Busy status This bit defines the status of the SDRAM controller after a Command Mode request 1; SDRAM Controller is not ready to accept a new request
pub type BUSY_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Refresh error flag An interrupt is generated if REIE = 1 and RE = 1
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Status Mode for Bank 1 This bit defines the Status Mode of SDRAM Bank 1.
    #[inline(always)]
    pub fn modes1(&self) -> MODES1_R {
        MODES1_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - Status Mode for Bank 2 This bit defines the Status Mode of SDRAM Bank 2.
    #[inline(always)]
    pub fn modes2(&self) -> MODES2_R {
        MODES2_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - Busy status This bit defines the status of the SDRAM controller after a Command Mode request 1; SDRAM Controller is not ready to accept a new request
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
///SDRAM status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sdsr](index.html) module
pub struct SDSR_SPEC;
impl crate::RegisterSpec for SDSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sdsr::R](R) reader structure
impl crate::Readable for SDSR_SPEC {
    type Reader = R;
}
///`reset()` method sets SDSR to value 0
impl crate::Resettable for SDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
