///Register `FGCOLR` reader
pub struct R(crate::R<FGCOLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGCOLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGCOLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGCOLR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FGCOLR` writer
pub struct W(crate::W<FGCOLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGCOLR_SPEC>;
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
impl From<crate::W<FGCOLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGCOLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BLUE` reader - Blue Value These bits defines the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only.
pub type BLUE_R = crate::FieldReader<u8, u8>;
///Field `BLUE` writer - Blue Value These bits defines the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only.
pub type BLUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGCOLR_SPEC, u8, u8, 8, O>;
///Field `GREEN` reader - Green Value These bits defines the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only.
pub type GREEN_R = crate::FieldReader<u8, u8>;
///Field `GREEN` writer - Green Value These bits defines the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only.
pub type GREEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGCOLR_SPEC, u8, u8, 8, O>;
///Field `RED` reader - Red Value These bits defines the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
pub type RED_R = crate::FieldReader<u8, u8>;
///Field `RED` writer - Red Value These bits defines the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
pub type RED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGCOLR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Blue Value These bits defines the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only.
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Green Value These bits defines the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only.
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Red Value These bits defines the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Blue Value These bits defines the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only.
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BLUE_W<0> {
        BLUE_W::new(self)
    }
    ///Bits 8:15 - Green Value These bits defines the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only.
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GREEN_W<8> {
        GREEN_W::new(self)
    }
    ///Bits 16:23 - Red Value These bits defines the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only.
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
///DMA2D foreground color register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fgcolr](index.html) module
pub struct FGCOLR_SPEC;
impl crate::RegisterSpec for FGCOLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fgcolr::R](R) reader structure
impl crate::Readable for FGCOLR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fgcolr::W](W) writer structure
impl crate::Writable for FGCOLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FGCOLR to value 0
impl crate::Resettable for FGCOLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
