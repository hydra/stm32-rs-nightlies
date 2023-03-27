///Register `YBUFCFG` reader
pub struct R(crate::R<YBUFCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<YBUFCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<YBUFCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<YBUFCFG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `YBUFCFG` writer
pub struct W(crate::W<YBUFCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<YBUFCFG_SPEC>;
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
impl From<crate::W<YBUFCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<YBUFCFG_SPEC>) -> Self {
        W(writer)
    }
}
///Field `Y_BASE` reader - X1_BASE
pub type Y_BASE_R = crate::FieldReader<u8, u8>;
///Field `Y_BASE` writer - X1_BASE
pub type Y_BASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, YBUFCFG_SPEC, u8, u8, 8, O>;
///Field `Y_BUF_SIZE` reader - X1_BUF_SIZE
pub type Y_BUF_SIZE_R = crate::FieldReader<u8, u8>;
///Field `Y_BUF_SIZE` writer - X1_BUF_SIZE
pub type Y_BUF_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, YBUFCFG_SPEC, u8, u8, 8, O>;
///Field `EMPTY_WM` reader - EMPTY_WM
pub type EMPTY_WM_R = crate::FieldReader<u8, u8>;
///Field `EMPTY_WM` writer - EMPTY_WM
pub type EMPTY_WM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, YBUFCFG_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:7 - X1_BASE
    #[inline(always)]
    pub fn y_base(&self) -> Y_BASE_R {
        Y_BASE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - X1_BUF_SIZE
    #[inline(always)]
    pub fn y_buf_size(&self) -> Y_BUF_SIZE_R {
        Y_BUF_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 24:25 - EMPTY_WM
    #[inline(always)]
    pub fn empty_wm(&self) -> EMPTY_WM_R {
        EMPTY_WM_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    ///Bits 0:7 - X1_BASE
    #[inline(always)]
    #[must_use]
    pub fn y_base(&mut self) -> Y_BASE_W<0> {
        Y_BASE_W::new(self)
    }
    ///Bits 8:15 - X1_BUF_SIZE
    #[inline(always)]
    #[must_use]
    pub fn y_buf_size(&mut self) -> Y_BUF_SIZE_W<8> {
        Y_BUF_SIZE_W::new(self)
    }
    ///Bits 24:25 - EMPTY_WM
    #[inline(always)]
    #[must_use]
    pub fn empty_wm(&mut self) -> EMPTY_WM_W<24> {
        EMPTY_WM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FMAC Y Buffer Configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ybufcfg](index.html) module
pub struct YBUFCFG_SPEC;
impl crate::RegisterSpec for YBUFCFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [ybufcfg::R](R) reader structure
impl crate::Readable for YBUFCFG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ybufcfg::W](W) writer structure
impl crate::Writable for YBUFCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets YBUFCFG to value 0
impl crate::Resettable for YBUFCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
