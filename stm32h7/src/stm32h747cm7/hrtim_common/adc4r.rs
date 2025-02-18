///Register `ADC4R` reader
pub struct R(crate::R<ADC4R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC4R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC4R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC4R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC4R` writer
pub struct W(crate::W<ADC4R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC4R_SPEC>;
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
impl From<crate::W<ADC4R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC4R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AD2MC1` reader - AD2MC1
pub type AD2MC1_R = crate::BitReader<bool>;
///Field `AD2MC1` writer - AD2MC1
pub type AD2MC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2MC2` reader - AD2MC2
pub type AD2MC2_R = crate::BitReader<bool>;
///Field `AD2MC2` writer - AD2MC2
pub type AD2MC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2MC3` reader - AD2MC3
pub type AD2MC3_R = crate::BitReader<bool>;
///Field `AD2MC3` writer - AD2MC3
pub type AD2MC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2MC4` reader - AD2MC4
pub type AD2MC4_R = crate::BitReader<bool>;
///Field `AD2MC4` writer - AD2MC4
pub type AD2MC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2MPER` reader - AD2MPER
pub type AD2MPER_R = crate::BitReader<bool>;
///Field `AD2MPER` writer - AD2MPER
pub type AD2MPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2EEV6` reader - AD2EEV6
pub type AD2EEV6_R = crate::BitReader<bool>;
///Field `AD2EEV6` writer - AD2EEV6
pub type AD2EEV6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2EEV7` reader - AD2EEV7
pub type AD2EEV7_R = crate::BitReader<bool>;
///Field `AD2EEV7` writer - AD2EEV7
pub type AD2EEV7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2EEV8` reader - AD2EEV8
pub type AD2EEV8_R = crate::BitReader<bool>;
///Field `AD2EEV8` writer - AD2EEV8
pub type AD2EEV8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2EEV9` reader - AD2EEV9
pub type AD2EEV9_R = crate::BitReader<bool>;
///Field `AD2EEV9` writer - AD2EEV9
pub type AD2EEV9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2EEV10` reader - AD2EEV10
pub type AD2EEV10_R = crate::BitReader<bool>;
///Field `AD2EEV10` writer - AD2EEV10
pub type AD2EEV10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TAC2` reader - AD2TAC2
pub type AD2TAC2_R = crate::BitReader<bool>;
///Field `AD2TAC2` writer - AD2TAC2
pub type AD2TAC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TAC3` reader - AD2TAC3
pub type AD2TAC3_R = crate::BitReader<bool>;
///Field `AD2TAC3` writer - AD2TAC3
pub type AD2TAC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TAC4` reader - AD2TAC4
pub type AD2TAC4_R = crate::BitReader<bool>;
///Field `AD2TAC4` writer - AD2TAC4
pub type AD2TAC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TAPER` reader - AD2TAPER
pub type AD2TAPER_R = crate::BitReader<bool>;
///Field `AD2TAPER` writer - AD2TAPER
pub type AD2TAPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TBC2` reader - AD2TBC2
pub type AD2TBC2_R = crate::BitReader<bool>;
///Field `AD2TBC2` writer - AD2TBC2
pub type AD2TBC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TBC3` reader - AD2TBC3
pub type AD2TBC3_R = crate::BitReader<bool>;
///Field `AD2TBC3` writer - AD2TBC3
pub type AD2TBC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TBC4` reader - AD2TBC4
pub type AD2TBC4_R = crate::BitReader<bool>;
///Field `AD2TBC4` writer - AD2TBC4
pub type AD2TBC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TBPER` reader - AD2TBPER
pub type AD2TBPER_R = crate::BitReader<bool>;
///Field `AD2TBPER` writer - AD2TBPER
pub type AD2TBPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TCC2` reader - AD2TCC2
pub type AD2TCC2_R = crate::BitReader<bool>;
///Field `AD2TCC2` writer - AD2TCC2
pub type AD2TCC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TCC3` reader - AD2TCC3
pub type AD2TCC3_R = crate::BitReader<bool>;
///Field `AD2TCC3` writer - AD2TCC3
pub type AD2TCC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TCC4` reader - AD2TCC4
pub type AD2TCC4_R = crate::BitReader<bool>;
///Field `AD2TCC4` writer - AD2TCC4
pub type AD2TCC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TCPER` reader - AD2TCPER
pub type AD2TCPER_R = crate::BitReader<bool>;
///Field `AD2TCPER` writer - AD2TCPER
pub type AD2TCPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TCRST` reader - AD2TCRST
pub type AD2TCRST_R = crate::BitReader<bool>;
///Field `AD2TCRST` writer - AD2TCRST
pub type AD2TCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TDC2` reader - AD2TDC2
pub type AD2TDC2_R = crate::BitReader<bool>;
///Field `AD2TDC2` writer - AD2TDC2
pub type AD2TDC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TDC3` reader - AD2TDC3
pub type AD2TDC3_R = crate::BitReader<bool>;
///Field `AD2TDC3` writer - AD2TDC3
pub type AD2TDC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TDC4` reader - AD2TDC4
pub type AD2TDC4_R = crate::BitReader<bool>;
///Field `AD2TDC4` writer - AD2TDC4
pub type AD2TDC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TDPER` reader - AD2TDPER
pub type AD2TDPER_R = crate::BitReader<bool>;
///Field `AD2TDPER` writer - AD2TDPER
pub type AD2TDPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TDRST` reader - AD2TDRST
pub type AD2TDRST_R = crate::BitReader<bool>;
///Field `AD2TDRST` writer - AD2TDRST
pub type AD2TDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TEC2` reader - AD2TEC2
pub type AD2TEC2_R = crate::BitReader<bool>;
///Field `AD2TEC2` writer - AD2TEC2
pub type AD2TEC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TEC3` reader - AD2TEC3
pub type AD2TEC3_R = crate::BitReader<bool>;
///Field `AD2TEC3` writer - AD2TEC3
pub type AD2TEC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TEC4` reader - AD2TEC4
pub type AD2TEC4_R = crate::BitReader<bool>;
///Field `AD2TEC4` writer - AD2TEC4
pub type AD2TEC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
///Field `AD2TERST` reader - AD2TERST
pub type AD2TERST_R = crate::BitReader<bool>;
///Field `AD2TERST` writer - AD2TERST
pub type AD2TERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, bool, O>;
impl R {
    ///Bit 0 - AD2MC1
    #[inline(always)]
    pub fn ad2mc1(&self) -> AD2MC1_R {
        AD2MC1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AD2MC2
    #[inline(always)]
    pub fn ad2mc2(&self) -> AD2MC2_R {
        AD2MC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AD2MC3
    #[inline(always)]
    pub fn ad2mc3(&self) -> AD2MC3_R {
        AD2MC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AD2MC4
    #[inline(always)]
    pub fn ad2mc4(&self) -> AD2MC4_R {
        AD2MC4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AD2MPER
    #[inline(always)]
    pub fn ad2mper(&self) -> AD2MPER_R {
        AD2MPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AD2EEV6
    #[inline(always)]
    pub fn ad2eev6(&self) -> AD2EEV6_R {
        AD2EEV6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AD2EEV7
    #[inline(always)]
    pub fn ad2eev7(&self) -> AD2EEV7_R {
        AD2EEV7_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AD2EEV8
    #[inline(always)]
    pub fn ad2eev8(&self) -> AD2EEV8_R {
        AD2EEV8_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AD2EEV9
    #[inline(always)]
    pub fn ad2eev9(&self) -> AD2EEV9_R {
        AD2EEV9_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AD2EEV10
    #[inline(always)]
    pub fn ad2eev10(&self) -> AD2EEV10_R {
        AD2EEV10_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - AD2TAC2
    #[inline(always)]
    pub fn ad2tac2(&self) -> AD2TAC2_R {
        AD2TAC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - AD2TAC3
    #[inline(always)]
    pub fn ad2tac3(&self) -> AD2TAC3_R {
        AD2TAC3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - AD2TAC4
    #[inline(always)]
    pub fn ad2tac4(&self) -> AD2TAC4_R {
        AD2TAC4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - AD2TAPER
    #[inline(always)]
    pub fn ad2taper(&self) -> AD2TAPER_R {
        AD2TAPER_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - AD2TBC2
    #[inline(always)]
    pub fn ad2tbc2(&self) -> AD2TBC2_R {
        AD2TBC2_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - AD2TBC3
    #[inline(always)]
    pub fn ad2tbc3(&self) -> AD2TBC3_R {
        AD2TBC3_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - AD2TBC4
    #[inline(always)]
    pub fn ad2tbc4(&self) -> AD2TBC4_R {
        AD2TBC4_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AD2TBPER
    #[inline(always)]
    pub fn ad2tbper(&self) -> AD2TBPER_R {
        AD2TBPER_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - AD2TCC2
    #[inline(always)]
    pub fn ad2tcc2(&self) -> AD2TCC2_R {
        AD2TCC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - AD2TCC3
    #[inline(always)]
    pub fn ad2tcc3(&self) -> AD2TCC3_R {
        AD2TCC3_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - AD2TCC4
    #[inline(always)]
    pub fn ad2tcc4(&self) -> AD2TCC4_R {
        AD2TCC4_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - AD2TCPER
    #[inline(always)]
    pub fn ad2tcper(&self) -> AD2TCPER_R {
        AD2TCPER_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - AD2TCRST
    #[inline(always)]
    pub fn ad2tcrst(&self) -> AD2TCRST_R {
        AD2TCRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - AD2TDC2
    #[inline(always)]
    pub fn ad2tdc2(&self) -> AD2TDC2_R {
        AD2TDC2_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - AD2TDC3
    #[inline(always)]
    pub fn ad2tdc3(&self) -> AD2TDC3_R {
        AD2TDC3_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - AD2TDC4
    #[inline(always)]
    pub fn ad2tdc4(&self) -> AD2TDC4_R {
        AD2TDC4_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - AD2TDPER
    #[inline(always)]
    pub fn ad2tdper(&self) -> AD2TDPER_R {
        AD2TDPER_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - AD2TDRST
    #[inline(always)]
    pub fn ad2tdrst(&self) -> AD2TDRST_R {
        AD2TDRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - AD2TEC2
    #[inline(always)]
    pub fn ad2tec2(&self) -> AD2TEC2_R {
        AD2TEC2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - AD2TEC3
    #[inline(always)]
    pub fn ad2tec3(&self) -> AD2TEC3_R {
        AD2TEC3_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - AD2TEC4
    #[inline(always)]
    pub fn ad2tec4(&self) -> AD2TEC4_R {
        AD2TEC4_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AD2TERST
    #[inline(always)]
    pub fn ad2terst(&self) -> AD2TERST_R {
        AD2TERST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - AD2MC1
    #[inline(always)]
    #[must_use]
    pub fn ad2mc1(&mut self) -> AD2MC1_W<0> {
        AD2MC1_W::new(self)
    }
    ///Bit 1 - AD2MC2
    #[inline(always)]
    #[must_use]
    pub fn ad2mc2(&mut self) -> AD2MC2_W<1> {
        AD2MC2_W::new(self)
    }
    ///Bit 2 - AD2MC3
    #[inline(always)]
    #[must_use]
    pub fn ad2mc3(&mut self) -> AD2MC3_W<2> {
        AD2MC3_W::new(self)
    }
    ///Bit 3 - AD2MC4
    #[inline(always)]
    #[must_use]
    pub fn ad2mc4(&mut self) -> AD2MC4_W<3> {
        AD2MC4_W::new(self)
    }
    ///Bit 4 - AD2MPER
    #[inline(always)]
    #[must_use]
    pub fn ad2mper(&mut self) -> AD2MPER_W<4> {
        AD2MPER_W::new(self)
    }
    ///Bit 5 - AD2EEV6
    #[inline(always)]
    #[must_use]
    pub fn ad2eev6(&mut self) -> AD2EEV6_W<5> {
        AD2EEV6_W::new(self)
    }
    ///Bit 6 - AD2EEV7
    #[inline(always)]
    #[must_use]
    pub fn ad2eev7(&mut self) -> AD2EEV7_W<6> {
        AD2EEV7_W::new(self)
    }
    ///Bit 7 - AD2EEV8
    #[inline(always)]
    #[must_use]
    pub fn ad2eev8(&mut self) -> AD2EEV8_W<7> {
        AD2EEV8_W::new(self)
    }
    ///Bit 8 - AD2EEV9
    #[inline(always)]
    #[must_use]
    pub fn ad2eev9(&mut self) -> AD2EEV9_W<8> {
        AD2EEV9_W::new(self)
    }
    ///Bit 9 - AD2EEV10
    #[inline(always)]
    #[must_use]
    pub fn ad2eev10(&mut self) -> AD2EEV10_W<9> {
        AD2EEV10_W::new(self)
    }
    ///Bit 10 - AD2TAC2
    #[inline(always)]
    #[must_use]
    pub fn ad2tac2(&mut self) -> AD2TAC2_W<10> {
        AD2TAC2_W::new(self)
    }
    ///Bit 11 - AD2TAC3
    #[inline(always)]
    #[must_use]
    pub fn ad2tac3(&mut self) -> AD2TAC3_W<11> {
        AD2TAC3_W::new(self)
    }
    ///Bit 12 - AD2TAC4
    #[inline(always)]
    #[must_use]
    pub fn ad2tac4(&mut self) -> AD2TAC4_W<12> {
        AD2TAC4_W::new(self)
    }
    ///Bit 13 - AD2TAPER
    #[inline(always)]
    #[must_use]
    pub fn ad2taper(&mut self) -> AD2TAPER_W<13> {
        AD2TAPER_W::new(self)
    }
    ///Bit 14 - AD2TBC2
    #[inline(always)]
    #[must_use]
    pub fn ad2tbc2(&mut self) -> AD2TBC2_W<14> {
        AD2TBC2_W::new(self)
    }
    ///Bit 15 - AD2TBC3
    #[inline(always)]
    #[must_use]
    pub fn ad2tbc3(&mut self) -> AD2TBC3_W<15> {
        AD2TBC3_W::new(self)
    }
    ///Bit 16 - AD2TBC4
    #[inline(always)]
    #[must_use]
    pub fn ad2tbc4(&mut self) -> AD2TBC4_W<16> {
        AD2TBC4_W::new(self)
    }
    ///Bit 17 - AD2TBPER
    #[inline(always)]
    #[must_use]
    pub fn ad2tbper(&mut self) -> AD2TBPER_W<17> {
        AD2TBPER_W::new(self)
    }
    ///Bit 18 - AD2TCC2
    #[inline(always)]
    #[must_use]
    pub fn ad2tcc2(&mut self) -> AD2TCC2_W<18> {
        AD2TCC2_W::new(self)
    }
    ///Bit 19 - AD2TCC3
    #[inline(always)]
    #[must_use]
    pub fn ad2tcc3(&mut self) -> AD2TCC3_W<19> {
        AD2TCC3_W::new(self)
    }
    ///Bit 20 - AD2TCC4
    #[inline(always)]
    #[must_use]
    pub fn ad2tcc4(&mut self) -> AD2TCC4_W<20> {
        AD2TCC4_W::new(self)
    }
    ///Bit 21 - AD2TCPER
    #[inline(always)]
    #[must_use]
    pub fn ad2tcper(&mut self) -> AD2TCPER_W<21> {
        AD2TCPER_W::new(self)
    }
    ///Bit 22 - AD2TCRST
    #[inline(always)]
    #[must_use]
    pub fn ad2tcrst(&mut self) -> AD2TCRST_W<22> {
        AD2TCRST_W::new(self)
    }
    ///Bit 23 - AD2TDC2
    #[inline(always)]
    #[must_use]
    pub fn ad2tdc2(&mut self) -> AD2TDC2_W<23> {
        AD2TDC2_W::new(self)
    }
    ///Bit 24 - AD2TDC3
    #[inline(always)]
    #[must_use]
    pub fn ad2tdc3(&mut self) -> AD2TDC3_W<24> {
        AD2TDC3_W::new(self)
    }
    ///Bit 25 - AD2TDC4
    #[inline(always)]
    #[must_use]
    pub fn ad2tdc4(&mut self) -> AD2TDC4_W<25> {
        AD2TDC4_W::new(self)
    }
    ///Bit 26 - AD2TDPER
    #[inline(always)]
    #[must_use]
    pub fn ad2tdper(&mut self) -> AD2TDPER_W<26> {
        AD2TDPER_W::new(self)
    }
    ///Bit 27 - AD2TDRST
    #[inline(always)]
    #[must_use]
    pub fn ad2tdrst(&mut self) -> AD2TDRST_W<27> {
        AD2TDRST_W::new(self)
    }
    ///Bit 28 - AD2TEC2
    #[inline(always)]
    #[must_use]
    pub fn ad2tec2(&mut self) -> AD2TEC2_W<28> {
        AD2TEC2_W::new(self)
    }
    ///Bit 29 - AD2TEC3
    #[inline(always)]
    #[must_use]
    pub fn ad2tec3(&mut self) -> AD2TEC3_W<29> {
        AD2TEC3_W::new(self)
    }
    ///Bit 30 - AD2TEC4
    #[inline(always)]
    #[must_use]
    pub fn ad2tec4(&mut self) -> AD2TEC4_W<30> {
        AD2TEC4_W::new(self)
    }
    ///Bit 31 - AD2TERST
    #[inline(always)]
    #[must_use]
    pub fn ad2terst(&mut self) -> AD2TERST_W<31> {
        AD2TERST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC Trigger 4 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc4r](index.html) module
pub struct ADC4R_SPEC;
impl crate::RegisterSpec for ADC4R_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc4r::R](R) reader structure
impl crate::Readable for ADC4R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc4r::W](W) writer structure
impl crate::Writable for ADC4R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC4R to value 0
impl crate::Resettable for ADC4R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
