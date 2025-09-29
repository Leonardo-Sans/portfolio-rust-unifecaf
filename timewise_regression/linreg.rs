/// Regressão linear simples (y = a + b*x)
pub struct LinReg {
    pub a: f64, // intercepto
    pub b: f64, // coef angular
}

impl LinReg {
    pub fn fit(x: &[f64], y: &[f64]) -> Option<Self> {
        if x.len() != y.len() || x.len() == 0 {
            return None;
        }
        let n = x.len() as f64;
        let sum_x: f64 = x.iter().sum();
        let sum_y: f64 = y.iter().sum();
        let sum_xy: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi*yi).sum();
        let sum_x2: f64 = x.iter().map(|xi| xi*xi).sum();

        let denom = n * sum_x2 - sum_x * sum_x;
        if denom.abs() < 1e-12 {
            return None;
        }
        let b = (n * sum_xy - sum_x * sum_y) / denom;
        let a = (sum_y - b * sum_x) / n;
        Some(Self { a, b })
    }

    pub fn predict(&self, x: f64) -> f64 {
        self.a + self.b * x
    }

    pub fn mse(&self, x: &[f64], y: &[f64]) -> Option<f64> {
        if x.len() != y.len() || x.len() == 0 { return None; }
        let n = x.len() as f64;
        let s: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| {
            let yhat = self.predict(*xi);
            let err = yhat - *yi;
            err*err
        }).sum();
        Some(s / n)
    }

    pub fn r2(&self, x: &[f64], y: &[f64]) -> Option<f64> {
        if x.len() != y.len() || x.len() == 0 { return None; }
        let mean_y: f64 = y.iter().sum::<f64>() / (y.len() as f64);
        let ss_tot: f64 = y.iter().map(|yi| {
            let d = yi - mean_y;
            d*d
        }).sum();
        let ss_res: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| {
            let yhat = self.predict(*xi);
            let d = yi - yhat;
            d*d
        }).sum();
        if ss_tot.abs() < 1e-12 { return None; }
        Some(1.0 - ss_res/ss_tot)
    }
}