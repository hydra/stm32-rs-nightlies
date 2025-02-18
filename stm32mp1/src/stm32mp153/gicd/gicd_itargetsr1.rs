///Register `GICD_ITARGETSR1` reader
pub struct R(crate::R<GICD_ITARGETSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CPU_TARGETS0` reader - CPU_TARGETS0
pub type CPU_TARGETS0_R = crate::FieldReader<u8, u8>;
///Field `CPU_TARGETS1` reader - CPU_TARGETS1
pub type CPU_TARGETS1_R = crate::FieldReader<u8, u8>;
///Field `CPU_TARGETS2` reader - CPU_TARGETS2
pub type CPU_TARGETS2_R = crate::FieldReader<u8, u8>;
///Field `CPU_TARGETS3` reader - CPU_TARGETS3
pub type CPU_TARGETS3_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:1 - CPU_TARGETS0
    #[inline(always)]
    pub fn cpu_targets0(&self) -> CPU_TARGETS0_R {
        CPU_TARGETS0_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:9 - CPU_TARGETS1
    #[inline(always)]
    pub fn cpu_targets1(&self) -> CPU_TARGETS1_R {
        CPU_TARGETS1_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - CPU_TARGETS2
    #[inline(always)]
    pub fn cpu_targets2(&self) -> CPU_TARGETS2_R {
        CPU_TARGETS2_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 24:25 - CPU_TARGETS3
    #[inline(always)]
    pub fn cpu_targets3(&self) -> CPU_TARGETS3_R {
        CPU_TARGETS3_R::new(((self.bits >> 24) & 3) as u8)
    }
}
///For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicd_itargetsr1](index.html) module
pub struct GICD_ITARGETSR1_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicd_itargetsr1::R](R) reader structure
impl crate::Readable for GICD_ITARGETSR1_SPEC {
    type Reader = R;
}
///`reset()` method sets GICD_ITARGETSR1 to value 0
impl crate::Resettable for GICD_ITARGETSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
