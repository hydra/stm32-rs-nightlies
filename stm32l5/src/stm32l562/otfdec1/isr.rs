///Register `ISR` reader
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SEIF` reader - Security Error Interrupt Flag status
pub type SEIF_R = crate::BitReader<bool>;
///Field `XONEIF` reader - Execute-only execute-Never Error Interrupt Flag status
pub type XONEIF_R = crate::BitReader<bool>;
///Field `KEIF` reader - Key Error Interrupt Flag status
pub type KEIF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Security Error Interrupt Flag status
    #[inline(always)]
    pub fn seif(&self) -> SEIF_R {
        SEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Execute-only execute-Never Error Interrupt Flag status
    #[inline(always)]
    pub fn xoneif(&self) -> XONEIF_R {
        XONEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key Error Interrupt Flag status
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
///OTFDEC interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr](index.html) module
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [isr::R](R) reader structure
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
