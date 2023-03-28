///Register `DDRPHYC_DX2GSR0` reader
pub struct R(crate::R<DDRPHYC_DX2GSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DX2GSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DX2GSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DX2GSR0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DTDONE` reader - DTDONE
pub type DTDONE_R = crate::BitReader<bool>;
///Field `DTERR` reader - DTERR
pub type DTERR_R = crate::BitReader<bool>;
///Field `DTIERR` reader - DTIERR
pub type DTIERR_R = crate::BitReader<bool>;
///Field `DTPASS` reader - DTPASS
pub type DTPASS_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - DTDONE
    #[inline(always)]
    pub fn dtdone(&self) -> DTDONE_R {
        DTDONE_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DTERR
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - DTIERR
    #[inline(always)]
    pub fn dtierr(&self) -> DTIERR_R {
        DTIERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 13:15 - DTPASS
    #[inline(always)]
    pub fn dtpass(&self) -> DTPASS_R {
        DTPASS_R::new(((self.bits >> 13) & 7) as u8)
    }
}
///DDRPHYC byte lane 2 GS register 0
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrphyc_dx2gsr0](index.html) module
pub struct DDRPHYC_DX2GSR0_SPEC;
impl crate::RegisterSpec for DDRPHYC_DX2GSR0_SPEC {
    type Ux = u16;
}
///`read()` method returns [ddrphyc_dx2gsr0::R](R) reader structure
impl crate::Readable for DDRPHYC_DX2GSR0_SPEC {
    type Reader = R;
}
///`reset()` method sets DDRPHYC_DX2GSR0 to value 0
impl crate::Resettable for DDRPHYC_DX2GSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
