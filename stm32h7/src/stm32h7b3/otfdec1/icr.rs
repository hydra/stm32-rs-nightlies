///Register `ICR` reader
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SEIF` reader - SEIF
pub type SEIF_R = crate::BitReader<bool>;
///Field `XONEIF` reader - Execute-only execute-Never Error Interrupt Flag clear
pub type XONEIF_R = crate::BitReader<bool>;
///Field `KEIF` reader - KEIF
pub type KEIF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - SEIF
    #[inline(always)]
    pub fn seif(&self) -> SEIF_R {
        SEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Execute-only execute-Never Error Interrupt Flag clear
    #[inline(always)]
    pub fn xoneif(&self) -> XONEIF_R {
        XONEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - KEIF
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
///OTFDEC interrupt clear register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](index.html) module
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [icr::R](R) reader structure
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
