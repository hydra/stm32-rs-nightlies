///Register `RCC_MP_BOOTCR` reader
pub struct R(crate::R<RCC_MP_BOOTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_BOOTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_BOOTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_BOOTCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MP_BOOTCR` writer
pub struct W(crate::W<RCC_MP_BOOTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_BOOTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RCC_MP_BOOTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_BOOTCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MCU_BEN` reader - MCU_BEN
pub type MCU_BEN_R = crate::BitReader<bool>;
///Field `MCU_BEN` writer - MCU_BEN
pub type MCU_BEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_BOOTCR_SPEC, bool, O>;
///Field `MPU_BEN` reader - MPU_BEN
pub type MPU_BEN_R = crate::BitReader<bool>;
///Field `MPU_BEN` writer - MPU_BEN
pub type MPU_BEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_BOOTCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - MCU_BEN
    #[inline(always)]
    pub fn mcu_ben(&self) -> MCU_BEN_R {
        MCU_BEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MPU_BEN
    #[inline(always)]
    pub fn mpu_ben(&self) -> MPU_BEN_R {
        MPU_BEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MCU_BEN
    #[inline(always)]
    #[must_use]
    pub fn mcu_ben(&mut self) -> MCU_BEN_W<0> {
        MCU_BEN_W::new(self)
    }
    ///Bit 1 - MPU_BEN
    #[inline(always)]
    #[must_use]
    pub fn mpu_ben(&mut self) -> MPU_BEN_W<1> {
        MPU_BEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mp_bootcr](index.html) module
pub struct RCC_MP_BOOTCR_SPEC;
impl crate::RegisterSpec for RCC_MP_BOOTCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mp_bootcr::R](R) reader structure
impl crate::Readable for RCC_MP_BOOTCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mp_bootcr::W](W) writer structure
impl crate::Writable for RCC_MP_BOOTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MP_BOOTCR to value 0
impl crate::Resettable for RCC_MP_BOOTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
