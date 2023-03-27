///Register `SR2` reader
pub struct R(crate::R<SR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TIM1F` reader - illegal access flag for TIM1
pub type TIM1F_R = crate::BitReader<bool>;
///Field `SPI1F` reader - illegal access flag for SPI1
pub type SPI1F_R = crate::BitReader<bool>;
///Field `TIM8F` reader - illegal access flag for TIM8
pub type TIM8F_R = crate::BitReader<bool>;
///Field `USART1F` reader - illegal access flag for USART1
pub type USART1F_R = crate::BitReader<bool>;
///Field `TIM15F` reader - illegal access flag for TIM5
pub type TIM15F_R = crate::BitReader<bool>;
///Field `TIM16F` reader - illegal access flag for TIM6
pub type TIM16F_R = crate::BitReader<bool>;
///Field `TIM17F` reader - illegal access flag for TIM7
pub type TIM17F_R = crate::BitReader<bool>;
///Field `SAI1F` reader - illegal access flag for SAI1
pub type SAI1F_R = crate::BitReader<bool>;
///Field `SAI2F` reader - illegal access flag for SAI2
pub type SAI2F_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - illegal access flag for TIM1
    #[inline(always)]
    pub fn tim1f(&self) -> TIM1F_R {
        TIM1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for SPI1
    #[inline(always)]
    pub fn spi1f(&self) -> SPI1F_R {
        SPI1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access flag for TIM8
    #[inline(always)]
    pub fn tim8f(&self) -> TIM8F_R {
        TIM8F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access flag for USART1
    #[inline(always)]
    pub fn usart1f(&self) -> USART1F_R {
        USART1F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access flag for TIM5
    #[inline(always)]
    pub fn tim15f(&self) -> TIM15F_R {
        TIM15F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access flag for TIM6
    #[inline(always)]
    pub fn tim16f(&self) -> TIM16F_R {
        TIM16F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access flag for TIM7
    #[inline(always)]
    pub fn tim17f(&self) -> TIM17F_R {
        TIM17F_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access flag for SAI1
    #[inline(always)]
    pub fn sai1f(&self) -> SAI1F_R {
        SAI1F_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access flag for SAI2
    #[inline(always)]
    pub fn sai2f(&self) -> SAI2F_R {
        SAI2F_R::new(((self.bits >> 8) & 1) != 0)
    }
}
///TZIC status register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr2](index.html) module
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr2::R](R) reader structure
impl crate::Readable for SR2_SPEC {
    type Reader = R;
}
///`reset()` method sets SR2 to value 0
impl crate::Resettable for SR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
