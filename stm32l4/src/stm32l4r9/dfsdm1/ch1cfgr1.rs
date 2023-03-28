///Register `CH1CFGR1` reader
pub struct R(crate::R<CH1CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CH1CFGR1` writer
pub struct W(crate::W<CH1CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1CFGR1_SPEC>;
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
impl From<crate::W<CH1CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SITP` reader - SITP
pub type SITP_R = crate::FieldReader<u8, u8>;
///Field `SITP` writer - SITP
pub type SITP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1CFGR1_SPEC, u8, u8, 2, O>;
///Field `SPICKSEL` reader - SPICKSEL
pub type SPICKSEL_R = crate::FieldReader<u8, u8>;
///Field `SPICKSEL` writer - SPICKSEL
pub type SPICKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1CFGR1_SPEC, u8, u8, 2, O>;
///Field `SCDEN` reader - SCDEN
pub type SCDEN_R = crate::BitReader<bool>;
///Field `SCDEN` writer - SCDEN
pub type SCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1CFGR1_SPEC, bool, O>;
///Field `CKABEN` reader - CKABEN
pub type CKABEN_R = crate::BitReader<bool>;
///Field `CKABEN` writer - CKABEN
pub type CKABEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1CFGR1_SPEC, bool, O>;
///Field `CHEN` reader - CHEN
pub type CHEN_R = crate::BitReader<bool>;
///Field `CHEN` writer - CHEN
pub type CHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1CFGR1_SPEC, bool, O>;
///Field `CHINSEL` reader - CHINSEL
pub type CHINSEL_R = crate::BitReader<bool>;
///Field `CHINSEL` writer - CHINSEL
pub type CHINSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1CFGR1_SPEC, bool, O>;
///Field `DATMPX` reader - DATMPX
pub type DATMPX_R = crate::FieldReader<u8, u8>;
///Field `DATMPX` writer - DATMPX
pub type DATMPX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1CFGR1_SPEC, u8, u8, 2, O>;
///Field `DATPACK` reader - DATPACK
pub type DATPACK_R = crate::FieldReader<u8, u8>;
///Field `DATPACK` writer - DATPACK
pub type DATPACK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1CFGR1_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:1 - SITP
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - SPICKSEL
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 5 - SCDEN
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CKABEN
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CHEN
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CHINSEL
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 12:13 - DATMPX
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - DATPACK
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - SITP
    #[inline(always)]
    #[must_use]
    pub fn sitp(&mut self) -> SITP_W<0> {
        SITP_W::new(self)
    }
    ///Bits 2:3 - SPICKSEL
    #[inline(always)]
    #[must_use]
    pub fn spicksel(&mut self) -> SPICKSEL_W<2> {
        SPICKSEL_W::new(self)
    }
    ///Bit 5 - SCDEN
    #[inline(always)]
    #[must_use]
    pub fn scden(&mut self) -> SCDEN_W<5> {
        SCDEN_W::new(self)
    }
    ///Bit 6 - CKABEN
    #[inline(always)]
    #[must_use]
    pub fn ckaben(&mut self) -> CKABEN_W<6> {
        CKABEN_W::new(self)
    }
    ///Bit 7 - CHEN
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<7> {
        CHEN_W::new(self)
    }
    ///Bit 8 - CHINSEL
    #[inline(always)]
    #[must_use]
    pub fn chinsel(&mut self) -> CHINSEL_W<8> {
        CHINSEL_W::new(self)
    }
    ///Bits 12:13 - DATMPX
    #[inline(always)]
    #[must_use]
    pub fn datmpx(&mut self) -> DATMPX_W<12> {
        DATMPX_W::new(self)
    }
    ///Bits 14:15 - DATPACK
    #[inline(always)]
    #[must_use]
    pub fn datpack(&mut self) -> DATPACK_W<14> {
        DATPACK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CH1CFGR1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch1cfgr1](index.html) module
pub struct CH1CFGR1_SPEC;
impl crate::RegisterSpec for CH1CFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ch1cfgr1::R](R) reader structure
impl crate::Readable for CH1CFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ch1cfgr1::W](W) writer structure
impl crate::Writable for CH1CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CH1CFGR1 to value 0
impl crate::Resettable for CH1CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
