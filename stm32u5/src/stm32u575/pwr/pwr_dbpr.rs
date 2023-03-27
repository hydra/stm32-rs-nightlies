///Register `PWR_DBPR` reader
pub struct R(crate::R<PWR_DBPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_DBPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_DBPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_DBPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PWR_DBPR` writer
pub struct W(crate::W<PWR_DBPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_DBPR_SPEC>;
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
impl From<crate::W<PWR_DBPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_DBPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DBP` reader - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers.
pub type DBP_R = crate::BitReader<bool>;
///Field `DBP` writer - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers.
pub type DBP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_DBPR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers.
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers.
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<0> {
        DBP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR disable Backup domain register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_dbpr](index.html) module
pub struct PWR_DBPR_SPEC;
impl crate::RegisterSpec for PWR_DBPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_dbpr::R](R) reader structure
impl crate::Readable for PWR_DBPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pwr_dbpr::W](W) writer structure
impl crate::Writable for PWR_DBPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PWR_DBPR to value 0
impl crate::Resettable for PWR_DBPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
