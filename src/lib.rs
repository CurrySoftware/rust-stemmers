use std::borrow::Cow;

mod simple_replace;
mod simple_insert;
mod simple_substring;
mod german;
mod combined;


#[derive(Debug)]
pub struct SnowballEnv<'a> {
    current: Cow<'a, str>,
    cursor: usize,
    limit: usize,
    limit_backward: usize,
    bra: usize,
    ket: usize,
}


struct Among(&'static str, i32, i32, Option<Box<Fn(&SnowballEnv) -> bool + Sync>>);

impl<'a> SnowballEnv<'a> {
    fn create(value: &'a str) -> Self {
        let len = value.len();
        SnowballEnv {
            current: Cow::from(value),
            cursor: 0,
            limit: len,
            limit_backward: 0,
            bra: 0,
            ket: len,
        }
    }

    fn get_current(self) -> Cow<'a, str> {
        self.current
    }

    /// Check if s is after cursor.
    /// If so, move cursor to the end of s
    fn eq_s(&mut self, s: &str) -> bool {
        if self.cursor >= self.limit {
            return false;
        }
        if self.current[self.cursor..].starts_with(s) {
            self.cursor += s.len();
            while !self.current.is_char_boundary(self.cursor) {
                self.cursor += 1;
            }
            true
        } else {
            false
        }
    }

    /// Check if 's' is before cursor
    /// If so, move cursor to the beginning of s
    fn eq_s_b(&mut self, s: &str) -> bool {
        if (self.cursor as i32 - self.limit_backward as i32) < s.len() as i32 {
            false
        } else if !self.current[self.cursor - s.len()..].starts_with(s) {
            false
        } else {
            self.cursor -= s.len();
            true
        }
    }

    /// Replace string between `bra` and `ket` with s
    fn slice_from(&mut self, s: &str) -> bool {
        let mut result = String::with_capacity(self.current.len());
        {
            let (lhs, _) = self.current.split_at(self.bra);
            let (_, rhs) = self.current.split_at(self.ket);
            result.push_str(lhs);
            result.push_str(s);
            result.push_str(rhs);
        }
        self.limit = result.len();
        self.current = Cow::from(result);
        true
    }

    /// Move cursor to next charater
    fn next_char(&mut self) {
        self.cursor += 1;
        while !self.current.is_char_boundary(self.cursor) {
            self.cursor += 1;
        }
    }

    /// Move cursor to previous character
    fn previous_char(&mut self) {
        self.cursor -= 1;
        while !self.current.is_char_boundary(self.cursor) {
            self.cursor -= 1;
        }
    }


    /// Check if the char the cursor points to is in the grouping
    /// This is determined by weird magic stuff. I have no idea how it works
    fn in_grouping(&mut self, chars: &[u8], min: u8, max: u8) -> bool {
        if self.cursor >= self.limit {
            return false;
        }
        let mut ch = self.current.as_bytes()[self.cursor];
        if ch > max || ch < min {
            return false;
        }
        ch -= min;
        if (chars[(ch >> 3) as usize] & (0x1 << (ch & 0x7))) == 0 {
            return false;
        }
        self.next_char();
        return true;
    }

    fn in_grouping_b(&mut self, chars: &[u8], min: u8, max: u8) -> bool {
        if self.cursor <= self.limit_backward {
            return false;
        }
        self.previous_char();
        let mut ch = self.current.as_bytes()[self.cursor];
        self.next_char();
        if ch > max || ch < min {
            return false;
        }
        ch -= min;
        if (chars[(ch >> 3) as usize] & (0x1 << (ch & 0x7))) == 0{
            return false;
        }
        self.previous_char();
        return false;
    }

    fn out_grouping(&mut self, chars: &[u8], min: u8, max: u8) -> bool {
        if self.cursor >= self.limit {
            return false;
        }
        let mut ch = self.current.as_bytes()[self.cursor];
        if ch > max || ch < min {
            self.next_char();
            return true;
        }
        ch -= min;
        if (chars[(ch >> 3) as usize] & (0x1 << (ch & 0x7))) == 0 {
            self.next_char();
            return true;
        }
        return false;
    }

    fn out_grouping_b(&mut self, chars: &[u8], min: u8, max: u8) -> bool {
        if self.cursor <= self.limit_backward {
            return false;
        }
        self.previous_char();
        let mut ch = self.current.as_bytes()[self.cursor];
        self.next_char();
        if ch > max || ch < min {
            self.previous_char();
            return true;
        }
        ch -= min;
        if (chars[(ch >> 3) as usize] & (0x1 << (ch & 0x7))) == 0 {
            self.previous_char();
            return true;
        }
        return false;
    }

    
    /// Helper function that removes the string slice between `bra` and `ket`
    fn slice_del(&mut self) -> bool {
        self.slice_from("")
    }


    fn find_among(&mut self, amongs: &[Among]) -> i32 {
        use std::cmp::min;
        let mut i: i32 = 0;
	let mut j: i32 = amongs.len() as i32;

	let c = self.cursor;
	let l = self.limit;

	let mut common_i = 0;
	let mut common_j = 0;

	let mut first_key_inspected = false;

	loop {
	    let k = i + ((j - i) >> 1);
	    let mut diff: i32 = 0;
	    let mut common = min(common_i, common_j);
	    let w = &amongs[k as usize];
            for lvar in common..w.0.len() {
		if c + common == l {
		    diff = -1;
		    break;
		}
                println!("{:?}", self);
		diff = self.current.as_bytes()[c + common] as i32 - w.0.as_bytes()[lvar] as i32;
		if diff != 0 { break; }
		common += 1;
	    }
	    if diff < 0 {
		j = k;
		common_j = common;
	    } else {
		i = k;
		common_i = common;
	    }
	    if j - i <= 1 {
		if i > 0 { break; }
		if j == i { break; }
		if first_key_inspected { break; }
		first_key_inspected = true;
	    }
	}
	loop {            
	    let w = &amongs[i as usize];
	    if common_i >= w.0.len() {
	        self.cursor = c + w.0.len();
                if let Some(ref method) = w.3 {
                    let res = method(self);
		    self.cursor = c + w.0.len();
                    if res { return w.2 };
                } else {
                    return w.2;
                }
	    }
	    i = w.1;
	    if i < 0 { return 0 };
	}
    }
    
    fn find_among_b(&mut self, amongs: &[Among]) -> i32 {
        let mut i: i32 = 0;
	let mut j: i32 = amongs.len() as i32;

	let c = self.cursor;
	let lb = self.limit_backward;

	let mut common_i = 0;
	let mut common_j = 0;

	let mut first_key_inspected = false;

	loop {
	    let k = i + ((j - i) >> 1);
	    let mut diff: i32 = 0;
	    let mut common = if common_i < common_j { common_i } else { common_j };
	    let w = &amongs[k as usize];
            for lvar in (0..w.0.len() - common).rev() {
		if c - common == lb {
		    diff = -1;
		    break;
		}
		diff = self.current.as_bytes()[c - 1 - common] as i32 - w.0.as_bytes()[lvar] as i32;
		if diff != 0 { break; }
		common += 1;
	    }
	    if diff < 0 {
		j = k;
		common_j = common;
	    } else {
		i = k;
		common_i = common;
	    }
	    if j - i <= 1 {
		if i > 0 { break; }
		if j == i { break; }
		if first_key_inspected { break; }
		first_key_inspected = true;
	    }
	}
	loop {
            
	    let w = &amongs[i as usize];
	    if common_i >= w.0.len() {
	        self.cursor = c - w.0.len();
                if let Some(ref method) = w.3 {
                    let res = method(self);
		    self.cursor = c - w.0.len();
                    if res { return w.2 };
                } else {
                    return w.2;
                }
	    }
	    i = w.1;
	    if i < 0 { return 0 };
	}
    }
}


#[cfg(test)]
mod tests {
    use SnowballEnv;

    fn stemms_to<F: Fn(&mut SnowballEnv) -> bool>(lhs: &str, rhs: &str, stemmer: F) {
        let mut env = SnowballEnv::create(lhs);
        stemmer(&mut env);
        assert_eq!(env.get_current(), rhs);
    }

    #[test]
    fn simple_replace_test() {
        use simple_replace::_stem;
        stemms_to("ß", "ss", _stem);
        stemms_to("ss", "ss", _stem);
        stemms_to("test", "test", _stem);
        stemms_to("ß\nss\nß", "ss\nss\nss", _stem);
    }

    #[test]
    fn simple_insert_test() {
        use simple_insert::_stem;
        stemms_to("uuu", "uUu", _stem);
        stemms_to("yyy", "yYy", _stem);
        stemms_to("aa", "aa", _stem);
        stemms_to("test", "test", _stem);
    }

    #[test]
    fn combined_test() {
        use combined::_stem;
        stemms_to("uuußs", "uUusss", _stem);
        stemms_to("ßsyy", "sssyy", _stem);
    }

    #[test]
    fn german_test() {
        use german::_stem;
        
        stemms_to("haß", "hass", _stem);
        stemms_to("hause", "haus", _stem);
        stemms_to("käuflich", "kauflich", _stem);
    }
}
