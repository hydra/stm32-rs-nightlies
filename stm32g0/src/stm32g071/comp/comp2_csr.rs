///Register `COMP2_CSR` reader
pub struct R(crate::R<COMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `COMP2_CSR` writer
pub struct W(crate::W<COMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP2_CSR_SPEC>;
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
impl From<crate::W<COMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP2_CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - COMP channel 1 enable bit
pub type EN_R = crate::BitReader<bool>;
///Field `EN` writer - COMP channel 1 enable bit
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
///Field `INMSEL` reader - Comparator 2 signal selector for inverting input INM
pub type INMSEL_R = crate::FieldReader<u8, u8>;
///Field `INMSEL` writer - Comparator 2 signal selector for inverting input INM
pub type INMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 4, O>;
///Field `INPSEL` reader - Comparator 2 signal selector for non-inverting input
pub type INPSEL_R = crate::FieldReader<u8, u8>;
///Field `INPSEL` writer - Comparator 2 signal selector for non-inverting input
pub type INPSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 2, O>;
///Field `WINMODE` reader - Comparator 2 non-inverting input selector for window mode
pub type WINMODE_R = crate::BitReader<bool>;
///Field `WINMODE` writer - Comparator 2 non-inverting input selector for window mode
pub type WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
///Field `WINOUT` reader - Comparator 2 output selector
pub type WINOUT_R = crate::BitReader<bool>;
///Field `WINOUT` writer - Comparator 2 output selector
pub type WINOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
///Field `POLARITY` reader - Comparator 2 polarity selector
pub type POLARITY_R = crate::BitReader<bool>;
///Field `POLARITY` writer - Comparator 2 polarity selector
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
///Field `HYST` reader - Comparator 2 hysteresis selector
pub type HYST_R = crate::FieldReader<u8, u8>;
///Field `HYST` writer - Comparator 2 hysteresis selector
pub type HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 2, O>;
///Field `PWRMODE` reader - Comparator 2 power mode selector
pub type PWRMODE_R = crate::FieldReader<u8, u8>;
///Field `PWRMODE` writer - Comparator 2 power mode selector
pub type PWRMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 2, O>;
///Field `BLANKSEL` reader - Comparator 2 blanking source selector
pub type BLANKSEL_R = crate::FieldReader<u8, u8>;
///Field `BLANKSEL` writer - Comparator 2 blanking source selector
pub type BLANKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 5, O>;
///Field `VALUE` reader - Comparator 2 output status
pub type VALUE_R = crate::BitReader<bool>;
///Field `VALUE` writer - Comparator 2 output status
pub type VALUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
///Field `LOCK` reader - COMP2_CSR register lock
pub type LOCK_R = crate::BitReader<bool>;
///Field `LOCK` writer - COMP2_CSR register lock
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
impl R {
    ///Bit 0 - COMP channel 1 enable bit
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:7 - Comparator 2 signal selector for inverting input INM
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Comparator 2 signal selector for non-inverting input
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - Comparator 2 non-inverting input selector for window mode
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - Comparator 2 output selector
    #[inline(always)]
    pub fn winout(&self) -> WINOUT_R {
        WINOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Comparator 2 polarity selector
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 2 hysteresis selector
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Comparator 2 power mode selector
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:24 - Comparator 2 blanking source selector
    #[inline(always)]
    pub fn blanksel(&self) -> BLANKSEL_R {
        BLANKSEL_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bit 30 - Comparator 2 output status
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - COMP2_CSR register lock
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - COMP channel 1 enable bit
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bits 4:7 - Comparator 2 signal selector for inverting input INM
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<4> {
        INMSEL_W::new(self)
    }
    ///Bits 8:9 - Comparator 2 signal selector for non-inverting input
    #[inline(always)]
    #[must_use]
    pub fn inpsel(&mut self) -> INPSEL_W<8> {
        INPSEL_W::new(self)
    }
    ///Bit 11 - Comparator 2 non-inverting input selector for window mode
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WINMODE_W<11> {
        WINMODE_W::new(self)
    }
    ///Bit 14 - Comparator 2 output selector
    #[inline(always)]
    #[must_use]
    pub fn winout(&mut self) -> WINOUT_W<14> {
        WINOUT_W::new(self)
    }
    ///Bit 15 - Comparator 2 polarity selector
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<15> {
        POLARITY_W::new(self)
    }
    ///Bits 16:17 - Comparator 2 hysteresis selector
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<16> {
        HYST_W::new(self)
    }
    ///Bits 18:19 - Comparator 2 power mode selector
    #[inline(always)]
    #[must_use]
    pub fn pwrmode(&mut self) -> PWRMODE_W<18> {
        PWRMODE_W::new(self)
    }
    ///Bits 20:24 - Comparator 2 blanking source selector
    #[inline(always)]
    #[must_use]
    pub fn blanksel(&mut self) -> BLANKSEL_W<20> {
        BLANKSEL_W::new(self)
    }
    ///Bit 30 - Comparator 2 output status
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<30> {
        VALUE_W::new(self)
    }
    ///Bit 31 - COMP2_CSR register lock
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Comparator 2 control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [comp2_csr](index.html) module
pub struct COMP2_CSR_SPEC;
impl crate::RegisterSpec for COMP2_CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [comp2_csr::R](R) reader structure
impl crate::Readable for COMP2_CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [comp2_csr::W](W) writer structure
impl crate::Writable for COMP2_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets COMP2_CSR to value 0
impl crate::Resettable for COMP2_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
