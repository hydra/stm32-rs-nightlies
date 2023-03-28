///Register `ETH_MACA0LR` reader
pub struct R(crate::R<ETH_MACA0LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACA0LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACA0LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACA0LR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETH_MACA0LR` writer
pub struct W(crate::W<ETH_MACA0LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACA0LR_SPEC>;
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
impl From<crate::W<ETH_MACA0LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACA0LR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADDRLO` reader - ADDRLO
pub type ADDRLO_R = crate::FieldReader<u32, u32>;
///Field `ADDRLO` writer - ADDRLO
pub type ADDRLO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACA0LR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - ADDRLO
    #[inline(always)]
    pub fn addrlo(&self) -> ADDRLO_R {
        ADDRLO_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - ADDRLO
    #[inline(always)]
    #[must_use]
    pub fn addrlo(&mut self) -> ADDRLO_W<0> {
        ADDRLO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The MAC Address x Low register holds the lower 32 bits of the 6-byte first MAC address of the station.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_maca0lr](index.html) module
pub struct ETH_MACA0LR_SPEC;
impl crate::RegisterSpec for ETH_MACA0LR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_maca0lr::R](R) reader structure
impl crate::Readable for ETH_MACA0LR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eth_maca0lr::W](W) writer structure
impl crate::Writable for ETH_MACA0LR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ETH_MACA0LR to value 0xffff_ffff
impl crate::Resettable for ETH_MACA0LR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
