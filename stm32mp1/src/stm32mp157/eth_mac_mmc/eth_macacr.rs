///Register `ETH_MACACR` reader
pub struct R(crate::R<ETH_MACACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACACR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETH_MACACR` writer
pub struct W(crate::W<ETH_MACACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACACR_SPEC>;
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
impl From<crate::W<ETH_MACACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACACR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ATSFC` reader - ATSFC
pub type ATSFC_R = crate::BitReader<bool>;
///Field `ATSFC` writer - ATSFC
pub type ATSFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACACR_SPEC, bool, O>;
///Field `ATSEN0` reader - ATSEN0
pub type ATSEN0_R = crate::BitReader<bool>;
///Field `ATSEN0` writer - ATSEN0
pub type ATSEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACACR_SPEC, bool, O>;
///Field `ATSEN1` reader - ATSEN1
pub type ATSEN1_R = crate::BitReader<bool>;
///Field `ATSEN1` writer - ATSEN1
pub type ATSEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACACR_SPEC, bool, O>;
///Field `ATSEN2` reader - ATSEN2
pub type ATSEN2_R = crate::BitReader<bool>;
///Field `ATSEN2` writer - ATSEN2
pub type ATSEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACACR_SPEC, bool, O>;
///Field `ATSEN3` reader - ATSEN3
pub type ATSEN3_R = crate::BitReader<bool>;
///Field `ATSEN3` writer - ATSEN3
pub type ATSEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACACR_SPEC, bool, O>;
impl R {
    ///Bit 0 - ATSFC
    #[inline(always)]
    pub fn atsfc(&self) -> ATSFC_R {
        ATSFC_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - ATSEN0
    #[inline(always)]
    pub fn atsen0(&self) -> ATSEN0_R {
        ATSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ATSEN1
    #[inline(always)]
    pub fn atsen1(&self) -> ATSEN1_R {
        ATSEN1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ATSEN2
    #[inline(always)]
    pub fn atsen2(&self) -> ATSEN2_R {
        ATSEN2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ATSEN3
    #[inline(always)]
    pub fn atsen3(&self) -> ATSEN3_R {
        ATSEN3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ATSFC
    #[inline(always)]
    #[must_use]
    pub fn atsfc(&mut self) -> ATSFC_W<0> {
        ATSFC_W::new(self)
    }
    ///Bit 4 - ATSEN0
    #[inline(always)]
    #[must_use]
    pub fn atsen0(&mut self) -> ATSEN0_W<4> {
        ATSEN0_W::new(self)
    }
    ///Bit 5 - ATSEN1
    #[inline(always)]
    #[must_use]
    pub fn atsen1(&mut self) -> ATSEN1_W<5> {
        ATSEN1_W::new(self)
    }
    ///Bit 6 - ATSEN2
    #[inline(always)]
    #[must_use]
    pub fn atsen2(&mut self) -> ATSEN2_W<6> {
        ATSEN2_W::new(self)
    }
    ///Bit 7 - ATSEN3
    #[inline(always)]
    #[must_use]
    pub fn atsen3(&mut self) -> ATSEN3_W<7> {
        ATSEN3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The Auxiliary Timestamp Control register controls the Auxiliary Timestamp snapshot.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_macacr](index.html) module
pub struct ETH_MACACR_SPEC;
impl crate::RegisterSpec for ETH_MACACR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_macacr::R](R) reader structure
impl crate::Readable for ETH_MACACR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eth_macacr::W](W) writer structure
impl crate::Writable for ETH_MACACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ETH_MACACR to value 0
impl crate::Resettable for ETH_MACACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
