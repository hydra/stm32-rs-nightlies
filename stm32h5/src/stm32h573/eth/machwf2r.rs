///Register `MACHWF2R` reader
pub struct R(crate::R<MACHWF2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACHWF2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACHWF2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACHWF2R_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXQCNT` reader - Number of MTL Receive Queues This field indicates the number of MTL Receive queues: ..
pub type RXQCNT_R = crate::FieldReader<u8, u8>;
///Field `TXQCNT` reader - Number of MTL Transmit Queues This field indicates the number of MTL Transmit queues: ..
pub type TXQCNT_R = crate::FieldReader<u8, u8>;
///Field `RXCHCNT` reader - Number of DMA Receive Channels This field indicates the number of DMA Receive channels: ..
pub type RXCHCNT_R = crate::FieldReader<u8, u8>;
///Field `RDCSZ` reader - Rx DMA Descriptor Cache Size in terms of 16-byte descriptors
pub type RDCSZ_R = crate::FieldReader<u8, u8>;
///Field `TXCHCNT` reader - Number of DMA Transmit Channels This field indicates the number of DMA Transmit channels: ..
pub type TXCHCNT_R = crate::FieldReader<u8, u8>;
///Field `TDCSZ` reader - Tx DMA Descriptor Cache Size in terms of 16-byte descriptors
pub type TDCSZ_R = crate::FieldReader<u8, u8>;
///Field `PPSOUTNUM` reader - Number of PPS Outputs This field indicates the number of PPS outputs: 101 to 111: Reserved, must not be used
pub type PPSOUTNUM_R = crate::FieldReader<u8, u8>;
///Field `AUXSNAPNUM` reader - Number of Auxiliary Snapshot Inputs This field indicates the number of auxiliary snapshot inputs: 101 to 111: Reserved, must not be used
pub type AUXSNAPNUM_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - Number of MTL Receive Queues This field indicates the number of MTL Receive queues: ..
    #[inline(always)]
    pub fn rxqcnt(&self) -> RXQCNT_R {
        RXQCNT_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 6:9 - Number of MTL Transmit Queues This field indicates the number of MTL Transmit queues: ..
    #[inline(always)]
    pub fn txqcnt(&self) -> TXQCNT_R {
        TXQCNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    ///Bits 12:15 - Number of DMA Receive Channels This field indicates the number of DMA Receive channels: ..
    #[inline(always)]
    pub fn rxchcnt(&self) -> RXCHCNT_R {
        RXCHCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:17 - Rx DMA Descriptor Cache Size in terms of 16-byte descriptors
    #[inline(always)]
    pub fn rdcsz(&self) -> RDCSZ_R {
        RDCSZ_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:21 - Number of DMA Transmit Channels This field indicates the number of DMA Transmit channels: ..
    #[inline(always)]
    pub fn txchcnt(&self) -> TXCHCNT_R {
        TXCHCNT_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 22:23 - Tx DMA Descriptor Cache Size in terms of 16-byte descriptors
    #[inline(always)]
    pub fn tdcsz(&self) -> TDCSZ_R {
        TDCSZ_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:26 - Number of PPS Outputs This field indicates the number of PPS outputs: 101 to 111: Reserved, must not be used
    #[inline(always)]
    pub fn ppsoutnum(&self) -> PPSOUTNUM_R {
        PPSOUTNUM_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - Number of Auxiliary Snapshot Inputs This field indicates the number of auxiliary snapshot inputs: 101 to 111: Reserved, must not be used
    #[inline(always)]
    pub fn auxsnapnum(&self) -> AUXSNAPNUM_R {
        AUXSNAPNUM_R::new(((self.bits >> 28) & 7) as u8)
    }
}
///HW feature 2 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [machwf2r](index.html) module
pub struct MACHWF2R_SPEC;
impl crate::RegisterSpec for MACHWF2R_SPEC {
    type Ux = u32;
}
///`read()` method returns [machwf2r::R](R) reader structure
impl crate::Readable for MACHWF2R_SPEC {
    type Reader = R;
}
///`reset()` method sets MACHWF2R to value 0x4100_0000
impl crate::Resettable for MACHWF2R_SPEC {
    const RESET_VALUE: Self::Ux = 0x4100_0000;
}
