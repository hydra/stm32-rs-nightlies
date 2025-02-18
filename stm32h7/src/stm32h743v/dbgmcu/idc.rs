///Register `IDC` reader
pub struct R(crate::R<IDC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDC_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DEV_ID` reader - Device ID
pub type DEV_ID_R = crate::FieldReader<u16, u16>;
///Field `REV_ID` reader - Revision
pub type REV_ID_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:11 - Device ID
    #[inline(always)]
    pub fn dev_id(&self) -> DEV_ID_R {
        DEV_ID_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:31 - Revision
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///DBGMCU Identity Code Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [idc](index.html) module
pub struct IDC_SPEC;
impl crate::RegisterSpec for IDC_SPEC {
    type Ux = u32;
}
///`read()` method returns [idc::R](R) reader structure
impl crate::Readable for IDC_SPEC {
    type Reader = R;
}
///`reset()` method sets IDC to value 0x1000_6450
impl crate::Resettable for IDC_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_6450;
}
