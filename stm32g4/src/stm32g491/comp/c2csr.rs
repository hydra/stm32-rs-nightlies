///Register `C2CSR` reader
pub struct R(crate::R<C2CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2CSR` writer
pub struct W(crate::W<C2CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2CSR_SPEC>;
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
impl From<crate::W<C2CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - EN
pub type EN_R = crate::BitReader<bool>;
///Field `EN` writer - EN
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CSR_SPEC, bool, O>;
///Field `COMP_DEGLITCH_EN` reader - COMP_DEGLITCH_EN
pub type COMP_DEGLITCH_EN_R = crate::BitReader<bool>;
///Field `COMP_DEGLITCH_EN` writer - COMP_DEGLITCH_EN
pub type COMP_DEGLITCH_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CSR_SPEC, bool, O>;
///Field `INMSEL` reader - INMSEL
pub type INMSEL_R = crate::FieldReader<u8, u8>;
///Field `INMSEL` writer - INMSEL
pub type INMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C2CSR_SPEC, u8, u8, 3, O>;
///Field `INPSEL` reader - INPSEL
pub type INPSEL_R = crate::BitReader<bool>;
///Field `INPSEL` writer - INPSEL
pub type INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CSR_SPEC, bool, O>;
///Field `POL` reader - POL
pub type POL_R = crate::BitReader<bool>;
///Field `POL` writer - POL
pub type POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CSR_SPEC, bool, O>;
///Field `HYST` reader - HYST
pub type HYST_R = crate::FieldReader<u8, u8>;
///Field `HYST` writer - HYST
pub type HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C2CSR_SPEC, u8, u8, 3, O>;
///Field `BLANKSEL` reader - BLANKSEL
pub type BLANKSEL_R = crate::FieldReader<u8, u8>;
///Field `BLANKSEL` writer - BLANKSEL
pub type BLANKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C2CSR_SPEC, u8, u8, 3, O>;
///Field `BRGEN` reader - BRGEN
pub type BRGEN_R = crate::BitReader<bool>;
///Field `BRGEN` writer - BRGEN
pub type BRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CSR_SPEC, bool, O>;
///Field `SCALEN` reader - SCALEN
pub type SCALEN_R = crate::BitReader<bool>;
///Field `SCALEN` writer - SCALEN
pub type SCALEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CSR_SPEC, bool, O>;
///Field `VALUE` reader - VALUE
pub type VALUE_R = crate::BitReader<bool>;
///Field `LOCK` reader - LOCK
pub type LOCK_R = crate::BitReader<bool>;
///Field `LOCK` writer - LOCK
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CSR_SPEC, bool, O>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - COMP_DEGLITCH_EN
    #[inline(always)]
    pub fn comp_deglitch_en(&self) -> COMP_DEGLITCH_EN_R {
        COMP_DEGLITCH_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:6 - INMSEL
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - INPSEL
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 15 - POL
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - HYST
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:21 - BLANKSEL
    #[inline(always)]
    pub fn blanksel(&self) -> BLANKSEL_R {
        BLANKSEL_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bit 22 - BRGEN
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SCALEN
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 30 - VALUE
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bit 1 - COMP_DEGLITCH_EN
    #[inline(always)]
    #[must_use]
    pub fn comp_deglitch_en(&mut self) -> COMP_DEGLITCH_EN_W<1> {
        COMP_DEGLITCH_EN_W::new(self)
    }
    ///Bits 4:6 - INMSEL
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<4> {
        INMSEL_W::new(self)
    }
    ///Bit 8 - INPSEL
    #[inline(always)]
    #[must_use]
    pub fn inpsel(&mut self) -> INPSEL_W<8> {
        INPSEL_W::new(self)
    }
    ///Bit 15 - POL
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<15> {
        POL_W::new(self)
    }
    ///Bits 16:18 - HYST
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<16> {
        HYST_W::new(self)
    }
    ///Bits 19:21 - BLANKSEL
    #[inline(always)]
    #[must_use]
    pub fn blanksel(&mut self) -> BLANKSEL_W<19> {
        BLANKSEL_W::new(self)
    }
    ///Bit 22 - BRGEN
    #[inline(always)]
    #[must_use]
    pub fn brgen(&mut self) -> BRGEN_W<22> {
        BRGEN_W::new(self)
    }
    ///Bit 23 - SCALEN
    #[inline(always)]
    #[must_use]
    pub fn scalen(&mut self) -> SCALEN_W<23> {
        SCALEN_W::new(self)
    }
    ///Bit 31 - LOCK
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
///Comparator control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2csr](index.html) module
pub struct C2CSR_SPEC;
impl crate::RegisterSpec for C2CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2csr::R](R) reader structure
impl crate::Readable for C2CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2csr::W](W) writer structure
impl crate::Writable for C2CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2CSR to value 0
impl crate::Resettable for C2CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
