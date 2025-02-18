///Register `ITLINE30` reader
pub struct R(crate::R<ITLINE30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE30_SPEC>) -> Self {
        R(reader)
    }
}
///Field `USART2` reader - CEC
pub type USART2_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - CEC
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new((self.bits & 1) != 0)
    }
}
///interrupt line 30 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline30](index.html) module
pub struct ITLINE30_SPEC;
impl crate::RegisterSpec for ITLINE30_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline30::R](R) reader structure
impl crate::Readable for ITLINE30_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE30 to value 0
impl crate::Resettable for ITLINE30_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
