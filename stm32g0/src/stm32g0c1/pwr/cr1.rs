///Register `CR1` reader
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR1` writer
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPMS` reader - Low-power mode selection
pub type LPMS_R = crate::FieldReader<u8, u8>;
///Field `LPMS` writer - Low-power mode selection
pub type LPMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
///Field `FPD_STOP` reader - Flash memory powered down during Stop mode
pub type FPD_STOP_R = crate::BitReader<bool>;
///Field `FPD_STOP` writer - Flash memory powered down during Stop mode
pub type FPD_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `FPD_LPRUN` reader - Flash memory powered down during Low-power run mode
pub type FPD_LPRUN_R = crate::BitReader<bool>;
///Field `FPD_LPRUN` writer - Flash memory powered down during Low-power run mode
pub type FPD_LPRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `FPD_LPSLP` reader - Flash memory powered down during Low-power sleep mode
pub type FPD_LPSLP_R = crate::BitReader<bool>;
///Field `FPD_LPSLP` writer - Flash memory powered down during Low-power sleep mode
pub type FPD_LPSLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `DBP` reader - Disable backup domain write protection
pub type DBP_R = crate::BitReader<bool>;
///Field `DBP` writer - Disable backup domain write protection
pub type DBP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `VOS` reader - Voltage scaling range selection
pub type VOS_R = crate::FieldReader<u8, u8>;
///Field `VOS` writer - Voltage scaling range selection
pub type VOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 2, O>;
///Field `LPR` reader - Low-power run
pub type LPR_R = crate::BitReader<bool>;
///Field `LPR` writer - Low-power run
pub type LPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    ///Bits 0:2 - Low-power mode selection
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Flash memory powered down during Stop mode
    #[inline(always)]
    pub fn fpd_stop(&self) -> FPD_STOP_R {
        FPD_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Flash memory powered down during Low-power run mode
    #[inline(always)]
    pub fn fpd_lprun(&self) -> FPD_LPRUN_R {
        FPD_LPRUN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Flash memory powered down during Low-power sleep mode
    #[inline(always)]
    pub fn fpd_lpslp(&self) -> FPD_LPSLP_R {
        FPD_LPSLP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - Voltage scaling range selection
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 14 - Low-power run
    #[inline(always)]
    pub fn lpr(&self) -> LPR_R {
        LPR_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Low-power mode selection
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LPMS_W<0> {
        LPMS_W::new(self)
    }
    ///Bit 3 - Flash memory powered down during Stop mode
    #[inline(always)]
    #[must_use]
    pub fn fpd_stop(&mut self) -> FPD_STOP_W<3> {
        FPD_STOP_W::new(self)
    }
    ///Bit 4 - Flash memory powered down during Low-power run mode
    #[inline(always)]
    #[must_use]
    pub fn fpd_lprun(&mut self) -> FPD_LPRUN_W<4> {
        FPD_LPRUN_W::new(self)
    }
    ///Bit 5 - Flash memory powered down during Low-power sleep mode
    #[inline(always)]
    #[must_use]
    pub fn fpd_lpslp(&mut self) -> FPD_LPSLP_W<5> {
        FPD_LPSLP_W::new(self)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<8> {
        DBP_W::new(self)
    }
    ///Bits 9:10 - Voltage scaling range selection
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<9> {
        VOS_W::new(self)
    }
    ///Bit 14 - Low-power run
    #[inline(always)]
    #[must_use]
    pub fn lpr(&mut self) -> LPR_W<14> {
        LPR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](index.html) module
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr1::R](R) reader structure
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr1::W](W) writer structure
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR1 to value 0x0208
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0208;
}
