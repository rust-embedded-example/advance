#[doc = "Register `SR4` reader"]
pub type R = crate::R<Sr4Spec>;
#[doc = "Register `SR4` writer"]
pub type W = crate::W<Sr4Spec>;
#[doc = "Field `IRS` reader - IRS"]
pub type IrsR = crate::BitReader;
#[doc = "Field `IRS` writer - IRS"]
pub type IrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILS` reader - ILS"]
pub type IlsR = crate::BitReader;
#[doc = "Field `ILS` writer - ILS"]
pub type IlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFS` reader - IFS"]
pub type IfsR = crate::BitReader;
#[doc = "Field `IFS` writer - IFS"]
pub type IfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IREN` reader - IREN"]
pub type IrenR = crate::BitReader;
#[doc = "Field `IREN` writer - IREN"]
pub type IrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILEN` reader - ILEN"]
pub type IlenR = crate::BitReader;
#[doc = "Field `ILEN` writer - ILEN"]
pub type IlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFEN` reader - IFEN"]
pub type IfenR = crate::BitReader;
#[doc = "Field `IFEN` writer - IFEN"]
pub type IfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEMPT` reader - FEMPT"]
pub type FemptR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IRS"]
    #[inline(always)]
    pub fn irs(&self) -> IrsR {
        IrsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ILS"]
    #[inline(always)]
    pub fn ils(&self) -> IlsR {
        IlsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IFS"]
    #[inline(always)]
    pub fn ifs(&self) -> IfsR {
        IfsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IREN"]
    #[inline(always)]
    pub fn iren(&self) -> IrenR {
        IrenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ILEN"]
    #[inline(always)]
    pub fn ilen(&self) -> IlenR {
        IlenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IFEN"]
    #[inline(always)]
    pub fn ifen(&self) -> IfenR {
        IfenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FEMPT"]
    #[inline(always)]
    pub fn fempt(&self) -> FemptR {
        FemptR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRS"]
    #[inline(always)]
    pub fn irs(&mut self) -> IrsW<'_, Sr4Spec> {
        IrsW::new(self, 0)
    }
    #[doc = "Bit 1 - ILS"]
    #[inline(always)]
    pub fn ils(&mut self) -> IlsW<'_, Sr4Spec> {
        IlsW::new(self, 1)
    }
    #[doc = "Bit 2 - IFS"]
    #[inline(always)]
    pub fn ifs(&mut self) -> IfsW<'_, Sr4Spec> {
        IfsW::new(self, 2)
    }
    #[doc = "Bit 3 - IREN"]
    #[inline(always)]
    pub fn iren(&mut self) -> IrenW<'_, Sr4Spec> {
        IrenW::new(self, 3)
    }
    #[doc = "Bit 4 - ILEN"]
    #[inline(always)]
    pub fn ilen(&mut self) -> IlenW<'_, Sr4Spec> {
        IlenW::new(self, 4)
    }
    #[doc = "Bit 5 - IFEN"]
    #[inline(always)]
    pub fn ifen(&mut self) -> IfenW<'_, Sr4Spec> {
        IfenW::new(self, 5)
    }
}
#[doc = "FIFO status and interrupt register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`sr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr4Spec;
impl crate::RegisterSpec for Sr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr4::R`](R) reader structure"]
impl crate::Readable for Sr4Spec {}
#[doc = "`write(|w| ..)` method takes [`sr4::W`](W) writer structure"]
impl crate::Writable for Sr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR4 to value 0x40"]
impl crate::Resettable for Sr4Spec {
    const RESET_VALUE: u32 = 0x40;
}
