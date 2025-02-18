///Register `CCSIDR` reader
pub struct R(crate::R<CCSIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCSIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCSIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCSIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `LineSize` reader - LineSize
pub type LINE_SIZE_R = crate::FieldReader<u8, u8>;
///Field `Associativity` reader - Associativity
pub type ASSOCIATIVITY_R = crate::FieldReader<u16, u16>;
///Field `NumSets` reader - NumSets
pub type NUM_SETS_R = crate::FieldReader<u16, u16>;
///Field `WA` reader - WA
pub type WA_R = crate::BitReader<bool>;
///Field `RA` reader - RA
pub type RA_R = crate::BitReader<bool>;
///Field `WB` reader - WB
pub type WB_R = crate::BitReader<bool>;
///Field `WT` reader - WT
pub type WT_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:2 - LineSize
    #[inline(always)]
    pub fn line_size(&self) -> LINE_SIZE_R {
        LINE_SIZE_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:12 - Associativity
    #[inline(always)]
    pub fn associativity(&self) -> ASSOCIATIVITY_R {
        ASSOCIATIVITY_R::new(((self.bits >> 3) & 0x03ff) as u16)
    }
    ///Bits 13:27 - NumSets
    #[inline(always)]
    pub fn num_sets(&self) -> NUM_SETS_R {
        NUM_SETS_R::new(((self.bits >> 13) & 0x7fff) as u16)
    }
    ///Bit 28 - WA
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - RA
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - WB
    #[inline(always)]
    pub fn wb(&self) -> WB_R {
        WB_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - WT
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
///Cache Size ID register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccsidr](index.html) module
pub struct CCSIDR_SPEC;
impl crate::RegisterSpec for CCSIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccsidr::R](R) reader structure
impl crate::Readable for CCSIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets CCSIDR to value 0
impl crate::Resettable for CCSIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
