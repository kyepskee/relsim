const CIE_COLOUR_MATCH: [(f64, f64, f64); 81] = [
    (0.0014, 0.0000, 0.0065),
    (0.0022, 0.0001, 0.0105),
    (0.0042, 0.0001, 0.0201),
    (0.0076, 0.0002, 0.0362),
    (0.0143, 0.0004, 0.0679),
    (0.0232, 0.0006, 0.1102),
    (0.0435, 0.0012, 0.2074),
    (0.0776, 0.0022, 0.3713),
    (0.1344, 0.0040, 0.6456),
    (0.2148, 0.0073, 1.0391),
    (0.2839, 0.0116, 1.3856),
    (0.3285, 0.0168, 1.6230),
    (0.3483, 0.0230, 1.7471),
    (0.3481, 0.0298, 1.7826),
    (0.3362, 0.0380, 1.7721),
    (0.3187, 0.0480, 1.7441),
    (0.2908, 0.0600, 1.6692),
    (0.2511, 0.0739, 1.5281),
    (0.1954, 0.0910, 1.2876),
    (0.1421, 0.1126, 1.0419),
    (0.0956, 0.1390, 0.8130),
    (0.0580, 0.1693, 0.6162),
    (0.0320, 0.2080, 0.4652),
    (0.0147, 0.2586, 0.3533),
    (0.0049, 0.3230, 0.2720),
    (0.0024, 0.4073, 0.2123),
    (0.0093, 0.5030, 0.1582),
    (0.0291, 0.6082, 0.1117),
    (0.0633, 0.7100, 0.0782),
    (0.1096, 0.7932, 0.0573),
    (0.1655, 0.8620, 0.0422),
    (0.2257, 0.9149, 0.0298),
    (0.2904, 0.9540, 0.0203),
    (0.3597, 0.9803, 0.0134),
    (0.4334, 0.9950, 0.0087),
    (0.5121, 1.0000, 0.0057),
    (0.5945, 0.9950, 0.0039),
    (0.6784, 0.9786, 0.0027),
    (0.7621, 0.9520, 0.0021),
    (0.8425, 0.9154, 0.0018),
    (0.9163, 0.8700, 0.0017),
    (0.9786, 0.8163, 0.0014),
    (1.0263, 0.7570, 0.0011),
    (1.0567, 0.6949, 0.0010),
    (1.0622, 0.6310, 0.0008),
    (1.0456, 0.5668, 0.0006),
    (1.0026, 0.5030, 0.0003),
    (0.9384, 0.4412, 0.0002),
    (0.8544, 0.3810, 0.0002),
    (0.7514, 0.3210, 0.0001),
    (0.6424, 0.2650, 0.0000),
    (0.5419, 0.2170, 0.0000),
    (0.4479, 0.1750, 0.0000),
    (0.3608, 0.1382, 0.0000),
    (0.2835, 0.1070, 0.0000),
    (0.2187, 0.0816, 0.0000),
    (0.1649, 0.0610, 0.0000),
    (0.1212, 0.0446, 0.0000),
    (0.0874, 0.0320, 0.0000),
    (0.0636, 0.0232, 0.0000),
    (0.0468, 0.0170, 0.0000),
    (0.0329, 0.0119, 0.0000),
    (0.0227, 0.0082, 0.0000),
    (0.0158, 0.0057, 0.0000),
    (0.0114, 0.0041, 0.0000),
    (0.0081, 0.0029, 0.0000),
    (0.0058, 0.0021, 0.0000),
    (0.0041, 0.0015, 0.0000),
    (0.0029, 0.0010, 0.0000),
    (0.0020, 0.0007, 0.0000),
    (0.0014, 0.0005, 0.0000),
    (0.0010, 0.0004, 0.0000),
    (0.0007, 0.0002, 0.0000),
    (0.0005, 0.0002, 0.0000),
    (0.0003, 0.0001, 0.0000),
    (0.0002, 0.0001, 0.0000),
    (0.0002, 0.0001, 0.0000),
    (0.0001, 0.0000, 0.0000),
    (0.0001, 0.0000, 0.0000),
    (0.0001, 0.0000, 0.0000),
    (0.0000, 0.0000, 0.0000),
];

pub struct XYZ {
    x: f64,
    y: f64,
    z: f64,
}

impl From<(f64, f64, f64)> for XYZ {
    fn from(a: (f64, f64, f64)) -> XYZ {
        XYZ {
            x: a.0,
            y: a.1,
            z: a.2,
        }
    }
}

// spec is the wavelength in nanometers
// y (luminosity) gets scaled by the amplitude parameter
pub fn spec_to_xyz(spec: f64, amp: Option<f64>) -> XYZ {
    let amp = amp.unwrap_or(1.0);
    debug_assert!(380.0 <= spec && spec <= 780.0);
    let mut xyz: XYZ = XYZ::from(CIE_COLOUR_MATCH[((spec - 480.0) / 5.0) as usize]);
    xyz.y *= amp;
    xyz
}

pub fn xyz_to_rgb(xyz: XYZ) -> rustbitmap::Rgba {
    let R = 3.2404542 * xyz.x - 1.5371385 * xyz.y - 0.4985314 * xyz.z;
    let G = -0.9692660 * xyz.x + 1.8760108 * xyz.y + 0.0415560 * xyz.z;
    let B = 0.0556434 * xyz.x - 0.2040259 * xyz.y + 1.0572252 * xyz.z;
    rustbitmap::Rgba::rgb((R * 256.0) as u8, (G * 256.0) as u8, (B * 256.0) as u8)
}

pub fn freq_to_rgb(l: f64, amp: f64) -> rustbitmap::Rgba {
    let mut t: f64;
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    if (l >= 400.0) && (l < 410.0) {
        t = (l - 400.0) / (410.0 - 400.0);
        r = (0.33 * t) - (0.20 * t * t);
    } else if (l >= 410.0) && (l < 475.0) {
        t = (l - 410.0) / (475.0 - 410.0);
        r = 0.14 - (0.13 * t * t);
    } else if (l >= 545.0) && (l < 595.0) {
        t = (l - 545.0) / (595.0 - 545.0);
        r = (1.98 * t) - (t * t);
    } else if (l >= 595.0) && (l < 650.0) {
        t = (l - 595.0) / (650.0 - 595.0);
        r = 0.98 + (0.06 * t) - (0.40 * t * t);
    } else if (l >= 650.0) && (l < 700.0) {
        t = (l - 650.0) / (700.0 - 650.0);
        r = 0.65 - (0.84 * t) + (0.20 * t * t);
    }
    if (l >= 415.0) && (l < 475.0) {
        t = (l - 415.0) / (475.0 - 415.0);
        g = 0.80 * t * t;
    } else if (l >= 475.0) && (l < 590.0) {
        t = (l - 475.0) / (590.0 - 475.0);
        g = 0.8 + (0.76 * t) - (0.80 * t * t);
    } else if (l >= 585.0) && (l < 639.0) {
        t = (l - 585.0) / (639.0 - 585.0);
        g = 0.84 - (0.84 * t);
    }
    if (l >= 400.0) && (l < 475.0) {
        t = (l - 400.0) / (475.0 - 400.0);
        b = (2.20 * t) - (1.50 * t * t);
    } else if (l >= 475.0) && (l < 560.0) {
        t = (l - 475.0) / (560.0 - 475.0);
        b = 0.7 - (t) + (0.30 * t * t);
    }
    rustbitmap::Rgba::rgb(
        (r * 256.0 * amp) as u8,
        (g * 256.0 * amp) as u8,
        (b * 256.0 * amp) as u8,
    )
}
