///Register `MACHWF3R` reader
pub struct R(crate::R<MACHWF3R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACHWF3R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACHWF3R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACHWF3R_SPEC>) -> Self {
        R(reader)
    }
}
///Field `NRVF` reader - Number of Extended VLAN Tag Filters Enabled This field indicates the Number of Extended VLAN Tag Filters selected: 110 to 111: Reserved, must not be used
pub type NRVF_R = crate::FieldReader<u8, u8>;
///Field `CBTISEL` reader - Queue/Channel based VLAN tag insertion on Tx enable This bit is set to 1 when the Enable Queue/Channel based VLAN tag insertion on Tx feature is selected.
pub type CBTISEL_R = crate::BitReader<bool>;
///Field `DVLAN` reader - Double VLAN processing enable This bit is set to 1 when Double VLAN processing is enabled.
pub type DVLAN_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:2 - Number of Extended VLAN Tag Filters Enabled This field indicates the Number of Extended VLAN Tag Filters selected: 110 to 111: Reserved, must not be used
    #[inline(always)]
    pub fn nrvf(&self) -> NRVF_R {
        NRVF_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - Queue/Channel based VLAN tag insertion on Tx enable This bit is set to 1 when the Enable Queue/Channel based VLAN tag insertion on Tx feature is selected.
    #[inline(always)]
    pub fn cbtisel(&self) -> CBTISEL_R {
        CBTISEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Double VLAN processing enable This bit is set to 1 when Double VLAN processing is enabled.
    #[inline(always)]
    pub fn dvlan(&self) -> DVLAN_R {
        DVLAN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
///HW feature 3 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [machwf3r](index.html) module
pub struct MACHWF3R_SPEC;
impl crate::RegisterSpec for MACHWF3R_SPEC {
    type Ux = u32;
}
///`read()` method returns [machwf3r::R](R) reader structure
impl crate::Readable for MACHWF3R_SPEC {
    type Reader = R;
}
///`reset()` method sets MACHWF3R to value 0x20
impl crate::Resettable for MACHWF3R_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
