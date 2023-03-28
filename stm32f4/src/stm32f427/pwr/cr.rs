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
///Field `LPDS` reader - Low-power deep sleep
pub type LPDS_R = crate::BitReader<bool>;
///Field `LPDS` writer - Low-power deep sleep
pub type LPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `PDDS` reader - Power down deepsleep
pub type PDDS_R = crate::BitReader<bool>;
///Field `PDDS` writer - Power down deepsleep
pub type PDDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CWUF` reader - Clear wakeup flag
pub type CWUF_R = crate::BitReader<bool>;
///Field `CWUF` writer - Clear wakeup flag
pub type CWUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CSBF` reader - Clear standby flag
pub type CSBF_R = crate::BitReader<bool>;
///Field `CSBF` writer - Clear standby flag
pub type CSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `PVDE` reader - Power voltage detector enable
pub type PVDE_R = crate::BitReader<bool>;
///Field `PVDE` writer - Power voltage detector enable
pub type PVDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `PLS` reader - PVD level selection
pub type PLS_R = crate::FieldReader<u8, u8>;
///Field `PLS` writer - PVD level selection
pub type PLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
///Field `DBP` reader - Disable backup domain write protection
pub type DBP_R = crate::BitReader<bool>;
///Field `DBP` writer - Disable backup domain write protection
pub type DBP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `FPDS` reader - Flash power down in Stop mode
pub type FPDS_R = crate::BitReader<bool>;
///Field `FPDS` writer - Flash power down in Stop mode
pub type FPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `LPLVDS` reader - Low-Power Regulator Low Voltage in deepsleep
pub type LPLVDS_R = crate::BitReader<bool>;
///Field `LPLVDS` writer - Low-Power Regulator Low Voltage in deepsleep
pub type LPLVDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `MRLVDS` reader - Main regulator low voltage in deepsleep mode
pub type MRLVDS_R = crate::BitReader<bool>;
///Field `MRLVDS` writer - Main regulator low voltage in deepsleep mode
pub type MRLVDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ADCDC1` reader - Main regulator low voltage in deepsleep mode
pub type ADCDC1_R = crate::BitReader<bool>;
///Field `ADCDC1` writer - Main regulator low voltage in deepsleep mode
pub type ADCDC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `VOS` reader - Regulator voltage scaling output selection
pub type VOS_R = crate::FieldReader<u8, u8>;
///Field `VOS` writer - Regulator voltage scaling output selection
pub type VOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `ODEN` reader - Over-drive enable
pub type ODEN_R = crate::BitReader<bool>;
///Field `ODEN` writer - Over-drive enable
pub type ODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ODSWEN` reader - Over-drive switching enabled
pub type ODSWEN_R = crate::BitReader<bool>;
///Field `ODSWEN` writer - Over-drive switching enabled
pub type ODSWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `UDEN` reader - Under-drive enable in stop mode
pub type UDEN_R = crate::FieldReader<u8, u8>;
///Field `UDEN` writer - Under-drive enable in stop mode
pub type UDEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear wakeup flag
    #[inline(always)]
    pub fn cwuf(&self) -> CWUF_R {
        CWUF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Flash power down in Stop mode
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Low-Power Regulator Low Voltage in deepsleep
    #[inline(always)]
    pub fn lplvds(&self) -> LPLVDS_R {
        LPLVDS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Main regulator low voltage in deepsleep mode
    #[inline(always)]
    pub fn mrlvds(&self) -> MRLVDS_R {
        MRLVDS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - Main regulator low voltage in deepsleep mode
    #[inline(always)]
    pub fn adcdc1(&self) -> ADCDC1_R {
        ADCDC1_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Regulator voltage scaling output selection
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Over-drive enable
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Over-drive switching enabled
    #[inline(always)]
    pub fn odswen(&self) -> ODSWEN_R {
        ODSWEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Under-drive enable in stop mode
    #[inline(always)]
    pub fn uden(&self) -> UDEN_R {
        UDEN_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    #[must_use]
    pub fn lpds(&mut self) -> LPDS_W<0> {
        LPDS_W::new(self)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    #[must_use]
    pub fn pdds(&mut self) -> PDDS_W<1> {
        PDDS_W::new(self)
    }
    ///Bit 2 - Clear wakeup flag
    #[inline(always)]
    #[must_use]
    pub fn cwuf(&mut self) -> CWUF_W<2> {
        CWUF_W::new(self)
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CSBF_W<3> {
        CSBF_W::new(self)
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<4> {
        PVDE_W::new(self)
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<5> {
        PLS_W::new(self)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<8> {
        DBP_W::new(self)
    }
    ///Bit 9 - Flash power down in Stop mode
    #[inline(always)]
    #[must_use]
    pub fn fpds(&mut self) -> FPDS_W<9> {
        FPDS_W::new(self)
    }
    ///Bit 10 - Low-Power Regulator Low Voltage in deepsleep
    #[inline(always)]
    #[must_use]
    pub fn lplvds(&mut self) -> LPLVDS_W<10> {
        LPLVDS_W::new(self)
    }
    ///Bit 11 - Main regulator low voltage in deepsleep mode
    #[inline(always)]
    #[must_use]
    pub fn mrlvds(&mut self) -> MRLVDS_W<11> {
        MRLVDS_W::new(self)
    }
    ///Bit 13 - Main regulator low voltage in deepsleep mode
    #[inline(always)]
    #[must_use]
    pub fn adcdc1(&mut self) -> ADCDC1_W<13> {
        ADCDC1_W::new(self)
    }
    ///Bits 14:15 - Regulator voltage scaling output selection
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<14> {
        VOS_W::new(self)
    }
    ///Bit 16 - Over-drive enable
    #[inline(always)]
    #[must_use]
    pub fn oden(&mut self) -> ODEN_W<16> {
        ODEN_W::new(self)
    }
    ///Bit 17 - Over-drive switching enabled
    #[inline(always)]
    #[must_use]
    pub fn odswen(&mut self) -> ODSWEN_W<17> {
        ODSWEN_W::new(self)
    }
    ///Bits 18:19 - Under-drive enable in stop mode
    #[inline(always)]
    #[must_use]
    pub fn uden(&mut self) -> UDEN_W<18> {
        UDEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///power control register
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
///`reset()` method sets CR to value 0xc000
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000;
}
