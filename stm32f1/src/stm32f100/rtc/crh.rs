///Register `CRH` reader
pub struct R(crate::R<CRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRH_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRH` writer
pub struct W(crate::W<CRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRH_SPEC>;
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
impl From<crate::W<CRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRH_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SECIE` reader - Second interrupt Enable
pub type SECIE_R = crate::BitReader<bool>;
///Field `SECIE` writer - Second interrupt Enable
pub type SECIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRH_SPEC, bool, O>;
///Field `ALRIE` reader - Alarm interrupt Enable
pub type ALRIE_R = crate::BitReader<bool>;
///Field `ALRIE` writer - Alarm interrupt Enable
pub type ALRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRH_SPEC, bool, O>;
///Field `OWIE` reader - Overflow interrupt Enable
pub type OWIE_R = crate::BitReader<bool>;
///Field `OWIE` writer - Overflow interrupt Enable
pub type OWIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRH_SPEC, bool, O>;
impl R {
    ///Bit 0 - Second interrupt Enable
    #[inline(always)]
    pub fn secie(&self) -> SECIE_R {
        SECIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm interrupt Enable
    #[inline(always)]
    pub fn alrie(&self) -> ALRIE_R {
        ALRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Overflow interrupt Enable
    #[inline(always)]
    pub fn owie(&self) -> OWIE_R {
        OWIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Second interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn secie(&mut self) -> SECIE_W<0> {
        SECIE_W::new(self)
    }
    ///Bit 1 - Alarm interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn alrie(&mut self) -> ALRIE_W<1> {
        ALRIE_W::new(self)
    }
    ///Bit 2 - Overflow interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn owie(&mut self) -> OWIE_W<2> {
        OWIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC Control Register High
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crh](index.html) module
pub struct CRH_SPEC;
impl crate::RegisterSpec for CRH_SPEC {
    type Ux = u32;
}
///`read()` method returns [crh::R](R) reader structure
impl crate::Readable for CRH_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [crh::W](W) writer structure
impl crate::Writable for CRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CRH to value 0
impl crate::Resettable for CRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
