///Register `GRXSTSR_Host` reader
pub struct R(crate::R<GRXSTSR_HOST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRXSTSR_HOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRXSTSR_HOST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRXSTSR_HOST_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CHNUM` reader - Endpoint number
pub type CHNUM_R = crate::FieldReader<u8, u8>;
///Field `BCNT` reader - Byte count
pub type BCNT_R = crate::FieldReader<u16, u16>;
///Field `DPID` reader - Data PID
pub type DPID_R = crate::FieldReader<u8, u8>;
///Field `PKTSTS` reader - Packet status
pub type PKTSTS_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - Endpoint number
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new((self.bits & 0x0f) as u8)
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
}
///OTG_FS Receive status debug read(Host mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [grxstsr_host](index.html) module
pub struct GRXSTSR_HOST_SPEC;
impl crate::RegisterSpec for GRXSTSR_HOST_SPEC {
    type Ux = u32;
}
///`read()` method returns [grxstsr_host::R](R) reader structure
impl crate::Readable for GRXSTSR_HOST_SPEC {
    type Reader = R;
}
///`reset()` method sets GRXSTSR_Host to value 0
impl crate::Resettable for GRXSTSR_HOST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
