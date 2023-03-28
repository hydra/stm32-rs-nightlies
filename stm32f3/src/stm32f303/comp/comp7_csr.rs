///Register `COMP7_CSR` reader
pub struct R(crate::R<COMP7_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP7_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP7_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP7_CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `COMP7_CSR` writer
pub struct W(crate::W<COMP7_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP7_CSR_SPEC>;
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
impl From<crate::W<COMP7_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP7_CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `COMP7EN` reader - Comparator 7 enable
pub type COMP7EN_R = crate::BitReader<bool>;
///Field `COMP7EN` writer - Comparator 7 enable
pub type COMP7EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP7_CSR_SPEC, bool, O>;
///Field `COMP7MODE` reader - Comparator 7 mode
pub type COMP7MODE_R = crate::FieldReader<u8, u8>;
///Field `COMP7MODE` writer - Comparator 7 mode
pub type COMP7MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP7_CSR_SPEC, u8, u8, 2, O>;
///Field `COMP7INMSEL` reader - Comparator 7 inverting input selection
pub type COMP7INMSEL_R = crate::FieldReader<u8, u8>;
///Field `COMP7INMSEL` writer - Comparator 7 inverting input selection
pub type COMP7INMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP7_CSR_SPEC, u8, u8, 3, O>;
///Field `COMP7INPSEL` reader - Comparator 7 non inverted input
pub type COMP7INPSEL_R = crate::BitReader<bool>;
///Field `COMP7INPSEL` writer - Comparator 7 non inverted input
pub type COMP7INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP7_CSR_SPEC, bool, O>;
///Field `COMP7OUTSEL` reader - Comparator 7 output selection
pub type COMP7OUTSEL_R = crate::FieldReader<u8, u8>;
///Field `COMP7OUTSEL` writer - Comparator 7 output selection
pub type COMP7OUTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP7_CSR_SPEC, u8, u8, 4, O>;
///Field `COMP7POL` reader - Comparator 7 output polarity
pub type COMP7POL_R = crate::BitReader<bool>;
///Field `COMP7POL` writer - Comparator 7 output polarity
pub type COMP7POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP7_CSR_SPEC, bool, O>;
///Field `COMP7HYST` reader - Comparator 7 hysteresis
pub type COMP7HYST_R = crate::FieldReader<u8, u8>;
///Field `COMP7HYST` writer - Comparator 7 hysteresis
pub type COMP7HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP7_CSR_SPEC, u8, u8, 2, O>;
///Field `COMP7_BLANKING` reader - Comparator 7 blanking source
pub type COMP7_BLANKING_R = crate::FieldReader<u8, u8>;
///Field `COMP7_BLANKING` writer - Comparator 7 blanking source
pub type COMP7_BLANKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP7_CSR_SPEC, u8, u8, 3, O>;
///Field `COMP7OUT` reader - Comparator 7 output
pub type COMP7OUT_R = crate::BitReader<bool>;
///Field `COMP7LOCK` reader - Comparator 7 lock
pub type COMP7LOCK_R = crate::BitReader<bool>;
///Field `COMP7LOCK` writer - Comparator 7 lock
pub type COMP7LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP7_CSR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Comparator 7 enable
    #[inline(always)]
    pub fn comp7en(&self) -> COMP7EN_R {
        COMP7EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - Comparator 7 mode
    #[inline(always)]
    pub fn comp7mode(&self) -> COMP7MODE_R {
        COMP7MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Comparator 7 inverting input selection
    #[inline(always)]
    pub fn comp7inmsel(&self) -> COMP7INMSEL_R {
        COMP7INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Comparator 7 non inverted input
    #[inline(always)]
    pub fn comp7inpsel(&self) -> COMP7INPSEL_R {
        COMP7INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 10:13 - Comparator 7 output selection
    #[inline(always)]
    pub fn comp7outsel(&self) -> COMP7OUTSEL_R {
        COMP7OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 15 - Comparator 7 output polarity
    #[inline(always)]
    pub fn comp7pol(&self) -> COMP7POL_R {
        COMP7POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 7 hysteresis
    #[inline(always)]
    pub fn comp7hyst(&self) -> COMP7HYST_R {
        COMP7HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - Comparator 7 blanking source
    #[inline(always)]
    pub fn comp7_blanking(&self) -> COMP7_BLANKING_R {
        COMP7_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 30 - Comparator 7 output
    #[inline(always)]
    pub fn comp7out(&self) -> COMP7OUT_R {
        COMP7OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Comparator 7 lock
    #[inline(always)]
    pub fn comp7lock(&self) -> COMP7LOCK_R {
        COMP7LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Comparator 7 enable
    #[inline(always)]
    #[must_use]
    pub fn comp7en(&mut self) -> COMP7EN_W<0> {
        COMP7EN_W::new(self)
    }
    ///Bits 2:3 - Comparator 7 mode
    #[inline(always)]
    #[must_use]
    pub fn comp7mode(&mut self) -> COMP7MODE_W<2> {
        COMP7MODE_W::new(self)
    }
    ///Bits 4:6 - Comparator 7 inverting input selection
    #[inline(always)]
    #[must_use]
    pub fn comp7inmsel(&mut self) -> COMP7INMSEL_W<4> {
        COMP7INMSEL_W::new(self)
    }
    ///Bit 7 - Comparator 7 non inverted input
    #[inline(always)]
    #[must_use]
    pub fn comp7inpsel(&mut self) -> COMP7INPSEL_W<7> {
        COMP7INPSEL_W::new(self)
    }
    ///Bits 10:13 - Comparator 7 output selection
    #[inline(always)]
    #[must_use]
    pub fn comp7outsel(&mut self) -> COMP7OUTSEL_W<10> {
        COMP7OUTSEL_W::new(self)
    }
    ///Bit 15 - Comparator 7 output polarity
    #[inline(always)]
    #[must_use]
    pub fn comp7pol(&mut self) -> COMP7POL_W<15> {
        COMP7POL_W::new(self)
    }
    ///Bits 16:17 - Comparator 7 hysteresis
    #[inline(always)]
    #[must_use]
    pub fn comp7hyst(&mut self) -> COMP7HYST_W<16> {
        COMP7HYST_W::new(self)
    }
    ///Bits 18:20 - Comparator 7 blanking source
    #[inline(always)]
    #[must_use]
    pub fn comp7_blanking(&mut self) -> COMP7_BLANKING_W<18> {
        COMP7_BLANKING_W::new(self)
    }
    ///Bit 31 - Comparator 7 lock
    #[inline(always)]
    #[must_use]
    pub fn comp7lock(&mut self) -> COMP7LOCK_W<31> {
        COMP7LOCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [comp7_csr](index.html) module
pub struct COMP7_CSR_SPEC;
impl crate::RegisterSpec for COMP7_CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [comp7_csr::R](R) reader structure
impl crate::Readable for COMP7_CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [comp7_csr::W](W) writer structure
impl crate::Writable for COMP7_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets COMP7_CSR to value 0
impl crate::Resettable for COMP7_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
