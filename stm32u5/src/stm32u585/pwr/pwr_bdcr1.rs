///Register `PWR_BDCR1` reader
pub struct R(crate::R<PWR_BDCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_BDCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_BDCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_BDCR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PWR_BDCR1` writer
pub struct W(crate::W<PWR_BDCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_BDCR1_SPEC>;
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
impl From<crate::W<PWR_BDCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_BDCR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BREN` reader - Backup RAM retention in Standby and VBAT modes When this bit is set, the backup RAM content is kept in Standby and VBAT modes. If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS. Note: Backup RAM cannot be preserved in Shutdown mode.
pub type BREN_R = crate::BitReader<bool>;
///Field `BREN` writer - Backup RAM retention in Standby and VBAT modes When this bit is set, the backup RAM content is kept in Standby and VBAT modes. If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS. Note: Backup RAM cannot be preserved in Shutdown mode.
pub type BREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_BDCR1_SPEC, bool, O>;
///Field `MONEN` reader - Backup domain voltage and temperature monitoring enable
pub type MONEN_R = crate::BitReader<bool>;
///Field `MONEN` writer - Backup domain voltage and temperature monitoring enable
pub type MONEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_BDCR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - Backup RAM retention in Standby and VBAT modes When this bit is set, the backup RAM content is kept in Standby and VBAT modes. If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS. Note: Backup RAM cannot be preserved in Shutdown mode.
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Backup domain voltage and temperature monitoring enable
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Backup RAM retention in Standby and VBAT modes When this bit is set, the backup RAM content is kept in Standby and VBAT modes. If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS. Note: Backup RAM cannot be preserved in Shutdown mode.
    #[inline(always)]
    #[must_use]
    pub fn bren(&mut self) -> BREN_W<0> {
        BREN_W::new(self)
    }
    ///Bit 4 - Backup domain voltage and temperature monitoring enable
    #[inline(always)]
    #[must_use]
    pub fn monen(&mut self) -> MONEN_W<4> {
        MONEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR Backup domain control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_bdcr1](index.html) module
pub struct PWR_BDCR1_SPEC;
impl crate::RegisterSpec for PWR_BDCR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_bdcr1::R](R) reader structure
impl crate::Readable for PWR_BDCR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pwr_bdcr1::W](W) writer structure
impl crate::Writable for PWR_BDCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PWR_BDCR1 to value 0
impl crate::Resettable for PWR_BDCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
