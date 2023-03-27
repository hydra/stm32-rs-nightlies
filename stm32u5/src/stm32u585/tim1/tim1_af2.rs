///Register `TIM1_AF2` reader
pub struct R(crate::R<TIM1_AF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM1_AF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM1_AF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM1_AF2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM1_AF2` writer
pub struct W(crate::W<TIM1_AF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM1_AF2_SPEC>;
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
impl From<crate::W<TIM1_AF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM1_AF2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BK2INE` reader - TIMx_BKIN2 input enable This bit enables the TIMx_BKIN2 alternate function input for the timerâs tim_brk2 input. TIMx_BKIN2 input is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2INE_R = crate::BitReader<bool>;
///Field `BK2INE` writer - TIMx_BKIN2 input enable This bit enables the TIMx_BKIN2 alternate function input for the timerâs tim_brk2 input. TIMx_BKIN2 input is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2INE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
///Field `BK2CMP1E` reader - tim_brk2_cmp1 enable This bit enables the tim_brk2_cmp1 for the timerâs tim_brk2 input. tim_brk2_cmp1 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP1E_R = crate::BitReader<bool>;
///Field `BK2CMP1E` writer - tim_brk2_cmp1 enable This bit enables the tim_brk2_cmp1 for the timerâs tim_brk2 input. tim_brk2_cmp1 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
///Field `BK2CMP2E` reader - tim_brk2_cmp2 enable This bit enables the tim_brk2_cmp2 for the timerâs tim_brk2 input. tim_brk2_cmp2 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP2E_R = crate::BitReader<bool>;
///Field `BK2CMP2E` writer - tim_brk2_cmp2 enable This bit enables the tim_brk2_cmp2 for the timerâs tim_brk2 input. tim_brk2_cmp2 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
///Field `BK2CMP3E` reader - tim_brk2_cmp3 enable This bit enables the tim_brk2_cmp3 for the timerâs tim_brk2 input. tim_brk2_cmp3 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP3E_R = crate::BitReader<bool>;
///Field `BK2CMP3E` writer - tim_brk2_cmp3 enable This bit enables the tim_brk2_cmp3 for the timerâs tim_brk2 input. tim_brk2_cmp3 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP3E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
///Field `BK2CMP4E` reader - tim_brk2_cmp4 enable This bit enables the tim_brk2_cmp4 for the timerâs tim_brk2 input. tim_brk2_cmp4 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP4E_R = crate::BitReader<bool>;
///Field `BK2CMP4E` writer - tim_brk2_cmp4 enable This bit enables the tim_brk2_cmp4 for the timerâs tim_brk2 input. tim_brk2_cmp4 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP4E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
///Field `BK2CMP5E` reader - tim_brk2_cmp5 enable This bit enables the tim_brk2_cmp5 for the timerâs tim_brk2 input. tim_brk2_cmp5 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP5E_R = crate::BitReader<bool>;
///Field `BK2CMP5E` writer - tim_brk2_cmp5 enable This bit enables the tim_brk2_cmp5 for the timerâs tim_brk2 input. tim_brk2_cmp5 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP5E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
///Field `BK2CMP6E` reader - tim_brk2_cmp6 enable This bit enables the tim_brk2_cmp6 for the timerâs tim_brk2 input. tim_brk2_cmp6 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP6E_R = crate::BitReader<bool>;
///Field `BK2CMP6E` writer - tim_brk2_cmp6 enable This bit enables the tim_brk2_cmp6 for the timerâs tim_brk2 input. tim_brk2_cmp6 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP6E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
///Field `BK2CMP7E` reader - tim_brk2_cmp7 enable This bit enables the tim_brk2_cmp7 for the timerâs tim_brk2 input. tim_brk2_cmp7 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP7E_R = crate::BitReader<bool>;
///Field `BK2CMP7E` writer - tim_brk2_cmp7 enable This bit enables the tim_brk2_cmp7 for the timerâs tim_brk2 input. tim_brk2_cmp7 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP7E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
///Field `BK2CMP8E` reader - tim_brk2_cmp8 enable This bit enables the tim_brk2_cmp8 for the timerâs tim_brk2 input. tim_brk2_cmp8 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP8E_R = crate::BitReader<bool>;
///Field `BK2CMP8E` writer - tim_brk2_cmp8 enable This bit enables the tim_brk2_cmp8 for the timerâs tim_brk2 input. tim_brk2_cmp8 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP8E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
///Field `BK2INP` reader - TIMx_BKIN2 input polarity This bit selects the TIMx_BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2INP_R = crate::BitReader<bool>;
///Field `BK2INP` writer - TIMx_BKIN2 input polarity This bit selects the TIMx_BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2INP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
///Field `BK2CMP1P` reader - tim_brk2_cmp1 input polarity This bit selects the tim_brk2_cmp1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP1P_R = crate::BitReader<bool>;
///Field `BK2CMP1P` writer - tim_brk2_cmp1 input polarity This bit selects the tim_brk2_cmp1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
///Field `BK2CMP2P` reader - tim_brk2_cmp2 input polarity This bit selects the tim_brk2_cmp2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP2P_R = crate::BitReader<bool>;
///Field `BK2CMP2P` writer - tim_brk2_cmp2 input polarity This bit selects the tim_brk2_cmp2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
///Field `BK2CMP3P` reader - tim_brk2_cmp3 input polarity This bit selects the tim_brk2_cmp3 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP3P_R = crate::BitReader<bool>;
///Field `BK2CMP3P` writer - tim_brk2_cmp3 input polarity This bit selects the tim_brk2_cmp3 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP3P_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
///Field `BK2CMP4P` reader - tim_brk2_cmp4 input polarity This bit selects the tim_brk2_cmp4 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP4P_R = crate::BitReader<bool>;
///Field `BK2CMP4P` writer - tim_brk2_cmp4 input polarity This bit selects the tim_brk2_cmp4 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type BK2CMP4P_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
///Field `OCRSEL` reader - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific information. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OCRSEL_R = crate::FieldReader<u8, u8>;
///Field `OCRSEL` writer - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific information. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OCRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM1_AF2_SPEC, u8, u8, 3, O>;
impl R {
    ///Bit 0 - TIMx_BKIN2 input enable This bit enables the TIMx_BKIN2 alternate function input for the timerâs tim_brk2 input. TIMx_BKIN2 input is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - tim_brk2_cmp1 enable This bit enables the tim_brk2_cmp1 for the timerâs tim_brk2 input. tim_brk2_cmp1 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> BK2CMP1E_R {
        BK2CMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - tim_brk2_cmp2 enable This bit enables the tim_brk2_cmp2 for the timerâs tim_brk2 input. tim_brk2_cmp2 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> BK2CMP2E_R {
        BK2CMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - tim_brk2_cmp3 enable This bit enables the tim_brk2_cmp3 for the timerâs tim_brk2 input. tim_brk2_cmp3 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp3e(&self) -> BK2CMP3E_R {
        BK2CMP3E_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - tim_brk2_cmp4 enable This bit enables the tim_brk2_cmp4 for the timerâs tim_brk2 input. tim_brk2_cmp4 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp4e(&self) -> BK2CMP4E_R {
        BK2CMP4E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - tim_brk2_cmp5 enable This bit enables the tim_brk2_cmp5 for the timerâs tim_brk2 input. tim_brk2_cmp5 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp5e(&self) -> BK2CMP5E_R {
        BK2CMP5E_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - tim_brk2_cmp6 enable This bit enables the tim_brk2_cmp6 for the timerâs tim_brk2 input. tim_brk2_cmp6 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp6e(&self) -> BK2CMP6E_R {
        BK2CMP6E_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - tim_brk2_cmp7 enable This bit enables the tim_brk2_cmp7 for the timerâs tim_brk2 input. tim_brk2_cmp7 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp7e(&self) -> BK2CMP7E_R {
        BK2CMP7E_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - tim_brk2_cmp8 enable This bit enables the tim_brk2_cmp8 for the timerâs tim_brk2 input. tim_brk2_cmp8 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp8e(&self) -> BK2CMP8E_R {
        BK2CMP8E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TIMx_BKIN2 input polarity This bit selects the TIMx_BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - tim_brk2_cmp1 input polarity This bit selects the tim_brk2_cmp1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> BK2CMP1P_R {
        BK2CMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - tim_brk2_cmp2 input polarity This bit selects the tim_brk2_cmp2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> BK2CMP2P_R {
        BK2CMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - tim_brk2_cmp3 input polarity This bit selects the tim_brk2_cmp3 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp3p(&self) -> BK2CMP3P_R {
        BK2CMP3P_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - tim_brk2_cmp4 input polarity This bit selects the tim_brk2_cmp4 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bk2cmp4p(&self) -> BK2CMP4P_R {
        BK2CMP4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 16:18 - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific information. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ocrsel(&self) -> OCRSEL_R {
        OCRSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    ///Bit 0 - TIMx_BKIN2 input enable This bit enables the TIMx_BKIN2 alternate function input for the timerâs tim_brk2 input. TIMx_BKIN2 input is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn bk2ine(&mut self) -> BK2INE_W<0> {
        BK2INE_W::new(self)
    }
    ///Bit 1 - tim_brk2_cmp1 enable This bit enables the tim_brk2_cmp1 for the timerâs tim_brk2 input. tim_brk2_cmp1 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1e(&mut self) -> BK2CMP1E_W<1> {
        BK2CMP1E_W::new(self)
    }
    ///Bit 2 - tim_brk2_cmp2 enable This bit enables the tim_brk2_cmp2 for the timerâs tim_brk2 input. tim_brk2_cmp2 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2e(&mut self) -> BK2CMP2E_W<2> {
        BK2CMP2E_W::new(self)
    }
    ///Bit 3 - tim_brk2_cmp3 enable This bit enables the tim_brk2_cmp3 for the timerâs tim_brk2 input. tim_brk2_cmp3 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp3e(&mut self) -> BK2CMP3E_W<3> {
        BK2CMP3E_W::new(self)
    }
    ///Bit 4 - tim_brk2_cmp4 enable This bit enables the tim_brk2_cmp4 for the timerâs tim_brk2 input. tim_brk2_cmp4 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp4e(&mut self) -> BK2CMP4E_W<4> {
        BK2CMP4E_W::new(self)
    }
    ///Bit 5 - tim_brk2_cmp5 enable This bit enables the tim_brk2_cmp5 for the timerâs tim_brk2 input. tim_brk2_cmp5 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp5e(&mut self) -> BK2CMP5E_W<5> {
        BK2CMP5E_W::new(self)
    }
    ///Bit 6 - tim_brk2_cmp6 enable This bit enables the tim_brk2_cmp6 for the timerâs tim_brk2 input. tim_brk2_cmp6 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp6e(&mut self) -> BK2CMP6E_W<6> {
        BK2CMP6E_W::new(self)
    }
    ///Bit 7 - tim_brk2_cmp7 enable This bit enables the tim_brk2_cmp7 for the timerâs tim_brk2 input. tim_brk2_cmp7 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp7e(&mut self) -> BK2CMP7E_W<7> {
        BK2CMP7E_W::new(self)
    }
    ///Bit 8 - tim_brk2_cmp8 enable This bit enables the tim_brk2_cmp8 for the timerâs tim_brk2 input. tim_brk2_cmp8 output is 'ORedâ with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp8e(&mut self) -> BK2CMP8E_W<8> {
        BK2CMP8E_W::new(self)
    }
    ///Bit 9 - TIMx_BKIN2 input polarity This bit selects the TIMx_BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn bk2inp(&mut self) -> BK2INP_W<9> {
        BK2INP_W::new(self)
    }
    ///Bit 10 - tim_brk2_cmp1 input polarity This bit selects the tim_brk2_cmp1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1p(&mut self) -> BK2CMP1P_W<10> {
        BK2CMP1P_W::new(self)
    }
    ///Bit 11 - tim_brk2_cmp2 input polarity This bit selects the tim_brk2_cmp2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2p(&mut self) -> BK2CMP2P_W<11> {
        BK2CMP2P_W::new(self)
    }
    ///Bit 12 - tim_brk2_cmp3 input polarity This bit selects the tim_brk2_cmp3 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp3p(&mut self) -> BK2CMP3P_W<12> {
        BK2CMP3P_W::new(self)
    }
    ///Bit 13 - tim_brk2_cmp4 input polarity This bit selects the tim_brk2_cmp4 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp4p(&mut self) -> BK2CMP4P_W<13> {
        BK2CMP4P_W::new(self)
    }
    ///Bits 16:18 - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific information. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn ocrsel(&mut self) -> OCRSEL_W<16> {
        OCRSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM1 alternate function register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim1_af2](index.html) module
pub struct TIM1_AF2_SPEC;
impl crate::RegisterSpec for TIM1_AF2_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim1_af2::R](R) reader structure
impl crate::Readable for TIM1_AF2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim1_af2::W](W) writer structure
impl crate::Writable for TIM1_AF2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM1_AF2 to value 0x01
impl crate::Resettable for TIM1_AF2_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
