///Register `GRXSTSP_DEVICE` reader
pub struct R(crate::R<GRXSTSP_DEVICE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRXSTSP_DEVICE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRXSTSP_DEVICE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRXSTSP_DEVICE_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EPNUM` reader - EPNUM
pub type EPNUM_R = crate::FieldReader<u8, u8>;
///Field `BCNT` reader - BCNT
pub type BCNT_R = crate::FieldReader<u16, u16>;
///Field `DPID` reader - DPID
pub type DPID_R = crate::FieldReader<u8, u8>;
///Field `PKTSTS` reader - PKTSTS
pub type PKTSTS_R = crate::FieldReader<u8, u8>;
///Field `FRMNUM` reader - FRMNUM
pub type FRMNUM_R = crate::FieldReader<u8, u8>;
///Field `STSPHST` reader - STSPHST
pub type STSPHST_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:3 - EPNUM
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:14 - BCNT
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    ///Bits 15:16 - DPID
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bits 17:20 - PKTSTS
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    ///Bits 21:24 - FRMNUM
    #[inline(always)]
    pub fn frmnum(&self) -> FRMNUM_R {
        FRMNUM_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    ///Bit 27 - STSPHST
    #[inline(always)]
    pub fn stsphst(&self) -> STSPHST_R {
        STSPHST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
///This description is for register GRXSTSP in Device mode. Similarly to GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in GINTSTS) is asserted.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [grxstsp_device](index.html) module
pub struct GRXSTSP_DEVICE_SPEC;
impl crate::RegisterSpec for GRXSTSP_DEVICE_SPEC {
    type Ux = u32;
}
///`read()` method returns [grxstsp_device::R](R) reader structure
impl crate::Readable for GRXSTSP_DEVICE_SPEC {
    type Reader = R;
}
///`reset()` method sets GRXSTSP_DEVICE to value 0
impl crate::Resettable for GRXSTSP_DEVICE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
