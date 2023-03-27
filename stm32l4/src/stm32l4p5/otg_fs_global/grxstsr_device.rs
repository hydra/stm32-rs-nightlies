///Register `GRXSTSR_Device` reader
pub struct R(crate::R<GRXSTSR_DEVICE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRXSTSR_DEVICE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRXSTSR_DEVICE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRXSTSR_DEVICE_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EPNUM` reader - Endpoint number
pub type EPNUM_R = crate::FieldReader<u8, u8>;
///Field `BCNT` reader - Byte count
pub type BCNT_R = crate::FieldReader<u16, u16>;
///Field `DPID` reader - Data PID
pub type DPID_R = crate::FieldReader<u8, u8>;
///Field `PKTSTS` reader - Packet status
pub type PKTSTS_R = crate::FieldReader<u8, u8>;
///Field `FRMNUM` reader - Frame number
pub type FRMNUM_R = crate::FieldReader<u8, u8>;
///Field `STSPHST` reader - Status phase start
pub type STSPHST_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:3 - Endpoint number
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:14 - Byte count
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    ///Bits 15:16 - Data PID
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bits 17:20 - Packet status
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    ///Bits 21:24 - Frame number
    #[inline(always)]
    pub fn frmnum(&self) -> FRMNUM_R {
        FRMNUM_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    ///Bit 27 - Status phase start
    #[inline(always)]
    pub fn stsphst(&self) -> STSPHST_R {
        STSPHST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
///OTG_FS Receive status debug read(Device mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [grxstsr_device](index.html) module
pub struct GRXSTSR_DEVICE_SPEC;
impl crate::RegisterSpec for GRXSTSR_DEVICE_SPEC {
    type Ux = u32;
}
///`read()` method returns [grxstsr_device::R](R) reader structure
impl crate::Readable for GRXSTSR_DEVICE_SPEC {
    type Reader = R;
}
///`reset()` method sets GRXSTSR_Device to value 0
impl crate::Resettable for GRXSTSR_DEVICE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
