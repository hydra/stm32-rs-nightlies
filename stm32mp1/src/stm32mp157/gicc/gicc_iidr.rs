///Register `GICC_IIDR` reader
pub struct R(crate::R<GICC_IIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICC_IIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICC_IIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICC_IIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `IMPLEMENTER` reader - IMPLEMENTER
pub type IMPLEMENTER_R = crate::FieldReader<u16, u16>;
///Field `REVISION` reader - REVISION
pub type REVISION_R = crate::FieldReader<u8, u8>;
///Field `ARCH` reader - ARCH
pub type ARCH_R = crate::FieldReader<u8, u8>;
///Field `PRODUCTID` reader - PRODUCTID
pub type PRODUCTID_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:11 - IMPLEMENTER
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:15 - REVISION
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - ARCH
    #[inline(always)]
    pub fn arch(&self) -> ARCH_R {
        ARCH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:31 - PRODUCTID
    #[inline(always)]
    pub fn productid(&self) -> PRODUCTID_R {
        PRODUCTID_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
///GICC interface identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicc_iidr](index.html) module
pub struct GICC_IIDR_SPEC;
impl crate::RegisterSpec for GICC_IIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicc_iidr::R](R) reader structure
impl crate::Readable for GICC_IIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets GICC_IIDR to value 0x0102_143b
impl crate::Resettable for GICC_IIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0102_143b;
}
