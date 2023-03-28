///Register `PATT4` reader
pub struct R(crate::R<PATT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PATT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PATT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PATT4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PATT4` writer
pub struct W(crate::W<PATT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PATT4_SPEC>;
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
impl From<crate::W<PATT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PATT4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ATTSETx` reader - ATTSETx
pub type ATTSETX_R = crate::FieldReader<u8, u8>;
///Field `ATTSETx` writer - ATTSETx
pub type ATTSETX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT4_SPEC, u8, u8, 8, O>;
///Field `ATTWAITx` reader - ATTWAITx
pub type ATTWAITX_R = crate::FieldReader<u8, u8>;
///Field `ATTWAITx` writer - ATTWAITx
pub type ATTWAITX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT4_SPEC, u8, u8, 8, O>;
///Field `ATTHOLDx` reader - ATTHOLDx
pub type ATTHOLDX_R = crate::FieldReader<u8, u8>;
///Field `ATTHOLDx` writer - ATTHOLDx
pub type ATTHOLDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT4_SPEC, u8, u8, 8, O>;
///Field `ATTHIZx` reader - ATTHIZx
pub type ATTHIZX_R = crate::FieldReader<u8, u8>;
///Field `ATTHIZx` writer - ATTHIZx
pub type ATTHIZX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT4_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - ATTSETx
    #[inline(always)]
    pub fn attsetx(&self) -> ATTSETX_R {
        ATTSETX_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - ATTWAITx
    #[inline(always)]
    pub fn attwaitx(&self) -> ATTWAITX_R {
        ATTWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - ATTHOLDx
    #[inline(always)]
    pub fn attholdx(&self) -> ATTHOLDX_R {
        ATTHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - ATTHIZx
    #[inline(always)]
    pub fn atthizx(&self) -> ATTHIZX_R {
        ATTHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - ATTSETx
    #[inline(always)]
    #[must_use]
    pub fn attsetx(&mut self) -> ATTSETX_W<0> {
        ATTSETX_W::new(self)
    }
    ///Bits 8:15 - ATTWAITx
    #[inline(always)]
    #[must_use]
    pub fn attwaitx(&mut self) -> ATTWAITX_W<8> {
        ATTWAITX_W::new(self)
    }
    ///Bits 16:23 - ATTHOLDx
    #[inline(always)]
    #[must_use]
    pub fn attholdx(&mut self) -> ATTHOLDX_W<16> {
        ATTHOLDX_W::new(self)
    }
    ///Bits 24:31 - ATTHIZx
    #[inline(always)]
    #[must_use]
    pub fn atthizx(&mut self) -> ATTHIZX_W<24> {
        ATTHIZX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Attribute memory space timing register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [patt4](index.html) module
pub struct PATT4_SPEC;
impl crate::RegisterSpec for PATT4_SPEC {
    type Ux = u32;
}
///`read()` method returns [patt4::R](R) reader structure
impl crate::Readable for PATT4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [patt4::W](W) writer structure
impl crate::Writable for PATT4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PATT4 to value 0xfcfc_fcfc
impl crate::Resettable for PATT4_SPEC {
    const RESET_VALUE: Self::Ux = 0xfcfc_fcfc;
}
