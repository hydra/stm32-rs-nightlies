///Register `GICV_HPPIR` reader
pub struct R(crate::R<GICV_HPPIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICV_HPPIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICV_HPPIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICV_HPPIR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PENDINTID` reader - PENDINTID
pub type PENDINTID_R = crate::FieldReader<u16, u16>;
///Field `CPUID` reader - CPUID
pub type CPUID_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:9 - PENDINTID
    #[inline(always)]
    pub fn pendintid(&self) -> PENDINTID_R {
        PENDINTID_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - CPUID
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 10) & 1) != 0)
    }
}
///GICV VM highest priority pending interrupt register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicv_hppir](index.html) module
pub struct GICV_HPPIR_SPEC;
impl crate::RegisterSpec for GICV_HPPIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicv_hppir::R](R) reader structure
impl crate::Readable for GICV_HPPIR_SPEC {
    type Reader = R;
}
///`reset()` method sets GICV_HPPIR to value 0x03ff
impl crate::Resettable for GICV_HPPIR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff;
}
