pub mod rust {
    use crate::Info;
    
    pub fn get_wipe_timestamp(info: &Info) -> Option<i32> {
        match &info.extra_data.tags {
            Some(data) => {
                let a: Vec<_> = data.split(",").filter(|&x| x.starts_with("born")).map(|e| (e[4..]).parse::<i32>()).collect();

                if a[0].is_err() { return None };
                
                Some(a[0].clone().unwrap())
            },
            _ => None
        }
    }
}