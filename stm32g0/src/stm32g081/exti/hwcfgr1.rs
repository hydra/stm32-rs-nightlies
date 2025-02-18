///Register `HWCFGR1` reader
pub struct R(crate::R<HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `NBEVENTS` reader - configuration number of event
pub type NBEVENTS_R = crate::FieldReader<u8, u8>;
///Field `NBCPUS` reader - configuration number of CPUs
pub type NBCPUS_R = crate::FieldReader<u8, u8>;
///Field `CPUEVTEN` reader - HW configuration of CPU event output enable
pub type CPUEVTEN_R = crate::FieldReader<u8, u8>;
///Field `NBIOPORT` reader - HW configuration of number of IO ports
pub type NBIOPORT_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - configuration number of event
    #[inline(always)]
    pub fn nbevents(&self) -> NBEVENTS_R {
        NBEVENTS_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - configuration number of CPUs
    #[inline(always)]
    pub fn nbcpus(&self) -> NBCPUS_R {
        NBCPUS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - HW configuration of CPU event output enable
    #[inline(always)]
    pub fn cpuevten(&self) -> CPUEVTEN_R {
        CPUEVTEN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:23 - HW configuration of number of IO ports
    #[inline(always)]
    pub fn nbioport(&self) -> NBIOPORT_R {
        NBIOPORT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
///Hardware configuration registers
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hwcfgr1](index.html) module
pub struct HWCFGR1_SPEC;
impl crate::RegisterSpec for HWCFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [hwcfgr1::R](R) reader structure
impl crate::Readable for HWCFGR1_SPEC {
    type Reader = R;
}
///`reset()` method sets HWCFGR1 to value 0x0005_1021
impl crate::Resettable for HWCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0005_1021;
}
