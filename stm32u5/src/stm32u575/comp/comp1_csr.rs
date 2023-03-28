///Register `COMP1_CSR` reader
pub struct R(crate::R<COMP1_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP1_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP1_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP1_CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `COMP1_CSR` writer
pub struct W(crate::W<COMP1_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP1_CSR_SPEC>;
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
impl From<crate::W<COMP1_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP1_CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `COMP1_EN` reader - Comparator 1 enable bit
pub type COMP1_EN_R = crate::BitReader<bool>;
///Field `COMP1_EN` writer - Comparator 1 enable bit
pub type COMP1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
///Field `COMP1_INMSEL` reader - Comparator 1 Input Minus connection configuration bit
pub type COMP1_INMSEL_R = crate::FieldReader<u8, u8>;
///Field `COMP1_INMSEL` writer - Comparator 1 Input Minus connection configuration bit
pub type COMP1_INMSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 4, O>;
///Field `COMP1_INPSEL` reader - Comparator1 input plus selection bit
pub type COMP1_INPSEL_R = crate::FieldReader<u8, u8>;
///Field `COMP1_INPSEL` writer - Comparator1 input plus selection bit
pub type COMP1_INPSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 2, O>;
///Field `COMP1_WINMODE` reader - COMP1_WINMODE
pub type COMP1_WINMODE_R = crate::BitReader<bool>;
///Field `COMP1_WINMODE` writer - COMP1_WINMODE
pub type COMP1_WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
///Field `COMP1_WINOUT` reader - COMP1_WINOUT
pub type COMP1_WINOUT_R = crate::BitReader<bool>;
///Field `COMP1_WINOUT` writer - COMP1_WINOUT
pub type COMP1_WINOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
///Field `COMP1_POLARITY` reader - Comparator 1 polarity selection bit
pub type COMP1_POLARITY_R = crate::BitReader<bool>;
///Field `COMP1_POLARITY` writer - Comparator 1 polarity selection bit
pub type COMP1_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
///Field `COMP1_HYST` reader - Comparator 1 hysteresis selection bits
pub type COMP1_HYST_R = crate::FieldReader<u8, u8>;
///Field `COMP1_HYST` writer - Comparator 1 hysteresis selection bits
pub type COMP1_HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 2, O>;
///Field `COMP1_PWRMODE` reader - COMP1_PWRMODE
pub type COMP1_PWRMODE_R = crate::FieldReader<u8, u8>;
///Field `COMP1_PWRMODE` writer - COMP1_PWRMODE
pub type COMP1_PWRMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 2, O>;
///Field `COMP1_BLANKSEL` reader - COMP1_BLANKSEL
pub type COMP1_BLANKSEL_R = crate::FieldReader<u8, u8>;
///Field `COMP1_BLANKSEL` writer - COMP1_BLANKSEL
pub type COMP1_BLANKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 5, O>;
///Field `COMP1_VALUE` reader - Comparator 1 output status bit
pub type COMP1_VALUE_R = crate::BitReader<bool>;
///Field `COMP1_LOCK` reader - COMP1_CSR register lock bit
pub type COMP1_LOCK_R = crate::BitReader<bool>;
///Field `COMP1_LOCK` writer - COMP1_CSR register lock bit
pub type COMP1_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Comparator 1 enable bit
    #[inline(always)]
    pub fn comp1_en(&self) -> COMP1_EN_R {
        COMP1_EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:7 - Comparator 1 Input Minus connection configuration bit
    #[inline(always)]
    pub fn comp1_inmsel(&self) -> COMP1_INMSEL_R {
        COMP1_INMSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Comparator1 input plus selection bit
    #[inline(always)]
    pub fn comp1_inpsel(&self) -> COMP1_INPSEL_R {
        COMP1_INPSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - COMP1_WINMODE
    #[inline(always)]
    pub fn comp1_winmode(&self) -> COMP1_WINMODE_R {
        COMP1_WINMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - COMP1_WINOUT
    #[inline(always)]
    pub fn comp1_winout(&self) -> COMP1_WINOUT_R {
        COMP1_WINOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Comparator 1 polarity selection bit
    #[inline(always)]
    pub fn comp1_polarity(&self) -> COMP1_POLARITY_R {
        COMP1_POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 1 hysteresis selection bits
    #[inline(always)]
    pub fn comp1_hyst(&self) -> COMP1_HYST_R {
        COMP1_HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - COMP1_PWRMODE
    #[inline(always)]
    pub fn comp1_pwrmode(&self) -> COMP1_PWRMODE_R {
        COMP1_PWRMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:24 - COMP1_BLANKSEL
    #[inline(always)]
    pub fn comp1_blanksel(&self) -> COMP1_BLANKSEL_R {
        COMP1_BLANKSEL_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bit 30 - Comparator 1 output status bit
    #[inline(always)]
    pub fn comp1_value(&self) -> COMP1_VALUE_R {
        COMP1_VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - COMP1_CSR register lock bit
    #[inline(always)]
    pub fn comp1_lock(&self) -> COMP1_LOCK_R {
        COMP1_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Comparator 1 enable bit
    #[inline(always)]
    #[must_use]
    pub fn comp1_en(&mut self) -> COMP1_EN_W<0> {
        COMP1_EN_W::new(self)
    }
    ///Bits 4:7 - Comparator 1 Input Minus connection configuration bit
    #[inline(always)]
    #[must_use]
    pub fn comp1_inmsel(&mut self) -> COMP1_INMSEL_W<4> {
        COMP1_INMSEL_W::new(self)
    }
    ///Bits 8:9 - Comparator1 input plus selection bit
    #[inline(always)]
    #[must_use]
    pub fn comp1_inpsel(&mut self) -> COMP1_INPSEL_W<8> {
        COMP1_INPSEL_W::new(self)
    }
    ///Bit 11 - COMP1_WINMODE
    #[inline(always)]
    #[must_use]
    pub fn comp1_winmode(&mut self) -> COMP1_WINMODE_W<11> {
        COMP1_WINMODE_W::new(self)
    }
    ///Bit 14 - COMP1_WINOUT
    #[inline(always)]
    #[must_use]
    pub fn comp1_winout(&mut self) -> COMP1_WINOUT_W<14> {
        COMP1_WINOUT_W::new(self)
    }
    ///Bit 15 - Comparator 1 polarity selection bit
    #[inline(always)]
    #[must_use]
    pub fn comp1_polarity(&mut self) -> COMP1_POLARITY_W<15> {
        COMP1_POLARITY_W::new(self)
    }
    ///Bits 16:17 - Comparator 1 hysteresis selection bits
    #[inline(always)]
    #[must_use]
    pub fn comp1_hyst(&mut self) -> COMP1_HYST_W<16> {
        COMP1_HYST_W::new(self)
    }
    ///Bits 18:19 - COMP1_PWRMODE
    #[inline(always)]
    #[must_use]
    pub fn comp1_pwrmode(&mut self) -> COMP1_PWRMODE_W<18> {
        COMP1_PWRMODE_W::new(self)
    }
    ///Bits 20:24 - COMP1_BLANKSEL
    #[inline(always)]
    #[must_use]
    pub fn comp1_blanksel(&mut self) -> COMP1_BLANKSEL_W<20> {
        COMP1_BLANKSEL_W::new(self)
    }
    ///Bit 31 - COMP1_CSR register lock bit
    #[inline(always)]
    #[must_use]
    pub fn comp1_lock(&mut self) -> COMP1_LOCK_W<31> {
        COMP1_LOCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Comparator 1 control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [comp1_csr](index.html) module
pub struct COMP1_CSR_SPEC;
impl crate::RegisterSpec for COMP1_CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [comp1_csr::R](R) reader structure
impl crate::Readable for COMP1_CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [comp1_csr::W](W) writer structure
impl crate::Writable for COMP1_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets COMP1_CSR to value 0
impl crate::Resettable for COMP1_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
