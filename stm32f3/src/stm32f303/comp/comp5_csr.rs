///Register `COMP5_CSR` reader
pub struct R(crate::R<COMP5_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP5_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP5_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP5_CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `COMP5_CSR` writer
pub struct W(crate::W<COMP5_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP5_CSR_SPEC>;
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
impl From<crate::W<COMP5_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP5_CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `COMP5EN` reader - Comparator 5 enable
pub type COMP5EN_R = crate::BitReader<bool>;
///Field `COMP5EN` writer - Comparator 5 enable
pub type COMP5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP5_CSR_SPEC, bool, O>;
///Field `COMP5MODE` reader - Comparator 5 mode
pub type COMP5MODE_R = crate::FieldReader<u8, u8>;
///Field `COMP5MODE` writer - Comparator 5 mode
pub type COMP5MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP5_CSR_SPEC, u8, u8, 2, O>;
///Field `COMP5INMSEL` reader - Comparator 5 inverting input selection
pub type COMP5INMSEL_R = crate::FieldReader<u8, u8>;
///Field `COMP5INMSEL` writer - Comparator 5 inverting input selection
pub type COMP5INMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP5_CSR_SPEC, u8, u8, 3, O>;
///Field `COMP5INPSEL` reader - Comparator 5 non inverted input
pub type COMP5INPSEL_R = crate::BitReader<bool>;
///Field `COMP5INPSEL` writer - Comparator 5 non inverted input
pub type COMP5INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP5_CSR_SPEC, bool, O>;
///Field `COMP5OUTSEL` reader - Comparator 5 output selection
pub type COMP5OUTSEL_R = crate::FieldReader<u8, u8>;
///Field `COMP5OUTSEL` writer - Comparator 5 output selection
pub type COMP5OUTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP5_CSR_SPEC, u8, u8, 4, O>;
///Field `COMP5POL` reader - Comparator 5 output polarity
pub type COMP5POL_R = crate::BitReader<bool>;
///Field `COMP5POL` writer - Comparator 5 output polarity
pub type COMP5POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP5_CSR_SPEC, bool, O>;
///Field `COMP5HYST` reader - Comparator 5 hysteresis
pub type COMP5HYST_R = crate::FieldReader<u8, u8>;
///Field `COMP5HYST` writer - Comparator 5 hysteresis
pub type COMP5HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP5_CSR_SPEC, u8, u8, 2, O>;
///Field `COMP5_BLANKING` reader - Comparator 5 blanking source
pub type COMP5_BLANKING_R = crate::FieldReader<u8, u8>;
///Field `COMP5_BLANKING` writer - Comparator 5 blanking source
pub type COMP5_BLANKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP5_CSR_SPEC, u8, u8, 3, O>;
///Field `COMP5OUT` reader - Comparator 5 output
pub type COMP5OUT_R = crate::BitReader<bool>;
///Field `COMP5LOCK` reader - Comparator 5 lock
pub type COMP5LOCK_R = crate::BitReader<bool>;
///Field `COMP5LOCK` writer - Comparator 5 lock
pub type COMP5LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP5_CSR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Comparator 5 enable
    #[inline(always)]
    pub fn comp5en(&self) -> COMP5EN_R {
        COMP5EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - Comparator 5 mode
    #[inline(always)]
    pub fn comp5mode(&self) -> COMP5MODE_R {
        COMP5MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Comparator 5 inverting input selection
    #[inline(always)]
    pub fn comp5inmsel(&self) -> COMP5INMSEL_R {
        COMP5INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Comparator 5 non inverted input
    #[inline(always)]
    pub fn comp5inpsel(&self) -> COMP5INPSEL_R {
        COMP5INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 10:13 - Comparator 5 output selection
    #[inline(always)]
    pub fn comp5outsel(&self) -> COMP5OUTSEL_R {
        COMP5OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 15 - Comparator 5 output polarity
    #[inline(always)]
    pub fn comp5pol(&self) -> COMP5POL_R {
        COMP5POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 5 hysteresis
    #[inline(always)]
    pub fn comp5hyst(&self) -> COMP5HYST_R {
        COMP5HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - Comparator 5 blanking source
    #[inline(always)]
    pub fn comp5_blanking(&self) -> COMP5_BLANKING_R {
        COMP5_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 30 - Comparator 5 output
    #[inline(always)]
    pub fn comp5out(&self) -> COMP5OUT_R {
        COMP5OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Comparator 5 lock
    #[inline(always)]
    pub fn comp5lock(&self) -> COMP5LOCK_R {
        COMP5LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Comparator 5 enable
    #[inline(always)]
    #[must_use]
    pub fn comp5en(&mut self) -> COMP5EN_W<0> {
        COMP5EN_W::new(self)
    }
    ///Bits 2:3 - Comparator 5 mode
    #[inline(always)]
    #[must_use]
    pub fn comp5mode(&mut self) -> COMP5MODE_W<2> {
        COMP5MODE_W::new(self)
    }
    ///Bits 4:6 - Comparator 5 inverting input selection
    #[inline(always)]
    #[must_use]
    pub fn comp5inmsel(&mut self) -> COMP5INMSEL_W<4> {
        COMP5INMSEL_W::new(self)
    }
    ///Bit 7 - Comparator 5 non inverted input
    #[inline(always)]
    #[must_use]
    pub fn comp5inpsel(&mut self) -> COMP5INPSEL_W<7> {
        COMP5INPSEL_W::new(self)
    }
    ///Bits 10:13 - Comparator 5 output selection
    #[inline(always)]
    #[must_use]
    pub fn comp5outsel(&mut self) -> COMP5OUTSEL_W<10> {
        COMP5OUTSEL_W::new(self)
    }
    ///Bit 15 - Comparator 5 output polarity
    #[inline(always)]
    #[must_use]
    pub fn comp5pol(&mut self) -> COMP5POL_W<15> {
        COMP5POL_W::new(self)
    }
    ///Bits 16:17 - Comparator 5 hysteresis
    #[inline(always)]
    #[must_use]
    pub fn comp5hyst(&mut self) -> COMP5HYST_W<16> {
        COMP5HYST_W::new(self)
    }
    ///Bits 18:20 - Comparator 5 blanking source
    #[inline(always)]
    #[must_use]
    pub fn comp5_blanking(&mut self) -> COMP5_BLANKING_W<18> {
        COMP5_BLANKING_W::new(self)
    }
    ///Bit 31 - Comparator 5 lock
    #[inline(always)]
    #[must_use]
    pub fn comp5lock(&mut self) -> COMP5LOCK_W<31> {
        COMP5LOCK_W::new(self)
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
///For information about available fields see [comp5_csr](index.html) module
pub struct COMP5_CSR_SPEC;
impl crate::RegisterSpec for COMP5_CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [comp5_csr::R](R) reader structure
impl crate::Readable for COMP5_CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [comp5_csr::W](W) writer structure
impl crate::Writable for COMP5_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets COMP5_CSR to value 0
impl crate::Resettable for COMP5_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
