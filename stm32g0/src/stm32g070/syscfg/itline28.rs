///Register `ITLINE28` reader
pub struct R(crate::R<ITLINE28_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE28_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE28_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE28_SPEC>) -> Self {
        R(reader)
    }
}
///Field `USART2` reader - USART2
pub type USART2_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - USART2
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new((self.bits & 1) != 0)
    }
}
///interrupt line 28 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline28](index.html) module
pub struct ITLINE28_SPEC;
impl crate::RegisterSpec for ITLINE28_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline28::R](R) reader structure
impl crate::Readable for ITLINE28_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE28 to value 0
impl crate::Resettable for ITLINE28_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
