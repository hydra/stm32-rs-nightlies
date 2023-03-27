///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `BUSYF` reader - full invalidate busy flag
pub type BUSYF_R = crate::BitReader<bool>;
///Field `BSYENDF` reader - full invalidate busy end flag Cleared by writing DCACHE_FCR.CBSYENDF = 1.
pub type BSYENDF_R = crate::BitReader<bool>;
///Field `ERRF` reader - cache error flag Cleared by writing DCACHE_FCR.CERRF = 1.
pub type ERRF_R = crate::BitReader<bool>;
///Field `BUSYCMDF` reader - command busy flag
pub type BUSYCMDF_R = crate::BitReader<bool>;
///Field `CMDENDF` reader - command end flag Cleared by writing DCACHE_FCR.CCMDENDF = 1.
pub type CMDENDF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - full invalidate busy flag
    #[inline(always)]
    pub fn busyf(&self) -> BUSYF_R {
        BUSYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - full invalidate busy end flag Cleared by writing DCACHE_FCR.CBSYENDF = 1.
    #[inline(always)]
    pub fn bsyendf(&self) -> BSYENDF_R {
        BSYENDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - cache error flag Cleared by writing DCACHE_FCR.CERRF = 1.
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - command busy flag
    #[inline(always)]
    pub fn busycmdf(&self) -> BUSYCMDF_R {
        BUSYCMDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - command end flag Cleared by writing DCACHE_FCR.CCMDENDF = 1.
    #[inline(always)]
    pub fn cmdendf(&self) -> CMDENDF_R {
        CMDENDF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
///DCACHE status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`reset()` method sets SR to value 0x01
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
