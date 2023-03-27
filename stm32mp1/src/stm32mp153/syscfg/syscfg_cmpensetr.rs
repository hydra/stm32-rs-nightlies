///Register `SYSCFG_CMPENSETR` reader
pub struct R(crate::R<SYSCFG_CMPENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_CMPENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_CMPENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_CMPENSETR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SYSCFG_CMPENSETR` writer
pub struct W(crate::W<SYSCFG_CMPENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_CMPENSETR_SPEC>;
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
impl From<crate::W<SYSCFG_CMPENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_CMPENSETR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MPU_EN` reader - MPU_EN
pub type MPU_EN_R = crate::BitReader<bool>;
///Field `MPU_EN` writer - MPU_EN
pub type MPU_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_CMPENSETR_SPEC, bool, O>;
///Field `MCU_EN` reader - MCU_EN
pub type MCU_EN_R = crate::BitReader<bool>;
///Field `MCU_EN` writer - MCU_EN
pub type MCU_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_CMPENSETR_SPEC, bool, O>;
impl R {
    ///Bit 0 - MPU_EN
    #[inline(always)]
    pub fn mpu_en(&self) -> MPU_EN_R {
        MPU_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MCU_EN
    #[inline(always)]
    pub fn mcu_en(&self) -> MCU_EN_R {
        MCU_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MPU_EN
    #[inline(always)]
    #[must_use]
    pub fn mpu_en(&mut self) -> MPU_EN_W<0> {
        MPU_EN_W::new(self)
    }
    ///Bit 1 - MCU_EN
    #[inline(always)]
    #[must_use]
    pub fn mcu_en(&mut self) -> MCU_EN_W<1> {
        MCU_EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SYSCFG compensation cell enable set register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [syscfg_cmpensetr](index.html) module
pub struct SYSCFG_CMPENSETR_SPEC;
impl crate::RegisterSpec for SYSCFG_CMPENSETR_SPEC {
    type Ux = u32;
}
///`read()` method returns [syscfg_cmpensetr::R](R) reader structure
impl crate::Readable for SYSCFG_CMPENSETR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [syscfg_cmpensetr::W](W) writer structure
impl crate::Writable for SYSCFG_CMPENSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SYSCFG_CMPENSETR to value 0
impl crate::Resettable for SYSCFG_CMPENSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
