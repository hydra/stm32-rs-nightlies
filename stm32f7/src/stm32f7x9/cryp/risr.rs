///Register `RISR` reader
pub struct R(crate::R<RISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `INRIS` reader - Input FIFO service raw interrupt status
pub type INRIS_R = crate::BitReader<bool>;
///Field `OUTRIS` reader - Output FIFO service raw interrupt status
pub type OUTRIS_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Input FIFO service raw interrupt status
    #[inline(always)]
    pub fn inris(&self) -> INRIS_R {
        INRIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Output FIFO service raw interrupt status
    #[inline(always)]
    pub fn outris(&self) -> OUTRIS_R {
        OUTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
///raw interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [risr](index.html) module
pub struct RISR_SPEC;
impl crate::RegisterSpec for RISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [risr::R](R) reader structure
impl crate::Readable for RISR_SPEC {
    type Reader = R;
}
///`reset()` method sets RISR to value 0x01
impl crate::Resettable for RISR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
