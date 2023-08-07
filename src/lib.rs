mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn hello() -> i32 {
    let result = unsafe { bindings::hello() } as i32;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call() {
        assert_eq!(hello(), 42);
    }
}
