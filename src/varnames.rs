#[derive(Default, Clone)]
pub struct VarNames(usize);

impl Iterator for VarNames {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut out = "$_".to_string();
        let mut value = self.0;

        while value > 0 || out.len() <= 2 {
            out.push(b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"[value % 52] as char);
            value /= 52;
        }

        self.0 += 1;

        return Some(out);
    }
}
