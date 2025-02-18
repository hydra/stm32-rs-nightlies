///Register `OR3` reader
pub struct R(crate::R<OR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OR3` writer
pub struct W(crate::W<OR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR3_SPEC>;
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
impl From<crate::W<OR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BK2INE` reader - BRK2 BKIN input enable
pub type BK2INE_R = crate::BitReader<bool>;
///Field `BK2INE` writer - BRK2 BKIN input enable
pub type BK2INE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR3_SPEC, bool, O>;
///Field `BK2CMP1E` reader - BRK2 COMP1 enable
pub type BK2CMP1E_R = crate::BitReader<bool>;
///Field `BK2CMP1E` writer - BRK2 COMP1 enable
pub type BK2CMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR3_SPEC, bool, O>;
///Field `BK2CMP2E` reader - BRK2 COMP2 enable
pub type BK2CMP2E_R = crate::BitReader<bool>;
///Field `BK2CMP2E` writer - BRK2 COMP2 enable
pub type BK2CMP2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR3_SPEC, bool, O>;
///Field `BK2DFBK0E` reader - BRK2 DFSDM_BREAK0 enable
pub type BK2DFBK0E_R = crate::BitReader<bool>;
///Field `BK2DFBK0E` writer - BRK2 DFSDM_BREAK0 enable
pub type BK2DFBK0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR3_SPEC, bool, O>;
///Field `BK2INP` reader - BRK2 BKIN input polarity
pub type BK2INP_R = crate::BitReader<bool>;
///Field `BK2INP` writer - BRK2 BKIN input polarity
pub type BK2INP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR3_SPEC, bool, O>;
///Field `BK2CMP1P` reader - BRK2 COMP1 input polarity
pub type BK2CMP1P_R = crate::BitReader<bool>;
///Field `BK2CMP1P` writer - BRK2 COMP1 input polarity
pub type BK2CMP1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR3_SPEC, bool, O>;
///Field `BK2CMP2P` reader - BRK2 COMP2 input polarity
pub type BK2CMP2P_R = crate::BitReader<bool>;
///Field `BK2CMP2P` writer - BRK2 COMP2 input polarity
pub type BK2CMP2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR3_SPEC, bool, O>;
impl R {
    ///Bit 0 - BRK2 BKIN input enable
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BRK2 COMP1 enable
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> BK2CMP1E_R {
        BK2CMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BRK2 COMP2 enable
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> BK2CMP2E_R {
        BK2CMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - BRK2 DFSDM_BREAK0 enable
    #[inline(always)]
    pub fn bk2dfbk0e(&self) -> BK2DFBK0E_R {
        BK2DFBK0E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BRK2 BKIN input polarity
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - BRK2 COMP1 input polarity
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> BK2CMP1P_R {
        BK2CMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - BRK2 COMP2 input polarity
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> BK2CMP2P_R {
        BK2CMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - BRK2 BKIN input enable
    #[inline(always)]
    #[must_use]
    pub fn bk2ine(&mut self) -> BK2INE_W<0> {
        BK2INE_W::new(self)
    }
    ///Bit 1 - BRK2 COMP1 enable
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1e(&mut self) -> BK2CMP1E_W<1> {
        BK2CMP1E_W::new(self)
    }
    ///Bit 2 - BRK2 COMP2 enable
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2e(&mut self) -> BK2CMP2E_W<2> {
        BK2CMP2E_W::new(self)
    }
    ///Bit 8 - BRK2 DFSDM_BREAK0 enable
    #[inline(always)]
    #[must_use]
    pub fn bk2dfbk0e(&mut self) -> BK2DFBK0E_W<8> {
        BK2DFBK0E_W::new(self)
    }
    ///Bit 9 - BRK2 BKIN input polarity
    #[inline(always)]
    #[must_use]
    pub fn bk2inp(&mut self) -> BK2INP_W<9> {
        BK2INP_W::new(self)
    }
    ///Bit 10 - BRK2 COMP1 input polarity
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1p(&mut self) -> BK2CMP1P_W<10> {
        BK2CMP1P_W::new(self)
    }
    ///Bit 11 - BRK2 COMP2 input polarity
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2p(&mut self) -> BK2CMP2P_W<11> {
        BK2CMP2P_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA address for full transfer
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [or3](index.html) module
pub struct OR3_SPEC;
impl crate::RegisterSpec for OR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [or3::R](R) reader structure
impl crate::Readable for OR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [or3::W](W) writer structure
impl crate::Writable for OR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OR3 to value 0x01
impl crate::Resettable for OR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
