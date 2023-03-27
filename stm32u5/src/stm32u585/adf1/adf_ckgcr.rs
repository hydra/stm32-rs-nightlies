///Register `ADF_CKGCR` reader
pub struct R(crate::R<ADF_CKGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADF_CKGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADF_CKGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADF_CKGCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADF_CKGCR` writer
pub struct W(crate::W<ADF_CKGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADF_CKGCR_SPEC>;
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
impl From<crate::W<ADF_CKGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADF_CKGCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CKGDEN` reader - CKGEN dividers enable
pub type CKGDEN_R = crate::BitReader<bool>;
///Field `CKGDEN` writer - CKGEN dividers enable
pub type CKGDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_CKGCR_SPEC, bool, O>;
///Field `CCK0EN` reader - ADF_CCK0 clock enable
pub type CCK0EN_R = crate::BitReader<bool>;
///Field `CCK0EN` writer - ADF_CCK0 clock enable
pub type CCK0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_CKGCR_SPEC, bool, O>;
///Field `CCK1EN` reader - ADF_CCK1 clock enable
pub type CCK1EN_R = crate::BitReader<bool>;
///Field `CCK1EN` writer - ADF_CCK1 clock enable
pub type CCK1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_CKGCR_SPEC, bool, O>;
///Field `CKGMOD` reader - Clock generator mode
pub type CKGMOD_R = crate::BitReader<bool>;
///Field `CKGMOD` writer - Clock generator mode
pub type CKGMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_CKGCR_SPEC, bool, O>;
///Field `CCK0DIR` reader - ADF_CCK0 direction
pub type CCK0DIR_R = crate::BitReader<bool>;
///Field `CCK0DIR` writer - ADF_CCK0 direction
pub type CCK0DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_CKGCR_SPEC, bool, O>;
///Field `CCK1DIR` reader - ADF_CCK1 direction
pub type CCK1DIR_R = crate::BitReader<bool>;
///Field `CCK1DIR` writer - ADF_CCK1 direction
pub type CCK1DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_CKGCR_SPEC, bool, O>;
///Field `TRGSENS` reader - CKGEN trigger sensitivity selection
pub type TRGSENS_R = crate::BitReader<bool>;
///Field `TRGSENS` writer - CKGEN trigger sensitivity selection
pub type TRGSENS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_CKGCR_SPEC, bool, O>;
///Field `TRGSRC` reader - Digital filter trigger signal selection
pub type TRGSRC_R = crate::FieldReader<u8, u8>;
///Field `TRGSRC` writer - Digital filter trigger signal selection
pub type TRGSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_CKGCR_SPEC, u8, u8, 4, O>;
///Field `CCKDIV` reader - Divider to control the ADF_CCK clock
pub type CCKDIV_R = crate::FieldReader<u8, u8>;
///Field `CCKDIV` writer - Divider to control the ADF_CCK clock
pub type CCKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_CKGCR_SPEC, u8, u8, 4, O>;
///Field `PROCDIV` reader - Divider to control the serial interface clock
pub type PROCDIV_R = crate::FieldReader<u8, u8>;
///Field `PROCDIV` writer - Divider to control the serial interface clock
pub type PROCDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_CKGCR_SPEC, u8, u8, 7, O>;
///Field `CKGACTIVE` reader - Clock generator active flag
pub type CKGACTIVE_R = crate::BitReader<bool>;
///Field `CKGACTIVE` writer - Clock generator active flag
pub type CKGACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_CKGCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - CKGEN dividers enable
    #[inline(always)]
    pub fn ckgden(&self) -> CKGDEN_R {
        CKGDEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADF_CCK0 clock enable
    #[inline(always)]
    pub fn cck0en(&self) -> CCK0EN_R {
        CCK0EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADF_CCK1 clock enable
    #[inline(always)]
    pub fn cck1en(&self) -> CCK1EN_R {
        CCK1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Clock generator mode
    #[inline(always)]
    pub fn ckgmod(&self) -> CKGMOD_R {
        CKGMOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADF_CCK0 direction
    #[inline(always)]
    pub fn cck0dir(&self) -> CCK0DIR_R {
        CCK0DIR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ADF_CCK1 direction
    #[inline(always)]
    pub fn cck1dir(&self) -> CCK1DIR_R {
        CCK1DIR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - CKGEN trigger sensitivity selection
    #[inline(always)]
    pub fn trgsens(&self) -> TRGSENS_R {
        TRGSENS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 12:15 - Digital filter trigger signal selection
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Divider to control the ADF_CCK clock
    #[inline(always)]
    pub fn cckdiv(&self) -> CCKDIV_R {
        CCKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:30 - Divider to control the serial interface clock
    #[inline(always)]
    pub fn procdiv(&self) -> PROCDIV_R {
        PROCDIV_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    ///Bit 31 - Clock generator active flag
    #[inline(always)]
    pub fn ckgactive(&self) -> CKGACTIVE_R {
        CKGACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CKGEN dividers enable
    #[inline(always)]
    #[must_use]
    pub fn ckgden(&mut self) -> CKGDEN_W<0> {
        CKGDEN_W::new(self)
    }
    ///Bit 1 - ADF_CCK0 clock enable
    #[inline(always)]
    #[must_use]
    pub fn cck0en(&mut self) -> CCK0EN_W<1> {
        CCK0EN_W::new(self)
    }
    ///Bit 2 - ADF_CCK1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn cck1en(&mut self) -> CCK1EN_W<2> {
        CCK1EN_W::new(self)
    }
    ///Bit 4 - Clock generator mode
    #[inline(always)]
    #[must_use]
    pub fn ckgmod(&mut self) -> CKGMOD_W<4> {
        CKGMOD_W::new(self)
    }
    ///Bit 5 - ADF_CCK0 direction
    #[inline(always)]
    #[must_use]
    pub fn cck0dir(&mut self) -> CCK0DIR_W<5> {
        CCK0DIR_W::new(self)
    }
    ///Bit 6 - ADF_CCK1 direction
    #[inline(always)]
    #[must_use]
    pub fn cck1dir(&mut self) -> CCK1DIR_W<6> {
        CCK1DIR_W::new(self)
    }
    ///Bit 8 - CKGEN trigger sensitivity selection
    #[inline(always)]
    #[must_use]
    pub fn trgsens(&mut self) -> TRGSENS_W<8> {
        TRGSENS_W::new(self)
    }
    ///Bits 12:15 - Digital filter trigger signal selection
    #[inline(always)]
    #[must_use]
    pub fn trgsrc(&mut self) -> TRGSRC_W<12> {
        TRGSRC_W::new(self)
    }
    ///Bits 16:19 - Divider to control the ADF_CCK clock
    #[inline(always)]
    #[must_use]
    pub fn cckdiv(&mut self) -> CCKDIV_W<16> {
        CCKDIV_W::new(self)
    }
    ///Bits 24:30 - Divider to control the serial interface clock
    #[inline(always)]
    #[must_use]
    pub fn procdiv(&mut self) -> PROCDIV_W<24> {
        PROCDIV_W::new(self)
    }
    ///Bit 31 - Clock generator active flag
    #[inline(always)]
    #[must_use]
    pub fn ckgactive(&mut self) -> CKGACTIVE_W<31> {
        CKGACTIVE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADF clock generator control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adf_ckgcr](index.html) module
pub struct ADF_CKGCR_SPEC;
impl crate::RegisterSpec for ADF_CKGCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adf_ckgcr::R](R) reader structure
impl crate::Readable for ADF_CKGCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adf_ckgcr::W](W) writer structure
impl crate::Writable for ADF_CKGCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADF_CKGCR to value 0
impl crate::Resettable for ADF_CKGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
