#![feature(external_doc)]
use ta_common::fixed_queue::FixedQueue;
use ta_common::traits::Indicator;
use ema_rs::EMA;
#[doc(include = "../README.md")]
pub struct CVI {
    period: u32,
    ema: EMA,
    history: FixedQueue<f64>,
}

impl CVI {
    pub fn new(period: u32) -> CVI {
        Self {
            period,
            history: FixedQueue::new(period + 1),
            ema: EMA::new(period),
        }
    }
}

impl Indicator<[f64; 2], Option<f64>> for CVI {
    fn next(&mut self, input: [f64; 2]) -> Option<f64> {
        let [high, low] = input;
        let hema = self.ema.next(high - low);
        self.history.add(hema);
        if self.history.is_full() {
            let prev = self.history.at(0).unwrap();
            let cvi = 100.0 * ((hema - prev) / prev);
            return Some(cvi);
        }
        None
    }

    fn reset(&mut self) {
        self.history.clear();
        self.ema.reset();
    }
}


#[cfg(test)]
mod tests {
    use crate::CVI;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
        let mut cvi = CVI::new(5);
        assert_eq!(cvi.next([82.15, 81.29]), None);
        assert_eq!(cvi.next([81.89, 80.64]), None);
        assert_eq!(cvi.next([83.03, 81.31]), None);
        assert_eq!(cvi.next([83.30, 82.65]), None);
        assert_eq!(cvi.next([83.85, 83.07]), None);
        assert_eq!(cvi.next([83.90, 83.11]), Some(4.464542061441478));
        assert_eq!(cvi.next([83.33, 82.49]), Some(-11.219187762397418));
        assert_eq!(cvi.next([84.30, 82.30]), Some(1.5637860082306099));
        assert_eq!(cvi.next([84.84, 84.15]), Some(2.5210712792415677));
        assert_eq!(cvi.next([85.00, 84.11]), Some(5.682116365544989));
        assert_eq!(cvi.next([85.90, 84.03]), Some(44.08805917058712));
        assert_eq!(cvi.next([86.58, 85.39]), Some(43.3166782081056));
        assert_eq!(cvi.next([86.98, 85.76]), Some(-0.4937226078952195));
        assert_eq!(cvi.next([88.00, 87.17]), Some(3.994412353229816));
        assert_eq!(cvi.next([87.87, 87.01]), Some(1.8239886289963296));
    }
}
