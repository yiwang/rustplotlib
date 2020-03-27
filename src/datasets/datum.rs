pub trait BarDatum<T> {
    fn get_category(&self) -> &T;
    fn get_value(&self) -> &f32;
    fn get_key(&self) -> &str;
}

impl<T> BarDatum<T> for (T, f32, String) {
    fn get_category(&self) -> &T {
        &self.0
    }

    fn get_value(&self) -> &f32 {
        &self.1
    }

    fn get_key(&self) -> &str {
        &self.2.as_ref()
    }
}

impl<T> BarDatum<T> for (T, f32) {
    fn get_category(&self) -> &T {
        &self.0
    }

    fn get_value(&self) -> &f32 {
        &self.1
    }

    fn get_key(&self) -> &str {
        &""
    }
}