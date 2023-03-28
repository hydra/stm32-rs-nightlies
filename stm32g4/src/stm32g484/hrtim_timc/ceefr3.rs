///Register `CEEFR3` reader
pub struct R(crate::R<CEEFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEEFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEEFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEEFR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CEEFR3` writer
pub struct W(crate::W<CEEFR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEEFR3_SPEC>;
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
impl From<crate::W<CEEFR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEEFR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EEVACE` reader - External Event A Counter Enable
pub type EEVACE_R = crate::BitReader<bool>;
///Field `EEVACE` writer - External Event A Counter Enable
pub type EEVACE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEEFR3_SPEC, bool, O>;
///Field `EEVACRES` reader - External Event A Counter Reset
pub type EEVACRES_R = crate::BitReader<bool>;
///Field `EEVACRES` writer - External Event A Counter Reset
pub type EEVACRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEEFR3_SPEC, bool, O>;
///Field `EEVARSTM` reader - External Event A Reset Mode
pub type EEVARSTM_R = crate::BitReader<bool>;
///Field `EEVARSTM` writer - External Event A Reset Mode
pub type EEVARSTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEEFR3_SPEC, bool, O>;
///Field `EEVASEL` reader - External Event A Selection
pub type EEVASEL_R = crate::FieldReader<u8, u8>;
///Field `EEVASEL` writer - External Event A Selection
pub type EEVASEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEEFR3_SPEC, u8, u8, 4, O>;
///Field `EEVACNT` reader - External Event A counter
pub type EEVACNT_R = crate::FieldReader<u8, u8>;
///Field `EEVACNT` writer - External Event A counter
pub type EEVACNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEEFR3_SPEC, u8, u8, 6, O>;
impl R {
    ///Bit 0 - External Event A Counter Enable
    #[inline(always)]
    pub fn eevace(&self) -> EEVACE_R {
        EEVACE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - External Event A Counter Reset
    #[inline(always)]
    pub fn eevacres(&self) -> EEVACRES_R {
        EEVACRES_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - External Event A Reset Mode
    #[inline(always)]
    pub fn eevarstm(&self) -> EEVARSTM_R {
        EEVARSTM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:7 - External Event A Selection
    #[inline(always)]
    pub fn eevasel(&self) -> EEVASEL_R {
        EEVASEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:13 - External Event A counter
    #[inline(always)]
    pub fn eevacnt(&self) -> EEVACNT_R {
        EEVACNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    ///Bit 0 - External Event A Counter Enable
    #[inline(always)]
    #[must_use]
    pub fn eevace(&mut self) -> EEVACE_W<0> {
        EEVACE_W::new(self)
    }
    ///Bit 1 - External Event A Counter Reset
    #[inline(always)]
    #[must_use]
    pub fn eevacres(&mut self) -> EEVACRES_W<1> {
        EEVACRES_W::new(self)
    }
    ///Bit 2 - External Event A Reset Mode
    #[inline(always)]
    #[must_use]
    pub fn eevarstm(&mut self) -> EEVARSTM_W<2> {
        EEVARSTM_W::new(self)
    }
    ///Bits 4:7 - External Event A Selection
    #[inline(always)]
    #[must_use]
    pub fn eevasel(&mut self) -> EEVASEL_W<4> {
        EEVASEL_W::new(self)
    }
    ///Bits 8:13 - External Event A counter
    #[inline(always)]
    #[must_use]
    pub fn eevacnt(&mut self) -> EEVACNT_W<8> {
        EEVACNT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HRTIM Timerx External Event Filtering Register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ceefr3](index.html) module
pub struct CEEFR3_SPEC;
impl crate::RegisterSpec for CEEFR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [ceefr3::R](R) reader structure
impl crate::Readable for CEEFR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ceefr3::W](W) writer structure
impl crate::Writable for CEEFR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CEEFR3 to value 0
impl crate::Resettable for CEEFR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
