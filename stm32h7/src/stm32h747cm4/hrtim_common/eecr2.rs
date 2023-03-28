///Register `EECR2` reader
pub struct R(crate::R<EECR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EECR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EECR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EECR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EECR2` writer
pub struct W(crate::W<EECR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EECR2_SPEC>;
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
impl From<crate::W<EECR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EECR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EE6SRC` reader - External Event 6 Source
pub type EE6SRC_R = crate::FieldReader<u8, u8>;
///Field `EE6SRC` writer - External Event 6 Source
pub type EE6SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR2_SPEC, u8, u8, 2, O>;
///Field `EE6POL` reader - External Event 6 Polarity
pub type EE6POL_R = crate::BitReader<bool>;
///Field `EE6POL` writer - External Event 6 Polarity
pub type EE6POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR2_SPEC, bool, O>;
///Field `EE6SNS` reader - External Event 6 Sensitivity
pub type EE6SNS_R = crate::FieldReader<u8, u8>;
///Field `EE6SNS` writer - External Event 6 Sensitivity
pub type EE6SNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR2_SPEC, u8, u8, 2, O>;
///Field `EE7SRC` reader - External Event 7 Source
pub type EE7SRC_R = crate::FieldReader<u8, u8>;
///Field `EE7SRC` writer - External Event 7 Source
pub type EE7SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR2_SPEC, u8, u8, 2, O>;
///Field `EE7POL` reader - External Event 7 Polarity
pub type EE7POL_R = crate::BitReader<bool>;
///Field `EE7POL` writer - External Event 7 Polarity
pub type EE7POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR2_SPEC, bool, O>;
///Field `EE7SNS` reader - External Event 7 Sensitivity
pub type EE7SNS_R = crate::FieldReader<u8, u8>;
///Field `EE7SNS` writer - External Event 7 Sensitivity
pub type EE7SNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR2_SPEC, u8, u8, 2, O>;
///Field `EE8SRC` reader - External Event 8 Source
pub type EE8SRC_R = crate::FieldReader<u8, u8>;
///Field `EE8SRC` writer - External Event 8 Source
pub type EE8SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR2_SPEC, u8, u8, 2, O>;
///Field `EE8POL` reader - External Event 8 Polarity
pub type EE8POL_R = crate::BitReader<bool>;
///Field `EE8POL` writer - External Event 8 Polarity
pub type EE8POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR2_SPEC, bool, O>;
///Field `EE8SNS` reader - External Event 8 Sensitivity
pub type EE8SNS_R = crate::FieldReader<u8, u8>;
///Field `EE8SNS` writer - External Event 8 Sensitivity
pub type EE8SNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR2_SPEC, u8, u8, 2, O>;
///Field `EE9SRC` reader - External Event 9 Source
pub type EE9SRC_R = crate::FieldReader<u8, u8>;
///Field `EE9SRC` writer - External Event 9 Source
pub type EE9SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR2_SPEC, u8, u8, 2, O>;
///Field `EE9POL` reader - External Event 9 Polarity
pub type EE9POL_R = crate::BitReader<bool>;
///Field `EE9POL` writer - External Event 9 Polarity
pub type EE9POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR2_SPEC, bool, O>;
///Field `EE9SNS` reader - External Event 9 Sensitivity
pub type EE9SNS_R = crate::FieldReader<u8, u8>;
///Field `EE9SNS` writer - External Event 9 Sensitivity
pub type EE9SNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR2_SPEC, u8, u8, 2, O>;
///Field `EE10SRC` reader - External Event 10 Source
pub type EE10SRC_R = crate::FieldReader<u8, u8>;
///Field `EE10SRC` writer - External Event 10 Source
pub type EE10SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR2_SPEC, u8, u8, 2, O>;
///Field `EE10POL` reader - External Event 10 Polarity
pub type EE10POL_R = crate::BitReader<bool>;
///Field `EE10POL` writer - External Event 10 Polarity
pub type EE10POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR2_SPEC, bool, O>;
///Field `EE10SNS` reader - External Event 10 Sensitivity
pub type EE10SNS_R = crate::FieldReader<u8, u8>;
///Field `EE10SNS` writer - External Event 10 Sensitivity
pub type EE10SNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR2_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:1 - External Event 6 Source
    #[inline(always)]
    pub fn ee6src(&self) -> EE6SRC_R {
        EE6SRC_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - External Event 6 Polarity
    #[inline(always)]
    pub fn ee6pol(&self) -> EE6POL_R {
        EE6POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - External Event 6 Sensitivity
    #[inline(always)]
    pub fn ee6sns(&self) -> EE6SNS_R {
        EE6SNS_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 6:7 - External Event 7 Source
    #[inline(always)]
    pub fn ee7src(&self) -> EE7SRC_R {
        EE7SRC_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - External Event 7 Polarity
    #[inline(always)]
    pub fn ee7pol(&self) -> EE7POL_R {
        EE7POL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - External Event 7 Sensitivity
    #[inline(always)]
    pub fn ee7sns(&self) -> EE7SNS_R {
        EE7SNS_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 12:13 - External Event 8 Source
    #[inline(always)]
    pub fn ee8src(&self) -> EE8SRC_R {
        EE8SRC_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - External Event 8 Polarity
    #[inline(always)]
    pub fn ee8pol(&self) -> EE8POL_R {
        EE8POL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 15:16 - External Event 8 Sensitivity
    #[inline(always)]
    pub fn ee8sns(&self) -> EE8SNS_R {
        EE8SNS_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bits 18:19 - External Event 9 Source
    #[inline(always)]
    pub fn ee9src(&self) -> EE9SRC_R {
        EE9SRC_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20 - External Event 9 Polarity
    #[inline(always)]
    pub fn ee9pol(&self) -> EE9POL_R {
        EE9POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - External Event 9 Sensitivity
    #[inline(always)]
    pub fn ee9sns(&self) -> EE9SNS_R {
        EE9SNS_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bits 24:25 - External Event 10 Source
    #[inline(always)]
    pub fn ee10src(&self) -> EE10SRC_R {
        EE10SRC_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - External Event 10 Polarity
    #[inline(always)]
    pub fn ee10pol(&self) -> EE10POL_R {
        EE10POL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:28 - External Event 10 Sensitivity
    #[inline(always)]
    pub fn ee10sns(&self) -> EE10SNS_R {
        EE10SNS_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - External Event 6 Source
    #[inline(always)]
    #[must_use]
    pub fn ee6src(&mut self) -> EE6SRC_W<0> {
        EE6SRC_W::new(self)
    }
    ///Bit 2 - External Event 6 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee6pol(&mut self) -> EE6POL_W<2> {
        EE6POL_W::new(self)
    }
    ///Bits 3:4 - External Event 6 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee6sns(&mut self) -> EE6SNS_W<3> {
        EE6SNS_W::new(self)
    }
    ///Bits 6:7 - External Event 7 Source
    #[inline(always)]
    #[must_use]
    pub fn ee7src(&mut self) -> EE7SRC_W<6> {
        EE7SRC_W::new(self)
    }
    ///Bit 8 - External Event 7 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee7pol(&mut self) -> EE7POL_W<8> {
        EE7POL_W::new(self)
    }
    ///Bits 9:10 - External Event 7 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee7sns(&mut self) -> EE7SNS_W<9> {
        EE7SNS_W::new(self)
    }
    ///Bits 12:13 - External Event 8 Source
    #[inline(always)]
    #[must_use]
    pub fn ee8src(&mut self) -> EE8SRC_W<12> {
        EE8SRC_W::new(self)
    }
    ///Bit 14 - External Event 8 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee8pol(&mut self) -> EE8POL_W<14> {
        EE8POL_W::new(self)
    }
    ///Bits 15:16 - External Event 8 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee8sns(&mut self) -> EE8SNS_W<15> {
        EE8SNS_W::new(self)
    }
    ///Bits 18:19 - External Event 9 Source
    #[inline(always)]
    #[must_use]
    pub fn ee9src(&mut self) -> EE9SRC_W<18> {
        EE9SRC_W::new(self)
    }
    ///Bit 20 - External Event 9 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee9pol(&mut self) -> EE9POL_W<20> {
        EE9POL_W::new(self)
    }
    ///Bits 21:22 - External Event 9 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee9sns(&mut self) -> EE9SNS_W<21> {
        EE9SNS_W::new(self)
    }
    ///Bits 24:25 - External Event 10 Source
    #[inline(always)]
    #[must_use]
    pub fn ee10src(&mut self) -> EE10SRC_W<24> {
        EE10SRC_W::new(self)
    }
    ///Bit 26 - External Event 10 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee10pol(&mut self) -> EE10POL_W<26> {
        EE10POL_W::new(self)
    }
    ///Bits 27:28 - External Event 10 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee10sns(&mut self) -> EE10SNS_W<27> {
        EE10SNS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timer External Event Control Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eecr2](index.html) module
pub struct EECR2_SPEC;
impl crate::RegisterSpec for EECR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [eecr2::R](R) reader structure
impl crate::Readable for EECR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eecr2::W](W) writer structure
impl crate::Writable for EECR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EECR2 to value 0
impl crate::Resettable for EECR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
