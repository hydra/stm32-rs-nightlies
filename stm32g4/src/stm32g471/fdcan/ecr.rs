///Register `ECR` reader
pub struct R(crate::R<ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TEC` reader - TEC
pub type TEC_R = crate::FieldReader<u8, u8>;
///Field `REC` reader - TREC
pub type REC_R = crate::FieldReader<u8, u8>;
///Field `RP` reader - RP
pub type RP_R = crate::BitReader<bool>;
///Field `CEL` reader - CEL
pub type CEL_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - TEC
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:14 - TREC
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 15 - RP
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - CEL
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
///FDCAN Error Counter Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ecr](index.html) module
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ecr::R](R) reader structure
impl crate::Readable for ECR_SPEC {
    type Reader = R;
}
///`reset()` method sets ECR to value 0
impl crate::Resettable for ECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
