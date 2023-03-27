///Register `BGCOLR` reader
pub struct R(crate::R<BGCOLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGCOLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGCOLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGCOLR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BGCOLR` writer
pub struct W(crate::W<BGCOLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGCOLR_SPEC>;
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
impl From<crate::W<BGCOLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGCOLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BLUE` reader - Blue Value
pub type BLUE_R = crate::FieldReader<u8, u8>;
///Field `BLUE` writer - Blue Value
pub type BLUE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BGCOLR_SPEC, u8, u8, 8, O>;
///Field `GREEN` reader - Green Value
pub type GREEN_R = crate::FieldReader<u8, u8>;
///Field `GREEN` writer - Green Value
pub type GREEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BGCOLR_SPEC, u8, u8, 8, O>;
///Field `RED` reader - Red Value
pub type RED_R = crate::FieldReader<u8, u8>;
///Field `RED` writer - Red Value
pub type RED_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BGCOLR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Blue Value
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Green Value
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Red Value
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Blue Value
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BLUE_W<0> {
        BLUE_W::new(self)
    }
    ///Bits 8:15 - Green Value
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GREEN_W<8> {
        GREEN_W::new(self)
    }
    ///Bits 16:23 - Red Value
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<16> {
        RED_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///background color register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bgcolr](index.html) module
pub struct BGCOLR_SPEC;
impl crate::RegisterSpec for BGCOLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bgcolr::R](R) reader structure
impl crate::Readable for BGCOLR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bgcolr::W](W) writer structure
impl crate::Writable for BGCOLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BGCOLR to value 0
impl crate::Resettable for BGCOLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
