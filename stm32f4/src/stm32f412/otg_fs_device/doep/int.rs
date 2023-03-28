///Register `INT` reader
pub struct R(crate::R<INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `INT` writer
pub struct W(crate::W<INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_SPEC>;
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
impl From<crate::W<INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `XFRC` reader - XFRC
pub type XFRC_R = crate::BitReader<bool>;
///Field `XFRC` writer - XFRC
pub type XFRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, bool, O>;
///Field `EPDISD` reader - EPDISD
pub type EPDISD_R = crate::BitReader<bool>;
///Field `EPDISD` writer - EPDISD
pub type EPDISD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, bool, O>;
///Field `STUP` reader - STUP
pub type STUP_R = crate::BitReader<bool>;
///Field `STUP` writer - STUP
pub type STUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, bool, O>;
///Field `OTEPDIS` reader - OTEPDIS
pub type OTEPDIS_R = crate::BitReader<bool>;
///Field `OTEPDIS` writer - OTEPDIS
pub type OTEPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, bool, O>;
///Field `B2BSTUP` reader - B2BSTUP
pub type B2BSTUP_R = crate::BitReader<bool>;
///Field `B2BSTUP` writer - B2BSTUP
pub type B2BSTUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SPEC, bool, O>;
impl R {
    ///Bit 0 - XFRC
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EPDISD
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - STUP
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OTEPDIS
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - B2BSTUP
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - XFRC
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<0> {
        XFRC_W::new(self)
    }
    ///Bit 1 - EPDISD
    #[inline(always)]
    #[must_use]
    pub fn epdisd(&mut self) -> EPDISD_W<1> {
        EPDISD_W::new(self)
    }
    ///Bit 3 - STUP
    #[inline(always)]
    #[must_use]
    pub fn stup(&mut self) -> STUP_W<3> {
        STUP_W::new(self)
    }
    ///Bit 4 - OTEPDIS
    #[inline(always)]
    #[must_use]
    pub fn otepdis(&mut self) -> OTEPDIS_W<4> {
        OTEPDIS_W::new(self)
    }
    ///Bit 6 - B2BSTUP
    #[inline(always)]
    #[must_use]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<6> {
        B2BSTUP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///device endpoint-1 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [int](index.html) module
pub struct INT_SPEC;
impl crate::RegisterSpec for INT_SPEC {
    type Ux = u32;
}
///`read()` method returns [int::R](R) reader structure
impl crate::Readable for INT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [int::W](W) writer structure
impl crate::Writable for INT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets INT to value 0x80
impl crate::Resettable for INT_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
