///Register `MMCRIR` reader
pub struct R(crate::R<MMCRIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCRIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCRIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCRIR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MMCRIR` writer
pub struct W(crate::W<MMCRIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCRIR_SPEC>;
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
impl From<crate::W<MMCRIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCRIR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RFCES` reader - Received frames CRC error status
pub type RFCES_R = crate::BitReader<bool>;
///Field `RFCES` writer - Received frames CRC error status
pub type RFCES_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCRIR_SPEC, bool, O>;
///Field `RFAES` reader - Received frames alignment error status
pub type RFAES_R = crate::BitReader<bool>;
///Field `RFAES` writer - Received frames alignment error status
pub type RFAES_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCRIR_SPEC, bool, O>;
///Field `RGUFS` reader - Received good Unicast frames status
pub type RGUFS_R = crate::BitReader<bool>;
///Field `RGUFS` writer - Received good Unicast frames status
pub type RGUFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCRIR_SPEC, bool, O>;
impl R {
    ///Bit 5 - Received frames CRC error status
    #[inline(always)]
    pub fn rfces(&self) -> RFCES_R {
        RFCES_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Received frames alignment error status
    #[inline(always)]
    pub fn rfaes(&self) -> RFAES_R {
        RFAES_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 17 - Received good Unicast frames status
    #[inline(always)]
    pub fn rgufs(&self) -> RGUFS_R {
        RGUFS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    ///Bit 5 - Received frames CRC error status
    #[inline(always)]
    #[must_use]
    pub fn rfces(&mut self) -> RFCES_W<5> {
        RFCES_W::new(self)
    }
    ///Bit 6 - Received frames alignment error status
    #[inline(always)]
    #[must_use]
    pub fn rfaes(&mut self) -> RFAES_W<6> {
        RFAES_W::new(self)
    }
    ///Bit 17 - Received good Unicast frames status
    #[inline(always)]
    #[must_use]
    pub fn rgufs(&mut self) -> RGUFS_W<17> {
        RGUFS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MMC receive interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mmcrir](index.html) module
pub struct MMCRIR_SPEC;
impl crate::RegisterSpec for MMCRIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mmcrir::R](R) reader structure
impl crate::Readable for MMCRIR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mmcrir::W](W) writer structure
impl crate::Writable for MMCRIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MMCRIR to value 0
impl crate::Resettable for MMCRIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
