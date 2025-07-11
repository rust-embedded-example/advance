#[doc = "Register `TR3` reader"]
pub type R = crate::R<Tr3Spec>;
#[doc = "Register `TR3` writer"]
pub type W = crate::W<Tr3Spec>;
#[doc = "Field `LT3` reader - LT3"]
pub type Lt3R = crate::FieldReader;
#[doc = "Field `LT3` writer - LT3"]
pub type Lt3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HT3` reader - HT3"]
pub type Ht3R = crate::FieldReader;
#[doc = "Field `HT3` writer - HT3"]
pub type Ht3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - LT3"]
    #[inline(always)]
    pub fn lt3(&self) -> Lt3R {
        Lt3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - HT3"]
    #[inline(always)]
    pub fn ht3(&self) -> Ht3R {
        Ht3R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LT3"]
    #[inline(always)]
    pub fn lt3(&mut self) -> Lt3W<'_, Tr3Spec> {
        Lt3W::new(self, 0)
    }
    #[doc = "Bits 16:23 - HT3"]
    #[inline(always)]
    pub fn ht3(&mut self) -> Ht3W<'_, Tr3Spec> {
        Ht3W::new(self, 16)
    }
}
#[doc = "watchdog threshold register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`tr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tr3Spec;
impl crate::RegisterSpec for Tr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr3::R`](R) reader structure"]
impl crate::Readable for Tr3Spec {}
#[doc = "`write(|w| ..)` method takes [`tr3::W`](W) writer structure"]
impl crate::Writable for Tr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TR3 to value 0x0fff_0000"]
impl crate::Resettable for Tr3Spec {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
