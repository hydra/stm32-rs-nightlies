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
///Field `COM2_EN` reader - Comparator 2 enable bit
pub type COM2_EN_R = crate::BitReader<bool>;
///Field `COM2_EN` writer - Comparator 2 enable bit
pub type COM2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
///Field `COM2_INMSEL` reader - Comparator 2 Input Minus connection configuration bit
pub type COM2_INMSEL_R = crate::FieldReader<u8, u8>;
///Field `COM2_INMSEL` writer - Comparator 2 Input Minus connection configuration bit
pub type COM2_INMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 4, O>;
///Field `COM2_INPSEL` reader - Comparator 2 input plus selection bit
pub type COM2_INPSEL_R = crate::FieldReader<u8, u8>;
///Field `COM2_INPSEL` writer - Comparator 2 input plus selection bit
pub type COM2_INPSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 2, O>;
///Field `COM2_WINMODE` reader - COM2_WINMODE
pub type COM2_WINMODE_R = crate::BitReader<bool>;
///Field `COM2_WINMODE` writer - COM2_WINMODE
pub type COM2_WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
///Field `COM2_WINOUT` reader - COM2_WINOUT
pub type COM2_WINOUT_R = crate::BitReader<bool>;
///Field `COM2_WINOUT` writer - COM2_WINOUT
pub type COM2_WINOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
///Field `COM2_POLARITY` reader - Comparator 2 polarity selection bit
pub type COM2_POLARITY_R = crate::BitReader<bool>;
///Field `COM2_POLARITY` writer - Comparator 2 polarity selection bit
pub type COM2_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
///Field `COM2_HYST` reader - Comparator 2 hysteresis selection bits
pub type COM2_HYST_R = crate::FieldReader<u8, u8>;
///Field `COM2_HYST` writer - Comparator 2 hysteresis selection bits
pub type COM2_HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 2, O>;
///Field `COM2_PWRMODE` reader - COM2_PWRMODE
pub type COM2_PWRMODE_R = crate::FieldReader<u8, u8>;
///Field `COM2_PWRMODE` writer - COM2_PWRMODE
pub type COM2_PWRMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 2, O>;
///Field `COM2_BLANKSEL` reader - COM2_BLANKSEL
pub type COM2_BLANKSEL_R = crate::FieldReader<u8, u8>;
///Field `COM2_BLANKSEL` writer - COM2_BLANKSEL
pub type COM2_BLANKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 5, O>;
///Field `COM2_VALUE` reader - Comparator 2 output status bit
pub type COM2_VALUE_R = crate::BitReader<bool>;
///Field `COM2_LOCK` reader - COMP2_CSR register lock bit
pub type COM2_LOCK_R = crate::BitReader<bool>;
///Field `COM2_LOCK` writer - COMP2_CSR register lock bit
pub type COM2_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Comparator 2 enable bit
    #[inline(always)]
    pub fn com2_en(&self) -> COM2_EN_R {
        COM2_EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:7 - Comparator 2 Input Minus connection configuration bit
    #[inline(always)]
    pub fn com2_inmsel(&self) -> COM2_INMSEL_R {
        COM2_INMSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Comparator 2 input plus selection bit
    #[inline(always)]
    pub fn com2_inpsel(&self) -> COM2_INPSEL_R {
        COM2_INPSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - COM2_WINMODE
    #[inline(always)]
    pub fn com2_winmode(&self) -> COM2_WINMODE_R {
        COM2_WINMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - COM2_WINOUT
    #[inline(always)]
    pub fn com2_winout(&self) -> COM2_WINOUT_R {
        COM2_WINOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Comparator 2 polarity selection bit
    #[inline(always)]
    pub fn com2_polarity(&self) -> COM2_POLARITY_R {
        COM2_POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 2 hysteresis selection bits
    #[inline(always)]
    pub fn com2_hyst(&self) -> COM2_HYST_R {
        COM2_HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - COM2_PWRMODE
    #[inline(always)]
    pub fn com2_pwrmode(&self) -> COM2_PWRMODE_R {
        COM2_PWRMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:24 - COM2_BLANKSEL
    #[inline(always)]
    pub fn com2_blanksel(&self) -> COM2_BLANKSEL_R {
        COM2_BLANKSEL_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bit 30 - Comparator 2 output status bit
    #[inline(always)]
    pub fn com2_value(&self) -> COM2_VALUE_R {
        COM2_VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - COMP2_CSR register lock bit
    #[inline(always)]
    pub fn com2_lock(&self) -> COM2_LOCK_R {
        COM2_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Comparator 2 enable bit
    #[inline(always)]
    #[must_use]
    pub fn com2_en(&mut self) -> COM2_EN_W<0> {
        COM2_EN_W::new(self)
    }
    ///Bits 4:7 - Comparator 2 Input Minus connection configuration bit
    #[inline(always)]
    #[must_use]
    pub fn com2_inmsel(&mut self) -> COM2_INMSEL_W<4> {
        COM2_INMSEL_W::new(self)
    }
    ///Bits 8:9 - Comparator 2 input plus selection bit
    #[inline(always)]
    #[must_use]
    pub fn com2_inpsel(&mut self) -> COM2_INPSEL_W<8> {
        COM2_INPSEL_W::new(self)
    }
    ///Bit 11 - COM2_WINMODE
    #[inline(always)]
    #[must_use]
    pub fn com2_winmode(&mut self) -> COM2_WINMODE_W<11> {
        COM2_WINMODE_W::new(self)
    }
    ///Bit 14 - COM2_WINOUT
    #[inline(always)]
    #[must_use]
    pub fn com2_winout(&mut self) -> COM2_WINOUT_W<14> {
        COM2_WINOUT_W::new(self)
    }
    ///Bit 15 - Comparator 2 polarity selection bit
    #[inline(always)]
    #[must_use]
    pub fn com2_polarity(&mut self) -> COM2_POLARITY_W<15> {
        COM2_POLARITY_W::new(self)
    }
    ///Bits 16:17 - Comparator 2 hysteresis selection bits
    #[inline(always)]
    #[must_use]
    pub fn com2_hyst(&mut self) -> COM2_HYST_W<16> {
        COM2_HYST_W::new(self)
    }
    ///Bits 18:19 - COM2_PWRMODE
    #[inline(always)]
    #[must_use]
    pub fn com2_pwrmode(&mut self) -> COM2_PWRMODE_W<18> {
        COM2_PWRMODE_W::new(self)
    }
    ///Bits 20:24 - COM2_BLANKSEL
    #[inline(always)]
    #[must_use]
    pub fn com2_blanksel(&mut self) -> COM2_BLANKSEL_W<20> {
        COM2_BLANKSEL_W::new(self)
    }
    ///Bit 31 - COMP2_CSR register lock bit
    #[inline(always)]
    #[must_use]
    pub fn com2_lock(&mut self) -> COM2_LOCK_W<31> {
        COM2_LOCK_W::new(self)
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
