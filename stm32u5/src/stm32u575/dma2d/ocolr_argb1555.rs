///Register `OCOLR_ARGB1555` reader
pub struct R(crate::R<OCOLR_ARGB1555_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCOLR_ARGB1555_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCOLR_ARGB1555_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCOLR_ARGB1555_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OCOLR_ARGB1555` writer
pub struct W(crate::W<OCOLR_ARGB1555_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCOLR_ARGB1555_SPEC>;
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
impl From<crate::W<OCOLR_ARGB1555_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCOLR_ARGB1555_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BLUE` reader - Blue value in ARGB1555 mode
pub type BLUE_R = crate::FieldReader<u8, u8>;
///Field `BLUE` writer - Blue value in ARGB1555 mode
pub type BLUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCOLR_ARGB1555_SPEC, u8, u8, 5, O>;
///Field `GREEN` reader - Green value in ARGB1555 mode
pub type GREEN_R = crate::FieldReader<u8, u8>;
///Field `GREEN` writer - Green value in ARGB1555 mode
pub type GREEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCOLR_ARGB1555_SPEC, u8, u8, 5, O>;
///Field `RED` reader - Red value in ARGB1555 mode
pub type RED_R = crate::FieldReader<u8, u8>;
///Field `RED` writer - Red value in ARGB1555 mode
pub type RED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCOLR_ARGB1555_SPEC, u8, u8, 5, O>;
///Field `A` reader - Alpha channel value in ARGB1555 mode
pub type A_R = crate::BitReader<bool>;
///Field `A` writer - Alpha channel value in ARGB1555 mode
pub type A_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCOLR_ARGB1555_SPEC, bool, O>;
impl R {
    ///Bits 0:4 - Blue value in ARGB1555 mode
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - Green value in ARGB1555 mode
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - Red value in ARGB1555 mode
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bit 15 - Alpha channel value in ARGB1555 mode
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - Blue value in ARGB1555 mode
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BLUE_W<0> {
        BLUE_W::new(self)
    }
    ///Bits 5:9 - Green value in ARGB1555 mode
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GREEN_W<5> {
        GREEN_W::new(self)
    }
    ///Bits 10:14 - Red value in ARGB1555 mode
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<10> {
        RED_W::new(self)
    }
    ///Bit 15 - Alpha channel value in ARGB1555 mode
    #[inline(always)]
    #[must_use]
    pub fn a(&mut self) -> A_W<15> {
        A_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///output color register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ocolr_argb1555](index.html) module
pub struct OCOLR_ARGB1555_SPEC;
impl crate::RegisterSpec for OCOLR_ARGB1555_SPEC {
    type Ux = u32;
}
///`read()` method returns [ocolr_argb1555::R](R) reader structure
impl crate::Readable for OCOLR_ARGB1555_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ocolr_argb1555::W](W) writer structure
impl crate::Writable for OCOLR_ARGB1555_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OCOLR_ARGB1555 to value 0
impl crate::Resettable for OCOLR_ARGB1555_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
