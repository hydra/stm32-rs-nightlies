///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ALGODIR` reader - Algorithm direction
pub type ALGODIR_R = crate::BitReader<bool>;
///Field `ALGODIR` writer - Algorithm direction
pub type ALGODIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ALGOMODE` reader - Algorithm mode
pub type ALGOMODE_R = crate::FieldReader<u8, u8>;
///Field `ALGOMODE` writer - Algorithm mode
pub type ALGOMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
///Field `DATATYPE` reader - Data type selection
pub type DATATYPE_R = crate::FieldReader<u8, u8>;
///Field `DATATYPE` writer - Data type selection
pub type DATATYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `KEYSIZE` reader - Key size selection (AES mode only)
pub type KEYSIZE_R = crate::FieldReader<u8, u8>;
///Field `KEYSIZE` writer - Key size selection (AES mode only)
pub type KEYSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `FFLUSH` writer - FIFO flush
pub type FFLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CRYPEN` reader - Cryptographic processor enable
pub type CRYPEN_R = crate::BitReader<bool>;
///Field `CRYPEN` writer - Cryptographic processor enable
pub type CRYPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    ///Bit 2 - Algorithm direction
    #[inline(always)]
    pub fn algodir(&self) -> ALGODIR_R {
        ALGODIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - Algorithm mode
    #[inline(always)]
    pub fn algomode(&self) -> ALGOMODE_R {
        ALGOMODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:7 - Data type selection
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Key size selection (AES mode only)
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 15 - Cryptographic processor enable
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Algorithm direction
    #[inline(always)]
    #[must_use]
    pub fn algodir(&mut self) -> ALGODIR_W<2> {
        ALGODIR_W::new(self)
    }
    ///Bits 3:5 - Algorithm mode
    #[inline(always)]
    #[must_use]
    pub fn algomode(&mut self) -> ALGOMODE_W<3> {
        ALGOMODE_W::new(self)
    }
    ///Bits 6:7 - Data type selection
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DATATYPE_W<6> {
        DATATYPE_W::new(self)
    }
    ///Bits 8:9 - Key size selection (AES mode only)
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<8> {
        KEYSIZE_W::new(self)
    }
    ///Bit 14 - FIFO flush
    #[inline(always)]
    #[must_use]
    pub fn fflush(&mut self) -> FFLUSH_W<14> {
        FFLUSH_W::new(self)
    }
    ///Bit 15 - Cryptographic processor enable
    #[inline(always)]
    #[must_use]
    pub fn crypen(&mut self) -> CRYPEN_W<15> {
        CRYPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
