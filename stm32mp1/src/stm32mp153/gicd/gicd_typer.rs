///Register `GICD_TYPER` reader
pub struct R(crate::R<GICD_TYPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_TYPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_TYPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_TYPER_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ITLINESNUMBER` reader - ITLINESNUMBER
pub type ITLINESNUMBER_R = crate::FieldReader<u8, u8>;
///Field `CPUNUMBER` reader - CPUNUMBER
pub type CPUNUMBER_R = crate::FieldReader<u8, u8>;
///Field `SECURITYEXTN` reader - SECURITYEXTN
pub type SECURITYEXTN_R = crate::BitReader<bool>;
///Field `LSPI` reader - LSPI
pub type LSPI_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:4 - ITLINESNUMBER
    #[inline(always)]
    pub fn itlinesnumber(&self) -> ITLINESNUMBER_R {
        ITLINESNUMBER_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:7 - CPUNUMBER
    #[inline(always)]
    pub fn cpunumber(&self) -> CPUNUMBER_R {
        CPUNUMBER_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 10 - SECURITYEXTN
    #[inline(always)]
    pub fn securityextn(&self) -> SECURITYEXTN_R {
        SECURITYEXTN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:15 - LSPI
    #[inline(always)]
    pub fn lspi(&self) -> LSPI_R {
        LSPI_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
///GICD interrupt controller type register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicd_typer](index.html) module
pub struct GICD_TYPER_SPEC;
impl crate::RegisterSpec for GICD_TYPER_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicd_typer::R](R) reader structure
impl crate::Readable for GICD_TYPER_SPEC {
    type Reader = R;
}
///`reset()` method sets GICD_TYPER to value 0xfc28
impl crate::Resettable for GICD_TYPER_SPEC {
    const RESET_VALUE: Self::Ux = 0xfc28;
}
