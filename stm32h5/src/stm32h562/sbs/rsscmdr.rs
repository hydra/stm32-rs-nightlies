///Register `RSSCMDR` reader
pub struct R(crate::R<RSSCMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSSCMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSSCMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSSCMDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RSSCMDR` writer
pub struct W(crate::W<RSSCMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSSCMDR_SPEC>;
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
impl From<crate::W<RSSCMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSSCMDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RSSCMD` reader - RSS command The application can use this bitfield to pass on a command to the RSS, executed at the next reset. When RSSCMD ≠ 0 and PRODUCT_STATE is in Open, then the system always boots on RSS whatever is the boot pin value.
pub type RSSCMD_R = crate::FieldReader<u16, u16>;
///Field `RSSCMD` writer - RSS command The application can use this bitfield to pass on a command to the RSS, executed at the next reset. When RSSCMD ≠ 0 and PRODUCT_STATE is in Open, then the system always boots on RSS whatever is the boot pin value.
pub type RSSCMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSSCMDR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - RSS command The application can use this bitfield to pass on a command to the RSS, executed at the next reset. When RSSCMD ≠ 0 and PRODUCT_STATE is in Open, then the system always boots on RSS whatever is the boot pin value.
    #[inline(always)]
    pub fn rsscmd(&self) -> RSSCMD_R {
        RSSCMD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - RSS command The application can use this bitfield to pass on a command to the RSS, executed at the next reset. When RSSCMD ≠ 0 and PRODUCT_STATE is in Open, then the system always boots on RSS whatever is the boot pin value.
    #[inline(always)]
    #[must_use]
    pub fn rsscmd(&mut self) -> RSSCMD_W<0> {
        RSSCMD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SBS RSS command register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rsscmdr](index.html) module
pub struct RSSCMDR_SPEC;
impl crate::RegisterSpec for RSSCMDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rsscmdr::R](R) reader structure
impl crate::Readable for RSSCMDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rsscmdr::W](W) writer structure
impl crate::Writable for RSSCMDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RSSCMDR to value 0
impl crate::Resettable for RSSCMDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
