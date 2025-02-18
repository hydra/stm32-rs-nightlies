///Register `RX_ORDSET` reader
pub struct R(crate::R<RX_ORDSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_ORDSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_ORDSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_ORDSET_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXORDSET` reader - RXORDSET
pub type RXORDSET_R = crate::FieldReader<u8, u8>;
///Field `RXSOP3OF4` reader - RXSOP3OF4
pub type RXSOP3OF4_R = crate::BitReader<bool>;
///Field `RXSOPKINVALID` reader - RXSOPKINVALID
pub type RXSOPKINVALID_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:2 - RXORDSET
    #[inline(always)]
    pub fn rxordset(&self) -> RXORDSET_R {
        RXORDSET_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - RXSOP3OF4
    #[inline(always)]
    pub fn rxsop3of4(&self) -> RXSOP3OF4_R {
        RXSOP3OF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - RXSOPKINVALID
    #[inline(always)]
    pub fn rxsopkinvalid(&self) -> RXSOPKINVALID_R {
        RXSOPKINVALID_R::new(((self.bits >> 4) & 7) as u8)
    }
}
///UCPD Rx Ordered Set Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rx_ordset](index.html) module
pub struct RX_ORDSET_SPEC;
impl crate::RegisterSpec for RX_ORDSET_SPEC {
    type Ux = u32;
}
///`read()` method returns [rx_ordset::R](R) reader structure
impl crate::Readable for RX_ORDSET_SPEC {
    type Reader = R;
}
///`reset()` method sets RX_ORDSET to value 0
impl crate::Resettable for RX_ORDSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
