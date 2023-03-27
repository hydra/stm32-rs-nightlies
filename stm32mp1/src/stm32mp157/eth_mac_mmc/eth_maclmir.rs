///Register `ETH_MACLMIR` reader
pub struct R(crate::R<ETH_MACLMIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACLMIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACLMIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACLMIR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETH_MACLMIR` writer
pub struct W(crate::W<ETH_MACLMIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACLMIR_SPEC>;
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
impl From<crate::W<ETH_MACLMIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACLMIR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSI` reader - LSI
pub type LSI_R = crate::FieldReader<u8, u8>;
///Field `LSI` writer - LSI
pub type LSI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACLMIR_SPEC, u8, u8, 8, O>;
///Field `DRSYNCR` reader - DRSYNCR
pub type DRSYNCR_R = crate::FieldReader<u8, u8>;
///Field `DRSYNCR` writer - DRSYNCR
pub type DRSYNCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACLMIR_SPEC, u8, u8, 3, O>;
///Field `LMPDRI` reader - LMPDRI
pub type LMPDRI_R = crate::FieldReader<u8, u8>;
///Field `LMPDRI` writer - LMPDRI
pub type LMPDRI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACLMIR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - LSI
    #[inline(always)]
    pub fn lsi(&self) -> LSI_R {
        LSI_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - DRSYNCR
    #[inline(always)]
    pub fn drsyncr(&self) -> DRSYNCR_R {
        DRSYNCR_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 24:31 - LMPDRI
    #[inline(always)]
    pub fn lmpdri(&self) -> LMPDRI_R {
        LMPDRI_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - LSI
    #[inline(always)]
    #[must_use]
    pub fn lsi(&mut self) -> LSI_W<0> {
        LSI_W::new(self)
    }
    ///Bits 8:10 - DRSYNCR
    #[inline(always)]
    #[must_use]
    pub fn drsyncr(&mut self) -> DRSYNCR_W<8> {
        DRSYNCR_W::new(self)
    }
    ///Bits 24:31 - LMPDRI
    #[inline(always)]
    #[must_use]
    pub fn lmpdri(&mut self) -> LMPDRI_W<24> {
        LMPDRI_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register contains the periodic intervals for automatic PTP packet generation.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_maclmir](index.html) module
pub struct ETH_MACLMIR_SPEC;
impl crate::RegisterSpec for ETH_MACLMIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_maclmir::R](R) reader structure
impl crate::Readable for ETH_MACLMIR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eth_maclmir::W](W) writer structure
impl crate::Writable for ETH_MACLMIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ETH_MACLMIR to value 0
impl crate::Resettable for ETH_MACLMIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
