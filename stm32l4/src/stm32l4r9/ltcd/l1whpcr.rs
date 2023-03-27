///Register `L1WHPCR` reader
pub struct R(crate::R<L1WHPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1WHPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1WHPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1WHPCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `L1WHPCR` writer
pub struct W(crate::W<L1WHPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1WHPCR_SPEC>;
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
impl From<crate::W<L1WHPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1WHPCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WHSTPOS` reader - Window Horizontal Start Position
pub type WHSTPOS_R = crate::FieldReader<u16, u16>;
///Field `WHSTPOS` writer - Window Horizontal Start Position
pub type WHSTPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, L1WHPCR_SPEC, u16, u16, 12, O>;
///Field `WHSPPOS` reader - Window Horizontal Stop Position
pub type WHSPPOS_R = crate::FieldReader<u16, u16>;
///Field `WHSPPOS` writer - Window Horizontal Stop Position
pub type WHSPPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, L1WHPCR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:11 - Window Horizontal Start Position
    #[inline(always)]
    pub fn whstpos(&self) -> WHSTPOS_R {
        WHSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Window Horizontal Stop Position
    #[inline(always)]
    pub fn whsppos(&self) -> WHSPPOS_R {
        WHSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Window Horizontal Start Position
    #[inline(always)]
    #[must_use]
    pub fn whstpos(&mut self) -> WHSTPOS_W<0> {
        WHSTPOS_W::new(self)
    }
    ///Bits 16:27 - Window Horizontal Stop Position
    #[inline(always)]
    #[must_use]
    pub fn whsppos(&mut self) -> WHSPPOS_W<16> {
        WHSPPOS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LTDC Layer Window Horizontal Position Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1whpcr](index.html) module
pub struct L1WHPCR_SPEC;
impl crate::RegisterSpec for L1WHPCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [l1whpcr::R](R) reader structure
impl crate::Readable for L1WHPCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [l1whpcr::W](W) writer structure
impl crate::Writable for L1WHPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets L1WHPCR to value 0
impl crate::Resettable for L1WHPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
