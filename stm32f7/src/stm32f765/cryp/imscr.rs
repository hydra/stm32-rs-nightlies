///Register `IMSCR` reader
pub struct R(crate::R<IMSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMSCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IMSCR` writer
pub struct W(crate::W<IMSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMSCR_SPEC>;
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
impl From<crate::W<IMSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMSCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INIM` reader - Input FIFO service interrupt mask
pub type INIM_R = crate::BitReader<bool>;
///Field `INIM` writer - Input FIFO service interrupt mask
pub type INIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMSCR_SPEC, bool, O>;
///Field `OUTIM` reader - Output FIFO service interrupt mask
pub type OUTIM_R = crate::BitReader<bool>;
///Field `OUTIM` writer - Output FIFO service interrupt mask
pub type OUTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMSCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Input FIFO service interrupt mask
    #[inline(always)]
    pub fn inim(&self) -> INIM_R {
        INIM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Output FIFO service interrupt mask
    #[inline(always)]
    pub fn outim(&self) -> OUTIM_R {
        OUTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Input FIFO service interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn inim(&mut self) -> INIM_W<0> {
        INIM_W::new(self)
    }
    ///Bit 1 - Output FIFO service interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn outim(&mut self) -> OUTIM_W<1> {
        OUTIM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt mask set/clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [imscr](index.html) module
pub struct IMSCR_SPEC;
impl crate::RegisterSpec for IMSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [imscr::R](R) reader structure
impl crate::Readable for IMSCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [imscr::W](W) writer structure
impl crate::Writable for IMSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IMSCR to value 0
impl crate::Resettable for IMSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
