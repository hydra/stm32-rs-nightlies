///Register `DCFG` reader
pub struct R(crate::R<DCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCFG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DCFG` writer
pub struct W(crate::W<DCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCFG_SPEC>;
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
impl From<crate::W<DCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCFG_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DSPD` reader - Device speed
pub type DSPD_R = crate::FieldReader<u8, u8>;
///Field `DSPD` writer - Device speed
pub type DSPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFG_SPEC, u8, u8, 2, O>;
///Field `NZLSOHSK` reader - Non-zero-length status OUT handshake
pub type NZLSOHSK_R = crate::BitReader<bool>;
///Field `NZLSOHSK` writer - Non-zero-length status OUT handshake
pub type NZLSOHSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCFG_SPEC, bool, O>;
///Field `DAD` reader - Device address
pub type DAD_R = crate::FieldReader<u8, u8>;
///Field `DAD` writer - Device address
pub type DAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFG_SPEC, u8, u8, 7, O>;
///Field `PFIVL` reader - Periodic frame interval
pub type PFIVL_R = crate::FieldReader<u8, u8>;
///Field `PFIVL` writer - Periodic frame interval
pub type PFIVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFG_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:1 - Device speed
    #[inline(always)]
    pub fn dspd(&self) -> DSPD_R {
        DSPD_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Non-zero-length status OUT handshake
    #[inline(always)]
    pub fn nzlsohsk(&self) -> NZLSOHSK_R {
        NZLSOHSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:10 - Device address
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    ///Bits 11:12 - Periodic frame interval
    #[inline(always)]
    pub fn pfivl(&self) -> PFIVL_R {
        PFIVL_R::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - Device speed
    #[inline(always)]
    #[must_use]
    pub fn dspd(&mut self) -> DSPD_W<0> {
        DSPD_W::new(self)
    }
    ///Bit 2 - Non-zero-length status OUT handshake
    #[inline(always)]
    #[must_use]
    pub fn nzlsohsk(&mut self) -> NZLSOHSK_W<2> {
        NZLSOHSK_W::new(self)
    }
    ///Bits 4:10 - Device address
    #[inline(always)]
    #[must_use]
    pub fn dad(&mut self) -> DAD_W<4> {
        DAD_W::new(self)
    }
    ///Bits 11:12 - Periodic frame interval
    #[inline(always)]
    #[must_use]
    pub fn pfivl(&mut self) -> PFIVL_W<11> {
        PFIVL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_FS device configuration register (OTG_FS_DCFG)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcfg](index.html) module
pub struct DCFG_SPEC;
impl crate::RegisterSpec for DCFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [dcfg::R](R) reader structure
impl crate::Readable for DCFG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dcfg::W](W) writer structure
impl crate::Writable for DCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DCFG to value 0x0220_0000
impl crate::Resettable for DCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0220_0000;
}
