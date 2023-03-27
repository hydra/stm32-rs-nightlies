///Register `MMCTIR` reader
pub struct R(crate::R<MMCTIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCTIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCTIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCTIR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MMCTIR` writer
pub struct W(crate::W<MMCTIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCTIR_SPEC>;
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
impl From<crate::W<MMCTIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCTIR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TGFSCS` reader - Transmitted good frames single collision status
pub type TGFSCS_R = crate::BitReader<bool>;
///Field `TGFSCS` writer - Transmitted good frames single collision status
pub type TGFSCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCTIR_SPEC, bool, O>;
///Field `TGFMSCS` reader - Transmitted good frames more than single collision status
pub type TGFMSCS_R = crate::BitReader<bool>;
///Field `TGFMSCS` writer - Transmitted good frames more than single collision status
pub type TGFMSCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCTIR_SPEC, bool, O>;
///Field `TGFS` reader - Transmitted good frames status
pub type TGFS_R = crate::BitReader<bool>;
///Field `TGFS` writer - Transmitted good frames status
pub type TGFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCTIR_SPEC, bool, O>;
impl R {
    ///Bit 14 - Transmitted good frames single collision status
    #[inline(always)]
    pub fn tgfscs(&self) -> TGFSCS_R {
        TGFSCS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Transmitted good frames more than single collision status
    #[inline(always)]
    pub fn tgfmscs(&self) -> TGFMSCS_R {
        TGFMSCS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 21 - Transmitted good frames status
    #[inline(always)]
    pub fn tgfs(&self) -> TGFS_R {
        TGFS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 14 - Transmitted good frames single collision status
    #[inline(always)]
    #[must_use]
    pub fn tgfscs(&mut self) -> TGFSCS_W<14> {
        TGFSCS_W::new(self)
    }
    ///Bit 15 - Transmitted good frames more than single collision status
    #[inline(always)]
    #[must_use]
    pub fn tgfmscs(&mut self) -> TGFMSCS_W<15> {
        TGFMSCS_W::new(self)
    }
    ///Bit 21 - Transmitted good frames status
    #[inline(always)]
    #[must_use]
    pub fn tgfs(&mut self) -> TGFS_W<21> {
        TGFS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MMC transmit interrupt register (ETH_MMCTIR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mmctir](index.html) module
pub struct MMCTIR_SPEC;
impl crate::RegisterSpec for MMCTIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mmctir::R](R) reader structure
impl crate::Readable for MMCTIR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mmctir::W](W) writer structure
impl crate::Writable for MMCTIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MMCTIR to value 0
impl crate::Resettable for MMCTIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
