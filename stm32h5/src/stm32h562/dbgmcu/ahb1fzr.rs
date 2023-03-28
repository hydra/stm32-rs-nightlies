///Register `AHB1FZR` reader
pub struct R(crate::R<AHB1FZR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1FZR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1FZR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1FZR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB1FZR` writer
pub struct W(crate::W<AHB1FZR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1FZR_SPEC>;
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
impl From<crate::W<AHB1FZR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1FZR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DBG_GPDMA1_0_STOP` reader - GPDMA1 channel 0 stop in debug
pub type DBG_GPDMA1_0_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_0_STOP` writer - GPDMA1 channel 0 stop in debug
pub type DBG_GPDMA1_0_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_1_STOP` reader - GPDMA1 channel 1 stop in debug
pub type DBG_GPDMA1_1_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_1_STOP` writer - GPDMA1 channel 1 stop in debug
pub type DBG_GPDMA1_1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_2_STOP` reader - GPDMA1 channel 2 stop in debug
pub type DBG_GPDMA1_2_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_2_STOP` writer - GPDMA1 channel 2 stop in debug
pub type DBG_GPDMA1_2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_3_STOP` reader - GPDMA1 channel 3 stop in debug
pub type DBG_GPDMA1_3_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_3_STOP` writer - GPDMA1 channel 3 stop in debug
pub type DBG_GPDMA1_3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_4_STOP` reader - GPDMA1 channel 4 stop in debug
pub type DBG_GPDMA1_4_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_4_STOP` writer - GPDMA1 channel 4 stop in debug
pub type DBG_GPDMA1_4_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_5_STOP` reader - GPDMA1 channel 5 stop in debug
pub type DBG_GPDMA1_5_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_5_STOP` writer - GPDMA1 channel 5 stop in debug
pub type DBG_GPDMA1_5_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_6_STOP` reader - GPDMA1 channel 6 stop in debug
pub type DBG_GPDMA1_6_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_6_STOP` writer - GPDMA1 channel 6 stop in debug
pub type DBG_GPDMA1_6_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_7_STOP` reader - GPDMA1 channel 7 stop in debug
pub type DBG_GPDMA1_7_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_7_STOP` writer - GPDMA1 channel 7 stop in debug
pub type DBG_GPDMA1_7_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_8_STOP` reader - GPDMA1 channel 8 stop in debug
pub type DBG_GPDMA1_8_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_8_STOP` writer - GPDMA1 channel 8 stop in debug
pub type DBG_GPDMA1_8_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_9_STOP` reader - GPDMA1 channel 9 stop in debug
pub type DBG_GPDMA1_9_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_9_STOP` writer - GPDMA1 channel 9 stop in debug
pub type DBG_GPDMA1_9_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_10_STOP` reader - GPDMA1 channel 10 stop in debug
pub type DBG_GPDMA1_10_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_10_STOP` writer - GPDMA1 channel 10 stop in debug
pub type DBG_GPDMA1_10_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_11_STOP` reader - GPDMA1 channel 11 stop in debug
pub type DBG_GPDMA1_11_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_11_STOP` writer - GPDMA1 channel 11 stop in debug
pub type DBG_GPDMA1_11_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_12_STOP` reader - GPDMA1 channel 12 stop in debug
pub type DBG_GPDMA1_12_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_12_STOP` writer - GPDMA1 channel 12 stop in debug
pub type DBG_GPDMA1_12_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_13_STOP` reader - GPDMA1 channel 13 stop in debug
pub type DBG_GPDMA1_13_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_13_STOP` writer - GPDMA1 channel 13 stop in debug
pub type DBG_GPDMA1_13_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_14_STOP` reader - GPDMA1 channel 14 stop in debug
pub type DBG_GPDMA1_14_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_14_STOP` writer - GPDMA1 channel 14 stop in debug
pub type DBG_GPDMA1_14_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_15_STOP` reader - GPDMA1 channel 15 stop in debug
pub type DBG_GPDMA1_15_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_15_STOP` writer - GPDMA1 channel 15 stop in debug
pub type DBG_GPDMA1_15_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_0_STOP` reader - GPDMA2 channel 0 stop in debug
pub type DBG_GPDMA2_0_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_0_STOP` writer - GPDMA2 channel 0 stop in debug
pub type DBG_GPDMA2_0_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_1_STOP` reader - GPDMA2 channel 1 stop in debug
pub type DBG_GPDMA2_1_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_1_STOP` writer - GPDMA2 channel 1 stop in debug
pub type DBG_GPDMA2_1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_2_STOP` reader - GPDMA2 channel 2 stop in debug
pub type DBG_GPDMA2_2_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_2_STOP` writer - GPDMA2 channel 2 stop in debug
pub type DBG_GPDMA2_2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_3_STOP` reader - GPDMA2 channel 3 stop in debug
pub type DBG_GPDMA2_3_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_3_STOP` writer - GPDMA2 channel 3 stop in debug
pub type DBG_GPDMA2_3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_4_STOP` reader - GPDMA2 channel 4 stop in debug
pub type DBG_GPDMA2_4_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_4_STOP` writer - GPDMA2 channel 4 stop in debug
pub type DBG_GPDMA2_4_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_5_STOP` reader - GPDMA2 channel 5 stop in debug
pub type DBG_GPDMA2_5_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_5_STOP` writer - GPDMA2 channel 5 stop in debug
pub type DBG_GPDMA2_5_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_6_STOP` reader - GPDMA2 channel 6 stop in debug
pub type DBG_GPDMA2_6_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_6_STOP` writer - GPDMA2 channel 6 stop in debug
pub type DBG_GPDMA2_6_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_7_STOP` reader - GPDMA2 channel 7 stop in debug
pub type DBG_GPDMA2_7_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_7_STOP` writer - GPDMA2 channel 7 stop in debug
pub type DBG_GPDMA2_7_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_8_STOP` reader - GPDMA2 channel 8 stop in debug
pub type DBG_GPDMA2_8_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_8_STOP` writer - GPDMA2 channel 8 stop in debug
pub type DBG_GPDMA2_8_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_9_STOP` reader - GPDMA2 channel 9 stop in debug
pub type DBG_GPDMA2_9_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_9_STOP` writer - GPDMA2 channel 9 stop in debug
pub type DBG_GPDMA2_9_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_10_STOP` reader - GPDMA2 channel 10 stop in debug
pub type DBG_GPDMA2_10_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_10_STOP` writer - GPDMA2 channel 10 stop in debug
pub type DBG_GPDMA2_10_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_11_STOP` reader - GPDMA2 channel 11 stop in debug
pub type DBG_GPDMA2_11_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_11_STOP` writer - GPDMA2 channel 11 stop in debug
pub type DBG_GPDMA2_11_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_12_STOP` reader - GPDMA2 channel 12 stop in debug
pub type DBG_GPDMA2_12_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_12_STOP` writer - GPDMA2 channel 12 stop in debug
pub type DBG_GPDMA2_12_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_13_STOP` reader - GPDMA2 channel 13 stop in debug
pub type DBG_GPDMA2_13_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_13_STOP` writer - GPDMA2 channel 13 stop in debug
pub type DBG_GPDMA2_13_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_14_STOP` reader - GPDMA2 channel 14 stop in debug
pub type DBG_GPDMA2_14_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_14_STOP` writer - GPDMA2 channel 14 stop in debug
pub type DBG_GPDMA2_14_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_15_STOP` reader - GPDMA2 channel 15 stop in debug
pub type DBG_GPDMA2_15_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_15_STOP` writer - GPDMA2 channel 15 stop in debug
pub type DBG_GPDMA2_15_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
impl R {
    ///Bit 0 - GPDMA1 channel 0 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_0_stop(&self) -> DBG_GPDMA1_0_STOP_R {
        DBG_GPDMA1_0_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPDMA1 channel 1 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_1_stop(&self) -> DBG_GPDMA1_1_STOP_R {
        DBG_GPDMA1_1_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPDMA1 channel 2 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_2_stop(&self) -> DBG_GPDMA1_2_STOP_R {
        DBG_GPDMA1_2_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPDMA1 channel 3 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_3_stop(&self) -> DBG_GPDMA1_3_STOP_R {
        DBG_GPDMA1_3_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPDMA1 channel 4 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_4_stop(&self) -> DBG_GPDMA1_4_STOP_R {
        DBG_GPDMA1_4_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPDMA1 channel 5 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_5_stop(&self) -> DBG_GPDMA1_5_STOP_R {
        DBG_GPDMA1_5_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPDMA1 channel 6 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_6_stop(&self) -> DBG_GPDMA1_6_STOP_R {
        DBG_GPDMA1_6_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPDMA1 channel 7 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_7_stop(&self) -> DBG_GPDMA1_7_STOP_R {
        DBG_GPDMA1_7_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPDMA1 channel 8 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_8_stop(&self) -> DBG_GPDMA1_8_STOP_R {
        DBG_GPDMA1_8_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPDMA1 channel 9 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_9_stop(&self) -> DBG_GPDMA1_9_STOP_R {
        DBG_GPDMA1_9_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GPDMA1 channel 10 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_10_stop(&self) -> DBG_GPDMA1_10_STOP_R {
        DBG_GPDMA1_10_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GPDMA1 channel 11 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_11_stop(&self) -> DBG_GPDMA1_11_STOP_R {
        DBG_GPDMA1_11_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GPDMA1 channel 12 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_12_stop(&self) -> DBG_GPDMA1_12_STOP_R {
        DBG_GPDMA1_12_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GPDMA1 channel 13 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_13_stop(&self) -> DBG_GPDMA1_13_STOP_R {
        DBG_GPDMA1_13_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GPDMA1 channel 14 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_14_stop(&self) -> DBG_GPDMA1_14_STOP_R {
        DBG_GPDMA1_14_STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GPDMA1 channel 15 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_15_stop(&self) -> DBG_GPDMA1_15_STOP_R {
        DBG_GPDMA1_15_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - GPDMA2 channel 0 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_0_stop(&self) -> DBG_GPDMA2_0_STOP_R {
        DBG_GPDMA2_0_STOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - GPDMA2 channel 1 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_1_stop(&self) -> DBG_GPDMA2_1_STOP_R {
        DBG_GPDMA2_1_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - GPDMA2 channel 2 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_2_stop(&self) -> DBG_GPDMA2_2_STOP_R {
        DBG_GPDMA2_2_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - GPDMA2 channel 3 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_3_stop(&self) -> DBG_GPDMA2_3_STOP_R {
        DBG_GPDMA2_3_STOP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - GPDMA2 channel 4 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_4_stop(&self) -> DBG_GPDMA2_4_STOP_R {
        DBG_GPDMA2_4_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - GPDMA2 channel 5 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_5_stop(&self) -> DBG_GPDMA2_5_STOP_R {
        DBG_GPDMA2_5_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - GPDMA2 channel 6 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_6_stop(&self) -> DBG_GPDMA2_6_STOP_R {
        DBG_GPDMA2_6_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - GPDMA2 channel 7 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_7_stop(&self) -> DBG_GPDMA2_7_STOP_R {
        DBG_GPDMA2_7_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - GPDMA2 channel 8 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_8_stop(&self) -> DBG_GPDMA2_8_STOP_R {
        DBG_GPDMA2_8_STOP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - GPDMA2 channel 9 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_9_stop(&self) -> DBG_GPDMA2_9_STOP_R {
        DBG_GPDMA2_9_STOP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - GPDMA2 channel 10 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_10_stop(&self) -> DBG_GPDMA2_10_STOP_R {
        DBG_GPDMA2_10_STOP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - GPDMA2 channel 11 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_11_stop(&self) -> DBG_GPDMA2_11_STOP_R {
        DBG_GPDMA2_11_STOP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - GPDMA2 channel 12 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_12_stop(&self) -> DBG_GPDMA2_12_STOP_R {
        DBG_GPDMA2_12_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - GPDMA2 channel 13 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_13_stop(&self) -> DBG_GPDMA2_13_STOP_R {
        DBG_GPDMA2_13_STOP_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - GPDMA2 channel 14 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_14_stop(&self) -> DBG_GPDMA2_14_STOP_R {
        DBG_GPDMA2_14_STOP_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - GPDMA2 channel 15 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_15_stop(&self) -> DBG_GPDMA2_15_STOP_R {
        DBG_GPDMA2_15_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPDMA1 channel 0 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_0_stop(&mut self) -> DBG_GPDMA1_0_STOP_W<0> {
        DBG_GPDMA1_0_STOP_W::new(self)
    }
    ///Bit 1 - GPDMA1 channel 1 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_1_stop(&mut self) -> DBG_GPDMA1_1_STOP_W<1> {
        DBG_GPDMA1_1_STOP_W::new(self)
    }
    ///Bit 2 - GPDMA1 channel 2 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_2_stop(&mut self) -> DBG_GPDMA1_2_STOP_W<2> {
        DBG_GPDMA1_2_STOP_W::new(self)
    }
    ///Bit 3 - GPDMA1 channel 3 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_3_stop(&mut self) -> DBG_GPDMA1_3_STOP_W<3> {
        DBG_GPDMA1_3_STOP_W::new(self)
    }
    ///Bit 4 - GPDMA1 channel 4 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_4_stop(&mut self) -> DBG_GPDMA1_4_STOP_W<4> {
        DBG_GPDMA1_4_STOP_W::new(self)
    }
    ///Bit 5 - GPDMA1 channel 5 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_5_stop(&mut self) -> DBG_GPDMA1_5_STOP_W<5> {
        DBG_GPDMA1_5_STOP_W::new(self)
    }
    ///Bit 6 - GPDMA1 channel 6 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_6_stop(&mut self) -> DBG_GPDMA1_6_STOP_W<6> {
        DBG_GPDMA1_6_STOP_W::new(self)
    }
    ///Bit 7 - GPDMA1 channel 7 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_7_stop(&mut self) -> DBG_GPDMA1_7_STOP_W<7> {
        DBG_GPDMA1_7_STOP_W::new(self)
    }
    ///Bit 8 - GPDMA1 channel 8 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_8_stop(&mut self) -> DBG_GPDMA1_8_STOP_W<8> {
        DBG_GPDMA1_8_STOP_W::new(self)
    }
    ///Bit 9 - GPDMA1 channel 9 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_9_stop(&mut self) -> DBG_GPDMA1_9_STOP_W<9> {
        DBG_GPDMA1_9_STOP_W::new(self)
    }
    ///Bit 10 - GPDMA1 channel 10 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_10_stop(&mut self) -> DBG_GPDMA1_10_STOP_W<10> {
        DBG_GPDMA1_10_STOP_W::new(self)
    }
    ///Bit 11 - GPDMA1 channel 11 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_11_stop(&mut self) -> DBG_GPDMA1_11_STOP_W<11> {
        DBG_GPDMA1_11_STOP_W::new(self)
    }
    ///Bit 12 - GPDMA1 channel 12 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_12_stop(&mut self) -> DBG_GPDMA1_12_STOP_W<12> {
        DBG_GPDMA1_12_STOP_W::new(self)
    }
    ///Bit 13 - GPDMA1 channel 13 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_13_stop(&mut self) -> DBG_GPDMA1_13_STOP_W<13> {
        DBG_GPDMA1_13_STOP_W::new(self)
    }
    ///Bit 14 - GPDMA1 channel 14 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_14_stop(&mut self) -> DBG_GPDMA1_14_STOP_W<14> {
        DBG_GPDMA1_14_STOP_W::new(self)
    }
    ///Bit 15 - GPDMA1 channel 15 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_15_stop(&mut self) -> DBG_GPDMA1_15_STOP_W<15> {
        DBG_GPDMA1_15_STOP_W::new(self)
    }
    ///Bit 16 - GPDMA2 channel 0 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_0_stop(&mut self) -> DBG_GPDMA2_0_STOP_W<16> {
        DBG_GPDMA2_0_STOP_W::new(self)
    }
    ///Bit 17 - GPDMA2 channel 1 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_1_stop(&mut self) -> DBG_GPDMA2_1_STOP_W<17> {
        DBG_GPDMA2_1_STOP_W::new(self)
    }
    ///Bit 18 - GPDMA2 channel 2 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_2_stop(&mut self) -> DBG_GPDMA2_2_STOP_W<18> {
        DBG_GPDMA2_2_STOP_W::new(self)
    }
    ///Bit 19 - GPDMA2 channel 3 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_3_stop(&mut self) -> DBG_GPDMA2_3_STOP_W<19> {
        DBG_GPDMA2_3_STOP_W::new(self)
    }
    ///Bit 20 - GPDMA2 channel 4 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_4_stop(&mut self) -> DBG_GPDMA2_4_STOP_W<20> {
        DBG_GPDMA2_4_STOP_W::new(self)
    }
    ///Bit 21 - GPDMA2 channel 5 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_5_stop(&mut self) -> DBG_GPDMA2_5_STOP_W<21> {
        DBG_GPDMA2_5_STOP_W::new(self)
    }
    ///Bit 22 - GPDMA2 channel 6 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_6_stop(&mut self) -> DBG_GPDMA2_6_STOP_W<22> {
        DBG_GPDMA2_6_STOP_W::new(self)
    }
    ///Bit 23 - GPDMA2 channel 7 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_7_stop(&mut self) -> DBG_GPDMA2_7_STOP_W<23> {
        DBG_GPDMA2_7_STOP_W::new(self)
    }
    ///Bit 24 - GPDMA2 channel 8 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_8_stop(&mut self) -> DBG_GPDMA2_8_STOP_W<24> {
        DBG_GPDMA2_8_STOP_W::new(self)
    }
    ///Bit 25 - GPDMA2 channel 9 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_9_stop(&mut self) -> DBG_GPDMA2_9_STOP_W<25> {
        DBG_GPDMA2_9_STOP_W::new(self)
    }
    ///Bit 26 - GPDMA2 channel 10 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_10_stop(&mut self) -> DBG_GPDMA2_10_STOP_W<26> {
        DBG_GPDMA2_10_STOP_W::new(self)
    }
    ///Bit 27 - GPDMA2 channel 11 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_11_stop(&mut self) -> DBG_GPDMA2_11_STOP_W<27> {
        DBG_GPDMA2_11_STOP_W::new(self)
    }
    ///Bit 28 - GPDMA2 channel 12 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_12_stop(&mut self) -> DBG_GPDMA2_12_STOP_W<28> {
        DBG_GPDMA2_12_STOP_W::new(self)
    }
    ///Bit 29 - GPDMA2 channel 13 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_13_stop(&mut self) -> DBG_GPDMA2_13_STOP_W<29> {
        DBG_GPDMA2_13_STOP_W::new(self)
    }
    ///Bit 30 - GPDMA2 channel 14 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_14_stop(&mut self) -> DBG_GPDMA2_14_STOP_W<30> {
        DBG_GPDMA2_14_STOP_W::new(self)
    }
    ///Bit 31 - GPDMA2 channel 15 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_15_stop(&mut self) -> DBG_GPDMA2_15_STOP_W<31> {
        DBG_GPDMA2_15_STOP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DBGMCU AHB1 peripheral freeze register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb1fzr](index.html) module
pub struct AHB1FZR_SPEC;
impl crate::RegisterSpec for AHB1FZR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb1fzr::R](R) reader structure
impl crate::Readable for AHB1FZR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb1fzr::W](W) writer structure
impl crate::Writable for AHB1FZR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB1FZR to value 0
impl crate::Resettable for AHB1FZR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
