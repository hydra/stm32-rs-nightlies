///Register `FDCAN_TXFQS` reader
pub struct R(crate::R<FDCAN_TXFQS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXFQS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXFQS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXFQS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TFFL` reader - Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\[TFQM\]
///= 1).
pub type TFFL_R = crate::FieldReader<u8, u8>;
///Field `TFGI` reader - Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)
pub type TFGI_R = crate::FieldReader<u8, u8>;
///Field `TFQPI` reader - Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3
pub type TFQPI_R = crate::FieldReader<u8, u8>;
///Field `TFQF` reader - Tx FIFO/queue full
pub type TFQF_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:2 - Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\[TFQM\]
    ///= 1).
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:9 - Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 21 - Tx FIFO/queue full
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
///FDCAN Tx FIFO/queue status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_txfqs](index.html) module
pub struct FDCAN_TXFQS_SPEC;
impl crate::RegisterSpec for FDCAN_TXFQS_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_txfqs::R](R) reader structure
impl crate::Readable for FDCAN_TXFQS_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_TXFQS to value 0x03
impl crate::Resettable for FDCAN_TXFQS_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
