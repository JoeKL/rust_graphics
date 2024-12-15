use crate::types::math::Vector3D;

#[derive(Debug, Clone, Copy)]
pub struct ColorRGB {
    as_u32: u32,
    r: u8,
    g: u8,
    b: u8,
}

impl ColorRGB {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> ColorRGB {
        let color = (r as u32) << 16 | (g as u32) << 8 | b as u32;
        ColorRGB { as_u32: color, r, g, b }
    }    

    pub fn f32_to_color_component(value: f32) -> u8 {
        (f32::min(value, 1.0) * 255.0) as u8
    }
    
    pub fn from_u32(color: u32) -> ColorRGB {
        let r = ((color >> 16) & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = (color & 0xFF) as u8;
        ColorRGB { as_u32: color, r, g, b }
    }

    pub fn update_color(&mut self) {
        self.as_u32 = (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32;
    }

    #[allow(dead_code)]
    pub fn set(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
        self.as_u32 = (r as u32) << 16 | (g as u32) << 8 | b as u32;
    }

    #[allow(dead_code)]
    pub fn set_r(&mut self, r: u8) -> &mut Self {
        self.r = r;
        self.update_color();
        self
    }

    #[allow(dead_code)]
    pub fn set_g(&mut self, g: u8) -> &mut Self {
        self.g = g;
        self.update_color();
        self
    }

    #[allow(dead_code)]
    pub fn set_b(&mut self, b: u8) -> &mut Self {
        self.b = b;
        self.update_color();
        self
    }

    pub fn get_as_u32(&self) -> u32 {self.as_u32}
    pub fn get_r(&self) -> u8 {self.r}
    pub fn get_g(&self) -> u8 {self.g}
    pub fn get_b(&self) -> u8 {self.b}

    pub fn to_vector(self) -> Vector3D {
        Vector3D::new(
            self.get_r() as f32 / 255.0,
            self.get_g() as f32 / 255.0,
            self.get_b() as f32 / 255.0,
        )
    }

    pub fn from_vector(vec: &Vector3D) -> Self {
        ColorRGB::from_rgb(
            Self::f32_to_color_component(vec.x),
            Self::f32_to_color_component(vec.y),
            Self::f32_to_color_component(vec.z)
        )
    }
    #[allow(dead_code)]
    pub const BLACK: ColorRGB = ColorRGB { 
        as_u32: 0x000000, 
        r: 0, 
        g: 0, 
        b: 0 
    };
    #[allow(dead_code)]
    pub const WHITE: ColorRGB = ColorRGB { 
        as_u32: 0xFFFFFF, 
        r: 255, 
        g: 255, 
        b: 255 
    };
    #[allow(dead_code)]
    pub const RED: ColorRGB = ColorRGB { 
        as_u32: 0xFF0000, 
        r: 255, 
        g: 0, 
        b: 0 
    };
    #[allow(dead_code)]
    pub const GREEN: ColorRGB = ColorRGB { 
        as_u32: 0x00FF00, 
        r: 0, 
        g: 255, 
        b: 0 
    };
    #[allow(dead_code)]
    pub const BLUE: ColorRGB = ColorRGB { 
        as_u32: 0x0000FF, 
        r: 0, 
        g: 0, 
        b: 255 
    };
    #[allow(dead_code)]
    pub const YELLOW: ColorRGB = ColorRGB { 
        as_u32: 0xFFFF00, 
        r: 255, 
        g: 255, 
        b: 0 
    };
    #[allow(dead_code)]
    pub const CYAN: ColorRGB = ColorRGB { 
        as_u32: 0x00FFFF, 
        r: 0, 
        g: 255, 
        b: 255 
    };
    #[allow(dead_code)]
    pub const MAGENTA: ColorRGB = ColorRGB { 
        as_u32: 0xFF00FF, 
        r: 255, 
        g: 0, 
        b: 255 
    };
    
}


