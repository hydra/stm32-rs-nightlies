///Register `GICD_IIDR` reader
pub struct R(crate::R<GICD_IIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `IMPLEMENTER` reader - IMPLEMENTER
pub type IMPLEMENTER_R = crate::FieldReader<u16, u16>;
///Field `VARIANT` reader - VARIANT
pub type VARIANT_R = crate::FieldReader<u8, u8>;
///Field `REVISION` reader - REVISION
pub type REVISION_R = crate::FieldReader<u8, u8>;
///Field `PRODUCTID` reader - PRODUCTID
pub type PRODUCTID_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:11 - IMPLEMENTER
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:15 - VARIANT
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - REVISION
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:31 - PRODUCTID
    #[inline(always)]
    pub fn productid(&self) -> PRODUCTID_R {
        PRODUCTID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
///GICD implementer identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicd_iidr](index.html) module
pub struct GICD_IIDR_SPEC;
impl crate::RegisterSpec for GICD_IIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicd_iidr::R](R) reader structure
impl crate::Readable for GICD_IIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets GICD_IIDR to value 0x0100_143b
impl crate::Resettable for GICD_IIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_143b;
}
