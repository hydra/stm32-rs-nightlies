///Register `CRYP_IMSCR` reader
pub struct R(crate::R<CRYP_IMSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_IMSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_IMSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_IMSCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRYP_IMSCR` writer
pub struct W(crate::W<CRYP_IMSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_IMSCR_SPEC>;
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
impl From<crate::W<CRYP_IMSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_IMSCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INIM` reader - INIM
pub type INIM_R = crate::BitReader<bool>;
///Field `INIM` writer - INIM
pub type INIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IMSCR_SPEC, bool, O>;
///Field `OUTIM` reader - OUTIM
pub type OUTIM_R = crate::BitReader<bool>;
///Field `OUTIM` writer - OUTIM
pub type OUTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IMSCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - INIM
    #[inline(always)]
    pub fn inim(&self) -> INIM_R {
        INIM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - OUTIM
    #[inline(always)]
    pub fn outim(&self) -> OUTIM_R {
        OUTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - INIM
    #[inline(always)]
    #[must_use]
    pub fn inim(&mut self) -> INIM_W<0> {
        INIM_W::new(self)
    }
    ///Bit 1 - OUTIM
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
///The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. When a read operation is performed, this register gives the current value of the mask applied to the relevant interrupt. Writing 1 to the particular bit sets the mask, thus enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cryp_imscr](index.html) module
pub struct CRYP_IMSCR_SPEC;
impl crate::RegisterSpec for CRYP_IMSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cryp_imscr::R](R) reader structure
impl crate::Readable for CRYP_IMSCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cryp_imscr::W](W) writer structure
impl crate::Writable for CRYP_IMSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CRYP_IMSCR to value 0
impl crate::Resettable for CRYP_IMSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
