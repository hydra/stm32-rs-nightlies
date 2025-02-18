///Register `EECR3` reader
pub struct R(crate::R<EECR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EECR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EECR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EECR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EECR3` writer
pub struct W(crate::W<EECR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EECR3_SPEC>;
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
impl From<crate::W<EECR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EECR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EE6SRC` reader - EE6SRC
pub type EE6SRC_R = crate::FieldReader<u8, u8>;
///Field `EE6SRC` writer - EE6SRC
pub type EE6SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 2, O>;
///Field `EE6POL` reader - EE6POL
pub type EE6POL_R = crate::BitReader<bool>;
///Field `EE6POL` writer - EE6POL
pub type EE6POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR3_SPEC, bool, O>;
///Field `EE6SNS` reader - EE6SNS
pub type EE6SNS_R = crate::FieldReader<u8, u8>;
///Field `EE6SNS` writer - EE6SNS
pub type EE6SNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 2, O>;
///Field `EE7SRC` reader - EE7SRC
pub type EE7SRC_R = crate::FieldReader<u8, u8>;
///Field `EE7SRC` writer - EE7SRC
pub type EE7SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 2, O>;
///Field `EE7POL` reader - EE7POL
pub type EE7POL_R = crate::BitReader<bool>;
///Field `EE7POL` writer - EE7POL
pub type EE7POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR3_SPEC, bool, O>;
///Field `EE7SNS` reader - EE7SNS
pub type EE7SNS_R = crate::FieldReader<u8, u8>;
///Field `EE7SNS` writer - EE7SNS
pub type EE7SNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 2, O>;
///Field `EE8SRC` reader - EE8SRC
pub type EE8SRC_R = crate::FieldReader<u8, u8>;
///Field `EE8SRC` writer - EE8SRC
pub type EE8SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 2, O>;
///Field `EE8POL` reader - EE8POL
pub type EE8POL_R = crate::BitReader<bool>;
///Field `EE8POL` writer - EE8POL
pub type EE8POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR3_SPEC, bool, O>;
///Field `EE8SNS` reader - EE8SNS
pub type EE8SNS_R = crate::FieldReader<u8, u8>;
///Field `EE8SNS` writer - EE8SNS
pub type EE8SNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 2, O>;
///Field `EE9SRC` reader - EE9SRC
pub type EE9SRC_R = crate::FieldReader<u8, u8>;
///Field `EE9SRC` writer - EE9SRC
pub type EE9SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 2, O>;
///Field `EE9POL` reader - EE9POL
pub type EE9POL_R = crate::BitReader<bool>;
///Field `EE9POL` writer - EE9POL
pub type EE9POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR3_SPEC, bool, O>;
///Field `EE9SNS` reader - EE9SNS
pub type EE9SNS_R = crate::FieldReader<u8, u8>;
///Field `EE9SNS` writer - EE9SNS
pub type EE9SNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 2, O>;
///Field `EE10SRC` reader - EE10SRC
pub type EE10SRC_R = crate::FieldReader<u8, u8>;
///Field `EE10SRC` writer - EE10SRC
pub type EE10SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 2, O>;
///Field `EE10POL` reader - EE10POL
pub type EE10POL_R = crate::BitReader<bool>;
///Field `EE10POL` writer - EE10POL
pub type EE10POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR3_SPEC, bool, O>;
///Field `EE10SNS` reader - EE10SNS
pub type EE10SNS_R = crate::FieldReader<u8, u8>;
///Field `EE10SNS` writer - EE10SNS
pub type EE10SNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR3_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:1 - EE6SRC
    #[inline(always)]
    pub fn ee6src(&self) -> EE6SRC_R {
        EE6SRC_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - EE6POL
    #[inline(always)]
    pub fn ee6pol(&self) -> EE6POL_R {
        EE6POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - EE6SNS
    #[inline(always)]
    pub fn ee6sns(&self) -> EE6SNS_R {
        EE6SNS_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 6:7 - EE7SRC
    #[inline(always)]
    pub fn ee7src(&self) -> EE7SRC_R {
        EE7SRC_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - EE7POL
    #[inline(always)]
    pub fn ee7pol(&self) -> EE7POL_R {
        EE7POL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - EE7SNS
    #[inline(always)]
    pub fn ee7sns(&self) -> EE7SNS_R {
        EE7SNS_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 12:13 - EE8SRC
    #[inline(always)]
    pub fn ee8src(&self) -> EE8SRC_R {
        EE8SRC_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - EE8POL
    #[inline(always)]
    pub fn ee8pol(&self) -> EE8POL_R {
        EE8POL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 15:16 - EE8SNS
    #[inline(always)]
    pub fn ee8sns(&self) -> EE8SNS_R {
        EE8SNS_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bits 18:19 - EE9SRC
    #[inline(always)]
    pub fn ee9src(&self) -> EE9SRC_R {
        EE9SRC_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20 - EE9POL
    #[inline(always)]
    pub fn ee9pol(&self) -> EE9POL_R {
        EE9POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - EE9SNS
    #[inline(always)]
    pub fn ee9sns(&self) -> EE9SNS_R {
        EE9SNS_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bits 24:25 - EE10SRC
    #[inline(always)]
    pub fn ee10src(&self) -> EE10SRC_R {
        EE10SRC_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - EE10POL
    #[inline(always)]
    pub fn ee10pol(&self) -> EE10POL_R {
        EE10POL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:28 - EE10SNS
    #[inline(always)]
    pub fn ee10sns(&self) -> EE10SNS_R {
        EE10SNS_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - EE6SRC
    #[inline(always)]
    #[must_use]
    pub fn ee6src(&mut self) -> EE6SRC_W<0> {
        EE6SRC_W::new(self)
    }
    ///Bit 2 - EE6POL
    #[inline(always)]
    #[must_use]
    pub fn ee6pol(&mut self) -> EE6POL_W<2> {
        EE6POL_W::new(self)
    }
    ///Bits 3:4 - EE6SNS
    #[inline(always)]
    #[must_use]
    pub fn ee6sns(&mut self) -> EE6SNS_W<3> {
        EE6SNS_W::new(self)
    }
    ///Bits 6:7 - EE7SRC
    #[inline(always)]
    #[must_use]
    pub fn ee7src(&mut self) -> EE7SRC_W<6> {
        EE7SRC_W::new(self)
    }
    ///Bit 8 - EE7POL
    #[inline(always)]
    #[must_use]
    pub fn ee7pol(&mut self) -> EE7POL_W<8> {
        EE7POL_W::new(self)
    }
    ///Bits 9:10 - EE7SNS
    #[inline(always)]
    #[must_use]
    pub fn ee7sns(&mut self) -> EE7SNS_W<9> {
        EE7SNS_W::new(self)
    }
    ///Bits 12:13 - EE8SRC
    #[inline(always)]
    #[must_use]
    pub fn ee8src(&mut self) -> EE8SRC_W<12> {
        EE8SRC_W::new(self)
    }
    ///Bit 14 - EE8POL
    #[inline(always)]
    #[must_use]
    pub fn ee8pol(&mut self) -> EE8POL_W<14> {
        EE8POL_W::new(self)
    }
    ///Bits 15:16 - EE8SNS
    #[inline(always)]
    #[must_use]
    pub fn ee8sns(&mut self) -> EE8SNS_W<15> {
        EE8SNS_W::new(self)
    }
    ///Bits 18:19 - EE9SRC
    #[inline(always)]
    #[must_use]
    pub fn ee9src(&mut self) -> EE9SRC_W<18> {
        EE9SRC_W::new(self)
    }
    ///Bit 20 - EE9POL
    #[inline(always)]
    #[must_use]
    pub fn ee9pol(&mut self) -> EE9POL_W<20> {
        EE9POL_W::new(self)
    }
    ///Bits 21:22 - EE9SNS
    #[inline(always)]
    #[must_use]
    pub fn ee9sns(&mut self) -> EE9SNS_W<21> {
        EE9SNS_W::new(self)
    }
    ///Bits 24:25 - EE10SRC
    #[inline(always)]
    #[must_use]
    pub fn ee10src(&mut self) -> EE10SRC_W<24> {
        EE10SRC_W::new(self)
    }
    ///Bit 26 - EE10POL
    #[inline(always)]
    #[must_use]
    pub fn ee10pol(&mut self) -> EE10POL_W<26> {
        EE10POL_W::new(self)
    }
    ///Bits 27:28 - EE10SNS
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
///Timer External Event Control Register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eecr3](index.html) module
pub struct EECR3_SPEC;
impl crate::RegisterSpec for EECR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [eecr3::R](R) reader structure
impl crate::Readable for EECR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eecr3::W](W) writer structure
impl crate::Writable for EECR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EECR3 to value 0
impl crate::Resettable for EECR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
