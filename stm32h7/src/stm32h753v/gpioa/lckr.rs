///Register `LCKR` reader
pub struct R(crate::R<LCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCKR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LCKR` writer
pub struct W(crate::W<LCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCKR_SPEC>;
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
impl From<crate::W<LCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCKR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LCK0` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK0_R = crate::BitReader<LCK0_A>;
///Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK0_A {
    ///0: Port configuration not locked
    Unlocked = 0,
    ///1: Port configuration locked
    Locked = 1,
}
impl From<LCK0_A> for bool {
    #[inline(always)]
    fn from(variant: LCK0_A) -> Self {
        variant as u8 != 0
    }
}
impl LCK0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LCK0_A {
        match self.bits {
            false => LCK0_A::Unlocked,
            true => LCK0_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LCK0_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LCK0_A::Locked
    }
}
///Field `LCK0` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub type LCK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCKR_SPEC, LCK0_A, O>;
impl<'a, const O: u8> LCK0_W<'a, O> {
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK0_A::Unlocked)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK0_A::Locked)
    }
}
///Field `LCK1` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_R as LCK1_R;
///Field `LCK2` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_R as LCK2_R;
///Field `LCK3` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_R as LCK3_R;
///Field `LCK4` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_R as LCK4_R;
///Field `LCK5` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_R as LCK5_R;
///Field `LCK6` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_R as LCK6_R;
///Field `LCK7` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_R as LCK7_R;
///Field `LCK8` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_R as LCK8_R;
///Field `LCK9` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_R as LCK9_R;
///Field `LCK10` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_R as LCK10_R;
///Field `LCK11` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_R as LCK11_R;
///Field `LCK12` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_R as LCK12_R;
///Field `LCK13` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_R as LCK13_R;
///Field `LCK14` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_R as LCK14_R;
///Field `LCK15` reader - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_R as LCK15_R;
///Field `LCK1` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_W as LCK1_W;
///Field `LCK2` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_W as LCK2_W;
///Field `LCK3` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_W as LCK3_W;
///Field `LCK4` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_W as LCK4_W;
///Field `LCK5` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_W as LCK5_W;
///Field `LCK6` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_W as LCK6_W;
///Field `LCK7` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_W as LCK7_W;
///Field `LCK8` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_W as LCK8_W;
///Field `LCK9` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_W as LCK9_W;
///Field `LCK10` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_W as LCK10_W;
///Field `LCK11` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_W as LCK11_W;
///Field `LCK12` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_W as LCK12_W;
///Field `LCK13` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_W as LCK13_W;
///Field `LCK14` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_W as LCK14_W;
///Field `LCK15` writer - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
pub use LCK0_W as LCK15_W;
///Field `LCKK` reader - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\[16\]
///= 1 + LCKR\[15:0\]
///WR LCKR\[16\]
///= 0 + LCKR\[15:0\]
///WR LCKR\[16\]
///= 1 + LCKR\[15:0\]
///RD LCKR RD LCKR\[16\]
///= 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\[15:0\]
///must not change. Any error in the lock sequence aborts the lock. After the first lock sequence on any bit of the port, any read access on the LCKK bit will return 1 until the next MCU reset or peripheral reset.
pub type LCKK_R = crate::BitReader<LCKK_A>;
///Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\[16\]
///= 1 + LCKR\[15:0\]
///WR LCKR\[16\]
///= 0 + LCKR\[15:0\]
///WR LCKR\[16\]
///= 1 + LCKR\[15:0\]
///RD LCKR RD LCKR\[16\]
///= 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\[15:0\]
///must not change. Any error in the lock sequence aborts the lock. After the first lock sequence on any bit of the port, any read access on the LCKK bit will return 1 until the next MCU reset or peripheral reset.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCKK_A {
    ///0: Port configuration lock key not active
    NotActive = 0,
    ///1: Port configuration lock key active
    Active = 1,
}
impl From<LCKK_A> for bool {
    #[inline(always)]
    fn from(variant: LCKK_A) -> Self {
        variant as u8 != 0
    }
}
impl LCKK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LCKK_A {
        match self.bits {
            false => LCKK_A::NotActive,
            true => LCKK_A::Active,
        }
    }
    ///Checks if the value of the field is `NotActive`
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == LCKK_A::NotActive
    }
    ///Checks if the value of the field is `Active`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == LCKK_A::Active
    }
}
///Field `LCKK` writer - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\[16\]
///= 1 + LCKR\[15:0\]
///WR LCKR\[16\]
///= 0 + LCKR\[15:0\]
///WR LCKR\[16\]
///= 1 + LCKR\[15:0\]
///RD LCKR RD LCKR\[16\]
///= 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\[15:0\]
///must not change. Any error in the lock sequence aborts the lock. After the first lock sequence on any bit of the port, any read access on the LCKK bit will return 1 until the next MCU reset or peripheral reset.
pub type LCKK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCKR_SPEC, LCKK_A, O>;
impl<'a, const O: u8> LCKK_W<'a, O> {
    ///Port configuration lock key not active
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(LCKK_A::NotActive)
    }
    ///Port configuration lock key active
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(LCKK_A::Active)
    }
}
impl R {
    ///Bit 0 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck2(&self) -> LCK2_R {
        LCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck4(&self) -> LCK4_R {
        LCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck5(&self) -> LCK5_R {
        LCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck6(&self) -> LCK6_R {
        LCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck7(&self) -> LCK7_R {
        LCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck8(&self) -> LCK8_R {
        LCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck9(&self) -> LCK9_R {
        LCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck10(&self) -> LCK10_R {
        LCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck11(&self) -> LCK11_R {
        LCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck12(&self) -> LCK12_R {
        LCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck13(&self) -> LCK13_R {
        LCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck14(&self) -> LCK14_R {
        LCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    pub fn lck15(&self) -> LCK15_R {
        LCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\[16\]
    ///= 1 + LCKR\[15:0\]
    ///WR LCKR\[16\]
    ///= 0 + LCKR\[15:0\]
    ///WR LCKR\[16\]
    ///= 1 + LCKR\[15:0\]
    ///RD LCKR RD LCKR\[16\]
    ///= 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\[15:0\]
    ///must not change. Any error in the lock sequence aborts the lock. After the first lock sequence on any bit of the port, any read access on the LCKK bit will return 1 until the next MCU reset or peripheral reset.
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck0(&mut self) -> LCK0_W<0> {
        LCK0_W::new(self)
    }
    ///Bit 1 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck1(&mut self) -> LCK1_W<1> {
        LCK1_W::new(self)
    }
    ///Bit 2 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck2(&mut self) -> LCK2_W<2> {
        LCK2_W::new(self)
    }
    ///Bit 3 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck3(&mut self) -> LCK3_W<3> {
        LCK3_W::new(self)
    }
    ///Bit 4 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck4(&mut self) -> LCK4_W<4> {
        LCK4_W::new(self)
    }
    ///Bit 5 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck5(&mut self) -> LCK5_W<5> {
        LCK5_W::new(self)
    }
    ///Bit 6 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck6(&mut self) -> LCK6_W<6> {
        LCK6_W::new(self)
    }
    ///Bit 7 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck7(&mut self) -> LCK7_W<7> {
        LCK7_W::new(self)
    }
    ///Bit 8 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck8(&mut self) -> LCK8_W<8> {
        LCK8_W::new(self)
    }
    ///Bit 9 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck9(&mut self) -> LCK9_W<9> {
        LCK9_W::new(self)
    }
    ///Bit 10 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck10(&mut self) -> LCK10_W<10> {
        LCK10_W::new(self)
    }
    ///Bit 11 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck11(&mut self) -> LCK11_W<11> {
        LCK11_W::new(self)
    }
    ///Bit 12 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck12(&mut self) -> LCK12_W<12> {
        LCK12_W::new(self)
    }
    ///Bit 13 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck13(&mut self) -> LCK13_W<13> {
        LCK13_W::new(self)
    }
    ///Bit 14 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck14(&mut self) -> LCK14_W<14> {
        LCK14_W::new(self)
    }
    ///Bit 15 - Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    #[inline(always)]
    #[must_use]
    pub fn lck15(&mut self) -> LCK15_W<15> {
        LCK15_W::new(self)
    }
    ///Bit 16 - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\[16\]
    ///= 1 + LCKR\[15:0\]
    ///WR LCKR\[16\]
    ///= 0 + LCKR\[15:0\]
    ///WR LCKR\[16\]
    ///= 1 + LCKR\[15:0\]
    ///RD LCKR RD LCKR\[16\]
    ///= 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\[15:0\]
    ///must not change. Any error in the lock sequence aborts the lock. After the first lock sequence on any bit of the port, any read access on the LCKK bit will return 1 until the next MCU reset or peripheral reset.
    #[inline(always)]
    #[must_use]
    pub fn lckk(&mut self) -> LCKK_W<16> {
        LCKK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
///is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
///must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset.A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence.Each lock bit freezes a specific configuration register (control and alternate function registers).
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lckr](index.html) module
pub struct LCKR_SPEC;
impl crate::RegisterSpec for LCKR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lckr::R](R) reader structure
impl crate::Readable for LCKR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lckr::W](W) writer structure
impl crate::Writable for LCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LCKR to value 0
impl crate::Resettable for LCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
