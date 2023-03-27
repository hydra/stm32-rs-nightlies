///Register `FDCAN_TEST` reader
pub struct R(crate::R<FDCAN_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TEST_SPEC>) -> Self {
        R(reader)
    }
}
///Field `LBCK` reader - Loop Back mode
pub type LBCK_R = crate::BitReader<bool>;
///Field `TX` reader - Loop Back mode
pub type TX_R = crate::FieldReader<u8, u8>;
///Field `RX` reader - Control of Transmit Pin
pub type RX_R = crate::BitReader<bool>;
impl R {
    ///Bit 4 - Loop Back mode
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Loop Back mode
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Control of Transmit Pin
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
///FDCAN Test Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_test](index.html) module
pub struct FDCAN_TEST_SPEC;
impl crate::RegisterSpec for FDCAN_TEST_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_test::R](R) reader structure
impl crate::Readable for FDCAN_TEST_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_TEST to value 0
impl crate::Resettable for FDCAN_TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
