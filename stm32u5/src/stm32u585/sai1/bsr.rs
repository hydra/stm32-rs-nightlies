///Register `BSR` reader
pub struct R(crate::R<BSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `OVRUDR` reader - Overrun / underrun
pub type OVRUDR_R = crate::BitReader<bool>;
///Field `MUTEDET` reader - Mute detection
pub type MUTEDET_R = crate::BitReader<bool>;
///Field `WCKCFG` reader - Wrong clock configuration flag
pub type WCKCFG_R = crate::BitReader<bool>;
///Field `FREQ` reader - FIFO request
pub type FREQ_R = crate::BitReader<bool>;
///Field `CNRDY` reader - Codec not ready
pub type CNRDY_R = crate::BitReader<bool>;
///Field `AFSDET` reader - Anticipated frame synchronization detection
pub type AFSDET_R = crate::BitReader<bool>;
///Field `LFSDET` reader - Late frame synchronization detection
pub type LFSDET_R = crate::BitReader<bool>;
///Field `FLVL` reader - FIFO level threshold
pub type FLVL_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - Overrun / underrun
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Mute detection
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wrong clock configuration flag
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FIFO request
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Codec not ready
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Anticipated frame synchronization detection
    #[inline(always)]
    pub fn afsdet(&self) -> AFSDET_R {
        AFSDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Late frame synchronization detection
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 16:18 - FIFO level threshold
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
///B Status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bsr](index.html) module
pub struct BSR_SPEC;
impl crate::RegisterSpec for BSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bsr::R](R) reader structure
impl crate::Readable for BSR_SPEC {
    type Reader = R;
}
///`reset()` method sets BSR to value 0x08
impl crate::Resettable for BSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
