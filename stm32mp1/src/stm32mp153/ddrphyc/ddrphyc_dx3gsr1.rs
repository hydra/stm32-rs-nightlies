///Register `DDRPHYC_DX3GSR1` reader
pub struct R(crate::R<DDRPHYC_DX3GSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DX3GSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DX3GSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DX3GSR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DFTERR` reader - DFTERR
pub type DFTERR_R = crate::BitReader<bool>;
///Field `DQSDFT` reader - DQSDFT
pub type DQSDFT_R = crate::FieldReader<u8, u8>;
///Field `RVERR` reader - RVERR
pub type RVERR_R = crate::BitReader<bool>;
///Field `RVIERR` reader - RVIERR
pub type RVIERR_R = crate::BitReader<bool>;
///Field `RVPASS` reader - RVPASS
pub type RVPASS_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - DFTERR
    #[inline(always)]
    pub fn dfterr(&self) -> DFTERR_R {
        DFTERR_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:5 - DQSDFT
    #[inline(always)]
    pub fn dqsdft(&self) -> DQSDFT_R {
        DQSDFT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 12 - RVERR
    #[inline(always)]
    pub fn rverr(&self) -> RVERR_R {
        RVERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - RVIERR
    #[inline(always)]
    pub fn rvierr(&self) -> RVIERR_R {
        RVIERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:22 - RVPASS
    #[inline(always)]
    pub fn rvpass(&self) -> RVPASS_R {
        RVPASS_R::new(((self.bits >> 20) & 7) as u8)
    }
}
///DDRPHYC byte lane 3 GS register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrphyc_dx3gsr1](index.html) module
pub struct DDRPHYC_DX3GSR1_SPEC;
impl crate::RegisterSpec for DDRPHYC_DX3GSR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrphyc_dx3gsr1::R](R) reader structure
impl crate::Readable for DDRPHYC_DX3GSR1_SPEC {
    type Reader = R;
}
///`reset()` method sets DDRPHYC_DX3GSR1 to value 0
impl crate::Resettable for DDRPHYC_DX3GSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
