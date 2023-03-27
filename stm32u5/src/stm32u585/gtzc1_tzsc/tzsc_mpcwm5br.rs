///Register `TZSC_MPCWM5BR` reader
pub struct R(crate::R<TZSC_MPCWM5BR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_MPCWM5BR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_MPCWM5BR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_MPCWM5BR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TZSC_MPCWM5BR` writer
pub struct W(crate::W<TZSC_MPCWM5BR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_MPCWM5BR_SPEC>;
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
impl From<crate::W<TZSC_MPCWM5BR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_MPCWM5BR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SUBB_START` reader - Start of sub-region A
pub type SUBB_START_R = crate::FieldReader<u16, u16>;
///Field `SUBB_START` writer - Start of sub-region A
pub type SUBB_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZSC_MPCWM5BR_SPEC, u16, u16, 11, O>;
///Field `SUBB_LENGTH` reader - Length of sub-region A
pub type SUBB_LENGTH_R = crate::FieldReader<u16, u16>;
///Field `SUBB_LENGTH` writer - Length of sub-region A
pub type SUBB_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZSC_MPCWM5BR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:10 - Start of sub-region A
    #[inline(always)]
    pub fn subb_start(&self) -> SUBB_START_R {
        SUBB_START_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Length of sub-region A
    #[inline(always)]
    pub fn subb_length(&self) -> SUBB_LENGTH_R {
        SUBB_LENGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Start of sub-region A
    #[inline(always)]
    #[must_use]
    pub fn subb_start(&mut self) -> SUBB_START_W<0> {
        SUBB_START_W::new(self)
    }
    ///Bits 16:27 - Length of sub-region A
    #[inline(always)]
    #[must_use]
    pub fn subb_length(&mut self) -> SUBB_LENGTH_W<16> {
        SUBB_LENGTH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZSC memory 5 sub-region B watermark register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzsc_mpcwm5br](index.html) module
pub struct TZSC_MPCWM5BR_SPEC;
impl crate::RegisterSpec for TZSC_MPCWM5BR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzsc_mpcwm5br::R](R) reader structure
impl crate::Readable for TZSC_MPCWM5BR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tzsc_mpcwm5br::W](W) writer structure
impl crate::Writable for TZSC_MPCWM5BR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TZSC_MPCWM5BR to value 0
impl crate::Resettable for TZSC_MPCWM5BR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
