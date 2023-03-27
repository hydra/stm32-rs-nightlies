///Register `SR1` reader
pub struct R(crate::R<SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TIM2F` reader - illegal access flag for TIM2
pub type TIM2F_R = crate::BitReader<bool>;
///Field `TIM3F` reader - illegal access flag for TIM3
pub type TIM3F_R = crate::BitReader<bool>;
///Field `TIM4F` reader - illegal access flag for TIM4
pub type TIM4F_R = crate::BitReader<bool>;
///Field `TIM5F` reader - illegal access flag for TIM5
pub type TIM5F_R = crate::BitReader<bool>;
///Field `TIM6F` reader - illegal access flag for TIM6
pub type TIM6F_R = crate::BitReader<bool>;
///Field `TIM7F` reader - illegal access flag for TIM7
pub type TIM7F_R = crate::BitReader<bool>;
///Field `WWDGF` reader - illegal access flag for WWDG
pub type WWDGF_R = crate::BitReader<bool>;
///Field `IWDGF` reader - illegal access flag for IWDG
pub type IWDGF_R = crate::BitReader<bool>;
///Field `SPI2F` reader - illegal access flag for SPI2
pub type SPI2F_R = crate::BitReader<bool>;
///Field `USART2F` reader - illegal access flag for USART2
pub type USART2F_R = crate::BitReader<bool>;
///Field `USART3F` reader - illegal access flag for USART3
pub type USART3F_R = crate::BitReader<bool>;
///Field `UART4F` reader - illegal access flag for UART4
pub type UART4F_R = crate::BitReader<bool>;
///Field `UART5F` reader - illegal access flag for UART5
pub type UART5F_R = crate::BitReader<bool>;
///Field `I2C1F` reader - illegal access flag for I2C1
pub type I2C1F_R = crate::BitReader<bool>;
///Field `I2C2F` reader - illegal access flag for I2C2
pub type I2C2F_R = crate::BitReader<bool>;
///Field `CRSF` reader - illegal access flag for CRS
pub type CRSF_R = crate::BitReader<bool>;
///Field `I2C4F` reader - illegal access flag for I2C4
pub type I2C4F_R = crate::BitReader<bool>;
///Field `LPTIM2F` reader - illegal access flag for LPTIM2
pub type LPTIM2F_R = crate::BitReader<bool>;
///Field `FDCAN1F` reader - illegal access flag for FDCAN1
pub type FDCAN1F_R = crate::BitReader<bool>;
///Field `UCPD1F` reader - illegal access flag for UCPD1
pub type UCPD1F_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - illegal access flag for TIM2
    #[inline(always)]
    pub fn tim2f(&self) -> TIM2F_R {
        TIM2F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for TIM3
    #[inline(always)]
    pub fn tim3f(&self) -> TIM3F_R {
        TIM3F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access flag for TIM4
    #[inline(always)]
    pub fn tim4f(&self) -> TIM4F_R {
        TIM4F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access flag for TIM5
    #[inline(always)]
    pub fn tim5f(&self) -> TIM5F_R {
        TIM5F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access flag for TIM6
    #[inline(always)]
    pub fn tim6f(&self) -> TIM6F_R {
        TIM6F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access flag for TIM7
    #[inline(always)]
    pub fn tim7f(&self) -> TIM7F_R {
        TIM7F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access flag for WWDG
    #[inline(always)]
    pub fn wwdgf(&self) -> WWDGF_R {
        WWDGF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access flag for IWDG
    #[inline(always)]
    pub fn iwdgf(&self) -> IWDGF_R {
        IWDGF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access flag for SPI2
    #[inline(always)]
    pub fn spi2f(&self) -> SPI2F_R {
        SPI2F_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access flag for USART2
    #[inline(always)]
    pub fn usart2f(&self) -> USART2F_R {
        USART2F_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access flag for USART3
    #[inline(always)]
    pub fn usart3f(&self) -> USART3F_R {
        USART3F_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access flag for UART4
    #[inline(always)]
    pub fn uart4f(&self) -> UART4F_R {
        UART4F_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access flag for UART5
    #[inline(always)]
    pub fn uart5f(&self) -> UART5F_R {
        UART5F_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access flag for I2C1
    #[inline(always)]
    pub fn i2c1f(&self) -> I2C1F_R {
        I2C1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access flag for I2C2
    #[inline(always)]
    pub fn i2c2f(&self) -> I2C2F_R {
        I2C2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access flag for CRS
    #[inline(always)]
    pub fn crsf(&self) -> CRSF_R {
        CRSF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access flag for I2C4
    #[inline(always)]
    pub fn i2c4f(&self) -> I2C4F_R {
        I2C4F_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access flag for LPTIM2
    #[inline(always)]
    pub fn lptim2f(&self) -> LPTIM2F_R {
        LPTIM2F_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access flag for FDCAN1
    #[inline(always)]
    pub fn fdcan1f(&self) -> FDCAN1F_R {
        FDCAN1F_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access flag for UCPD1
    #[inline(always)]
    pub fn ucpd1f(&self) -> UCPD1F_R {
        UCPD1F_R::new(((self.bits >> 19) & 1) != 0)
    }
}
///TZIC status register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr1](index.html) module
pub struct SR1_SPEC;
impl crate::RegisterSpec for SR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr1::R](R) reader structure
impl crate::Readable for SR1_SPEC {
    type Reader = R;
}
///`reset()` method sets SR1 to value 0
impl crate::Resettable for SR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
