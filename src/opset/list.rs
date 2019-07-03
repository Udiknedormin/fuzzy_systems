use super::Opset;

use crate::value::Membership;


fuzzy_opset! {
    /// Yager fuzzy operation set with w = 1.
    pub Yager1 {
         ~x   = 1.0 - x;
        a | b = (a + b).min(1.0);
        a & b = (a + b).max(1.0) - 1.0;
    }

    /// Yager fuzzy operation set with w -> inf.
    pub YagerInf {
         ~x   = 1.0 - x;
        a | b = a.max(b);
        a & b = a.min(b);
    }

    /// Hamacher fuzzy operation set with gamma = 0.
    pub Hamacher0 {
         ~x   = 1.0 - x;
        a | b = (a + b - 2.0 * a * b) / (1.0 - a * b);
        a & b = (a * b) / (a + b - a * b);
    }

    /// Hamacher fuzzy operation set with gamma = 1.
    pub Hamacher1 {
         ~x   = 1.0 - x;
        a | b = a + b - a * b;
        a & b = a * b;
    }

    /// Hamacher fuzzy operation set with gamma = 2.
    pub Hamacher2 {
         ~x   = 1.0 - x;
        a | b = (a + b) / (1.0 + a * b);
        a & b = (a * b) / (2.0 - a - b - a * b);
    }
}

