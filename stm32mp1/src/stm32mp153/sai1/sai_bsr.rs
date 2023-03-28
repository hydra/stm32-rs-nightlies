///Register `SAI_BSR` reader
pub struct R(crate::R<SAI_BSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_BSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_BSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_BSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `OVRUDR` reader - OVRUDR
pub type OVRUDR_R = crate::BitReader<bool>;
///Field `MUTEDET` reader - MUTEDET
pub type MUTEDET_R = crate::BitReader<bool>;
///Field `WCKCFG` reader - WCKCFG
pub type WCKCFG_R = crate::BitReader<bool>;
///Field `FREQ` reader - FREQ
pub type FREQ_R = crate::BitReader<bool>;
///Field `CNRDY` reader - CNRDY
pub type CNRDY_R = crate::BitReader<bool>;
///Field `AFSDET` reader - AFSDET
pub type AFSDET_R = crate::BitReader<bool>;
///Field `LFSDET` reader - LFSDET
pub type LFSDET_R = crate::BitReader<bool>;
///Field `FLVL` reader - FLVL
pub type FLVL_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - OVRUDR
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MUTEDET
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WCKCFG
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FREQ
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CNRDY
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AFSDET
    #[inline(always)]
    pub fn afsdet(&self) -> AFSDET_R {
        AFSDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LFSDET
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 16:18 - FLVL
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
///Status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sai_bsr](index.html) module
pub struct SAI_BSR_SPEC;
impl crate::RegisterSpec for SAI_BSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sai_bsr::R](R) reader structure
impl crate::Readable for SAI_BSR_SPEC {
    type Reader = R;
}
///`reset()` method sets SAI_BSR to value 0x08
impl crate::Resettable for SAI_BSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
