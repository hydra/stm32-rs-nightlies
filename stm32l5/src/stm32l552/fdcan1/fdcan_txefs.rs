///Register `FDCAN_TXEFS` reader
pub struct R(crate::R<FDCAN_TXEFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXEFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXEFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXEFS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EFFL` reader - Event FIFO Fill Level
pub type EFFL_R = crate::FieldReader<u8, u8>;
///Field `EFGI` reader - Event FIFO Get Index.
pub type EFGI_R = crate::FieldReader<u8, u8>;
///Field `EFPI` reader - Event FIFO Put Index
pub type EFPI_R = crate::FieldReader<u8, u8>;
///Field `EFF` reader - Event FIFO Full.
pub type EFF_R = crate::BitReader<bool>;
///Field `TEFL` reader - Tx Event FIFO Element Lost.
pub type TEFL_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:2 - Event FIFO Fill Level
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:9 - Event FIFO Get Index.
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - Event FIFO Put Index
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 24 - Event FIFO Full.
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Tx Event FIFO Element Lost.
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
///FDCAN Tx Event FIFO Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_txefs](index.html) module
pub struct FDCAN_TXEFS_SPEC;
impl crate::RegisterSpec for FDCAN_TXEFS_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_txefs::R](R) reader structure
impl crate::Readable for FDCAN_TXEFS_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_TXEFS to value 0
impl crate::Resettable for FDCAN_TXEFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
