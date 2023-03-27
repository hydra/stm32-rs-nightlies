///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPDS` reader - Low Power Deep Sleep
pub type LPDS_R = crate::BitReader<bool>;
///Field `LPDS` writer - Low Power Deep Sleep
pub type LPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `PDDS` reader - Power Down Deep Sleep
pub type PDDS_R = crate::BitReader<PDDS_A>;
///Power Down Deep Sleep
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDS_A {
    ///0: Enter Stop mode when the CPU enters deepsleep
    StopMode = 0,
    ///1: Enter Standby mode when the CPU enters deepsleep
    StandbyMode = 1,
}
impl From<PDDS_A> for bool {
    #[inline(always)]
    fn from(variant: PDDS_A) -> Self {
        variant as u8 != 0
    }
}
impl PDDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PDDS_A {
        match self.bits {
            false => PDDS_A::StopMode,
            true => PDDS_A::StandbyMode,
        }
    }
    ///Checks if the value of the field is `StopMode`
    #[inline(always)]
    pub fn is_stop_mode(&self) -> bool {
        *self == PDDS_A::StopMode
    }
    ///Checks if the value of the field is `StandbyMode`
    #[inline(always)]
    pub fn is_standby_mode(&self) -> bool {
        *self == PDDS_A::StandbyMode
    }
}
///Field `PDDS` writer - Power Down Deep Sleep
pub type PDDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PDDS_A, O>;
impl<'a, const O: u8> PDDS_W<'a, O> {
    ///Enter Stop mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn stop_mode(self) -> &'a mut W {
        self.variant(PDDS_A::StopMode)
    }
    ///Enter Standby mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn standby_mode(self) -> &'a mut W {
        self.variant(PDDS_A::StandbyMode)
    }
}
///Field `CWUF` reader - Clear Wake-up Flag
pub type CWUF_R = crate::BitReader<bool>;
///Field `CWUF` writer - Clear Wake-up Flag
pub type CWUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CSBF` reader - Clear STANDBY Flag
pub type CSBF_R = crate::BitReader<bool>;
///Field `CSBF` writer - Clear STANDBY Flag
pub type CSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `PVDE` reader - Power Voltage Detector Enable
pub type PVDE_R = crate::BitReader<bool>;
///Field `PVDE` writer - Power Voltage Detector Enable
pub type PVDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `PLS` reader - PVD Level Selection
pub type PLS_R = crate::FieldReader<u8, u8>;
///Field `PLS` writer - PVD Level Selection
pub type PLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
///Field `DBP` reader - Disable Backup Domain write protection
pub type DBP_R = crate::BitReader<bool>;
///Field `DBP` writer - Disable Backup Domain write protection
pub type DBP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Low Power Deep Sleep
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Power Down Deep Sleep
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear Wake-up Flag
    #[inline(always)]
    pub fn cwuf(&self) -> CWUF_R {
        CWUF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear STANDBY Flag
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Power Voltage Detector Enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - PVD Level Selection
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 8 - Disable Backup Domain write protection
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Low Power Deep Sleep
    #[inline(always)]
    #[must_use]
    pub fn lpds(&mut self) -> LPDS_W<0> {
        LPDS_W::new(self)
    }
    ///Bit 1 - Power Down Deep Sleep
    #[inline(always)]
    #[must_use]
    pub fn pdds(&mut self) -> PDDS_W<1> {
        PDDS_W::new(self)
    }
    ///Bit 2 - Clear Wake-up Flag
    #[inline(always)]
    #[must_use]
    pub fn cwuf(&mut self) -> CWUF_W<2> {
        CWUF_W::new(self)
    }
    ///Bit 3 - Clear STANDBY Flag
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CSBF_W<3> {
        CSBF_W::new(self)
    }
    ///Bit 4 - Power Voltage Detector Enable
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<4> {
        PVDE_W::new(self)
    }
    ///Bits 5:7 - PVD Level Selection
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<5> {
        PLS_W::new(self)
    }
    ///Bit 8 - Disable Backup Domain write protection
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<8> {
        DBP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power control register (PWR_CR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
