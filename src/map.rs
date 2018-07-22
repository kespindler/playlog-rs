

#[macro_export]
macro_rules! map {
    ( $( $i:ident = $e:expr) , *) => {
        {
            let mut h = HashMap::new();
            $(
                h.insert($i, $e);
            )*
            h
        }
    };
    ( $( $i:ident : $e:expr) , *) => {
        {
            let mut h = HashMap::new();
            $(
                h.insert(stringify!($i), $e);
            )*
            h
        }
    };
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    #[macro_use]
    use super::*;

    const NAME: &str = "123";

    #[test]
    fn map_macro() {
        let h = map!(NAME="b");
        let g = map!(name: "b");

        assert_eq!(h[NAME], "b");
        assert_eq!(g["name"], "b");
    }
}
