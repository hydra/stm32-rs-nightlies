///Register `ITLINE27` reader
pub struct R(crate::R<ITLINE27_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE27_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE27_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE27_SPEC>) -> Self {
        R(reader)
    }
}
///Field `USART1` reader - USART1
pub type USART1_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - USART1
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new((self.bits & 1) != 0)
    }
}
///interrupt line 27 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline27](index.html) module
pub struct ITLINE27_SPEC;
impl crate::RegisterSpec for ITLINE27_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline27::R](R) reader structure
impl crate::Readable for ITLINE27_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE27 to value 0
impl crate::Resettable for ITLINE27_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
