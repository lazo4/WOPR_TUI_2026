use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Points},
        Widget,
    },
};

use crate::game::types::{CommMessage, CommPriority, Country, ThreatLevel};
use crate::ui::threat_overlay::{MissileTrajectory, ThreatMarker};

pub struct Location {
    pub name: &'static str,
    pub lat: f64,
    pub lon: f64,
}

pub const LOCATIONS: &[Location] = &[
    Location { name: "Washington", lat: 38.9, lon: -77.0 },
    Location { name: "Moscow",     lat: 55.8, lon:  37.6 },
    Location { name: "Beijing",    lat: 39.9, lon: 116.4 },
    Location { name: "London",     lat: 51.5, lon:  -0.1 },
    Location { name: "Paris",      lat: 48.9, lon:   2.3 },
    Location { name: "New Delhi",  lat: 28.6, lon:  77.2 },
    Location { name: "Pyongyang",  lat: 39.0, lon: 125.7 },
    Location { name: "Tehran",     lat: 35.7, lon:  51.4 },
    Location { name: "Islamabad",  lat: 33.7, lon:  73.0 },
    Location { name: "Tokyo",      lat: 35.7, lon: 139.7 },
    Location { name: "Ankara",     lat: 39.9, lon:  32.9 },
];

fn country_capital(c: &Country) -> (f64, f64) {
    match c {
        Country::USA           => (38.9, -77.0),
        Country::Russia        => (55.8,  37.6),
        Country::China         => (39.9, 116.4),
        Country::UnitedKingdom => (51.5,  -0.1),
        Country::France        => (48.9,   2.3),
        Country::India         => (28.6,  77.2),
        Country::Pakistan      => (33.7,  73.0),
        Country::DPRK          => (39.0, 125.7),
        Country::Iran          => (35.7,  51.4),
        Country::NATO          => (50.8,   4.4), // Brussels
        Country::Unknown       => (47.0,   8.0), // Geneva
    }
}

// (lat, lon) polylines — dense enough for recognizable shapes at terminal resolution
const MAP_LINES: &[&[(f64, f64)]] = &[
    // ── North America: East coast + Gulf + Central America ──
    &[
        (61.0, -140.0), (64.0, -142.0), (66.0, -145.0), (68.0, -150.0),
        (70.5, -157.0), (71.0, -155.0), (71.3, -150.0), (70.8, -145.0),
        (70.0, -141.0), (69.5, -139.0),
        // Arctic Canada
        (70.0, -130.0), (72.0, -118.0), (74.0, -95.0), (73.0, -85.0),
        // Hudson Bay east
        (63.0, -78.0), (60.0, -78.0), (58.0, -76.5), (56.0, -80.0),
        (55.0, -82.0), (53.0, -82.0), (52.0, -80.0),
        // Hudson Bay west
        (57.0, -88.0), (59.0, -92.0), (61.0, -94.0), (63.0, -92.0),
        (66.0, -86.0), (68.0, -75.0),
        // Labrador
        (62.0, -66.0), (58.0, -62.0), (54.0, -57.0), (52.0, -56.0),
        // Newfoundland
        (50.0, -57.0), (47.5, -59.0), (46.5, -53.0), (47.5, -53.0),
        (49.5, -56.5),
        // Maritimes + New England
        (47.0, -61.0), (46.0, -64.0), (44.0, -66.0), (43.5, -70.0),
        (42.0, -71.0), (41.5, -70.5), (41.2, -72.0),
        // Mid-Atlantic + Southeast
        (40.0, -74.0), (38.5, -75.0), (37.0, -76.0), (35.5, -75.5),
        (34.0, -77.5), (33.0, -79.0), (32.0, -81.0), (31.0, -81.0),
        (30.0, -81.5),
        // Florida
        (28.0, -80.5), (27.0, -80.0), (25.5, -80.2), (25.0, -81.0),
        (26.0, -82.0), (27.5, -82.5), (28.5, -82.8), (29.5, -83.5),
        (30.0, -84.5), (29.5, -85.5),
        // Gulf coast
        (30.2, -87.0), (30.0, -88.5), (29.5, -89.5), (29.0, -90.0),
        (29.2, -91.5), (29.5, -93.0), (29.0, -95.0), (28.0, -96.5),
        (27.0, -97.0), (26.0, -97.2),
        // Mexico Gulf + Pacific
        (23.0, -97.5), (21.5, -97.0), (20.0, -96.5), (19.0, -96.0),
        (18.0, -95.0), (16.5, -94.0), (16.0, -92.0), (15.5, -90.0),
        (14.0, -88.0), (13.0, -87.0), (12.0, -86.0), (11.0, -84.0),
        (10.0, -83.0), (9.0, -80.0), (8.5, -78.0), (8.0, -77.0),
    ],
    // ── North America: West coast ──
    &[
        (8.0, -77.0),
        (8.0, -79.0), (9.5, -84.5), (12.0, -87.0), (14.0, -90.0),
        (16.0, -96.0), (18.5, -103.0), (20.0, -105.0), (22.0, -106.0),
        (24.0, -107.5), (24.5, -110.0),
        // Baja tip
        (23.0, -110.0), (23.5, -109.5),
        // mainland
        (27.0, -110.5), (29.0, -112.0), (31.0, -113.5),
        (32.5, -117.0), (33.5, -118.0), (34.5, -120.5), (36.0, -122.0),
        (37.5, -122.5), (38.5, -123.0), (40.0, -124.0), (42.0, -124.5),
        (44.0, -124.0), (46.5, -124.0), (48.5, -124.5),
        // BC + Alaska panhandle
        (49.0, -125.5), (51.0, -128.0), (54.0, -130.0), (56.0, -132.0),
        (57.5, -136.0), (59.0, -139.0), (60.0, -141.0), (61.0, -150.0),
        (60.0, -152.0), (58.5, -155.0), (57.0, -157.0), (56.0, -160.0),
        (55.0, -163.0), (54.5, -165.0), (56.0, -167.0), (58.0, -168.0),
        (60.0, -167.0), (62.0, -164.0), (64.0, -163.0), (65.0, -168.0),
        (66.0, -164.0), (66.5, -162.0), (64.5, -158.0),
        (61.0, -140.0),
    ],
    // ── South America ──
    &[
        (12.0, -72.0), (11.0, -75.0), (12.5, -71.5), (11.0, -68.0),
        (10.5, -67.0), (10.5, -62.0), (8.5, -60.0), (7.0, -57.0),
        (6.0, -55.0), (5.0, -52.0), (3.0, -51.0), (1.5, -49.0),
        (0.0, -49.5), (-1.0, -48.0), (-2.5, -44.0), (-5.0, -36.0),
        (-7.5, -35.0), (-10.0, -37.0), (-13.0, -38.5), (-15.0, -39.0),
        (-18.0, -40.0), (-20.0, -41.0), (-22.5, -42.0), (-23.5, -44.0),
        (-25.0, -48.0), (-27.0, -48.5), (-29.0, -50.0), (-32.0, -52.0),
        (-34.0, -54.0), (-35.0, -57.0), (-37.0, -57.0), (-38.5, -58.0),
        (-40.0, -62.0), (-42.0, -64.0), (-44.0, -65.5), (-46.5, -67.0),
        (-48.0, -66.0), (-50.0, -69.0), (-52.0, -70.0), (-53.5, -71.0),
        (-55.0, -68.0), (-55.5, -66.0), (-54.0, -64.5), (-52.0, -69.0),
        // west coast going north
        (-48.0, -75.5), (-44.0, -73.0), (-40.0, -73.5), (-37.0, -73.5),
        (-33.0, -72.0), (-30.0, -71.5), (-27.0, -71.0), (-23.0, -70.5),
        (-18.0, -71.0), (-16.0, -75.0), (-14.0, -76.5), (-12.0, -77.0),
        (-8.0, -79.5), (-5.0, -81.0), (-3.0, -80.0), (-1.0, -80.0),
        (0.0, -80.0), (1.0, -79.0), (2.0, -77.5), (4.0, -77.0),
        (7.0, -77.5), (8.0, -77.0),
    ],
    // ── Europe mainland ──
    &[
        // Iberian peninsula
        (36.0, -6.0), (37.0, -9.0), (38.5, -9.5), (39.5, -9.0),
        (41.5, -9.0), (43.0, -9.5), (43.5, -8.0), (43.5, -3.5),
        (43.0, -1.5),
        // France coast
        (46.0, -1.5), (47.5, -3.0), (48.5, -5.0), (48.8, -3.5),
        (49.5, -1.0), (50.0, 1.5), (51.0, 2.0),
        // Low Countries + Germany
        (51.5, 3.5), (53.0, 5.0), (54.5, 8.0), (55.0, 9.0),
        // Denmark
        (56.0, 8.5), (57.5, 10.5), (56.0, 12.0),
        // Scandinavia
        (56.0, 13.0), (58.0, 12.0), (59.0, 11.0), (62.0, 5.0),
        (64.0, 11.0), (67.0, 15.0), (69.0, 16.0), (70.5, 22.0),
        (71.0, 26.0), (70.0, 28.0), (69.5, 31.0),
        // Finland / Russia border to Black Sea
        (68.0, 30.0), (64.0, 28.0), (61.0, 24.0), (60.0, 25.0),
        (59.5, 28.0), (58.0, 28.0), (56.0, 21.0), (55.0, 20.0),
        (54.5, 18.0), (54.0, 19.0), (53.5, 14.5),
        // Poland → Balkans
        (51.0, 14.0), (47.5, 18.0), (45.0, 20.0), (44.5, 22.0),
        (43.0, 28.0), (42.0, 28.0), (41.0, 29.0),
        // Turkey Straits → Greece
        (41.0, 28.0), (40.0, 24.0), (38.5, 23.0), (37.5, 24.0),
        (36.5, 22.5), (38.0, 21.0), (39.5, 20.0), (42.0, 19.0),
        (43.0, 17.0),
        // Italian + Mediterranean
        (45.5, 13.5), (44.0, 12.5), (42.5, 11.5), (41.0, 13.0),
        (40.5, 15.0), (40.0, 16.5), (38.0, 16.0), (38.0, 15.5),
        (37.5, 13.0), (38.5, 12.5), (40.5, 9.0), (41.0, 8.5),
        (43.5, 4.0), (42.5, 3.0),
        // Mediterranean coast → Strait of Gibraltar
        (39.5, 0.0), (37.5, -1.0), (36.5, -2.5), (36.0, -5.5), (36.0, -6.0),
    ],
    // ── British Isles ──
    &[
        (50.0, -5.5), (51.0, -3.0), (51.5, 1.0), (52.5, 1.5),
        (53.0, 0.0), (54.0, -0.5), (55.0, -1.5), (56.0, -3.0),
        (57.5, -5.0), (58.5, -5.0), (58.5, -3.0), (57.0, -2.0),
        (56.0, -2.5), (55.5, -4.5), (54.5, -5.0), (54.0, -3.0),
        (53.5, -3.0), (52.0, -4.5), (51.5, -5.0), (50.0, -5.5),
    ],
    // ── Ireland ──
    &[
        (51.5, -10.0), (52.0, -7.0), (53.5, -6.0), (55.0, -6.0),
        (55.5, -8.0), (54.0, -10.0), (52.5, -10.5), (51.5, -10.0),
    ],
    // ── Africa ──
    &[
        // North coast
        (36.0, -6.0), (35.0, -2.0), (37.0, 3.0), (37.0, 7.0),
        (37.5, 10.0), (33.0, 12.0), (32.5, 15.0), (32.0, 20.0),
        (31.5, 25.0), (31.0, 30.0), (31.5, 32.0),
        // Suez → Horn of Africa
        (30.0, 33.0), (28.0, 34.0), (22.0, 36.0), (18.0, 38.0),
        (15.0, 42.0), (13.0, 43.0), (12.0, 44.0), (11.5, 43.5),
        (12.0, 45.0), (11.0, 49.0), (10.0, 51.0), (6.0, 49.0),
        (2.0, 45.0), (0.0, 42.5),
        // East coast
        (-2.0, 41.0), (-5.0, 39.5), (-8.0, 39.5), (-10.0, 40.0),
        (-15.0, 40.5), (-18.0, 37.0), (-23.0, 35.5), (-26.0, 33.0),
        (-30.0, 31.0), (-33.0, 28.0), (-34.5, 26.0),
        // Southern tip
        (-34.0, 18.5), (-33.0, 17.5), (-32.0, 18.0), (-30.0, 17.0),
        // West coast
        (-28.0, 15.0), (-22.0, 14.0), (-17.0, 11.5), (-12.0, 13.5),
        (-6.0, 12.0), (-4.5, 11.5), (-1.0, 9.5), (4.0, 7.0), (5.0, 4.0),
        (6.0, 2.5), (6.5, 1.0), (5.0, -2.5), (5.0, -5.0),
        (7.5, -8.0), (8.0, -13.0), (11.0, -15.0), (13.0, -16.5),
        (15.0, -17.0), (16.0, -16.5), (19.0, -17.0), (21.0, -17.0),
        (24.0, -16.0), (26.0, -14.5), (28.0, -12.0),
        (30.0, -10.0), (32.0, -9.0), (34.0, -7.0), (35.5, -6.0), (36.0, -6.0),
    ],
    // ── Asia: Middle East + Central + East ──
    &[
        // Turkey east → Iran → Pakistan
        (41.0, 29.0), (42.0, 35.0), (41.5, 42.0), (40.0, 44.0),
        (38.5, 48.0), (37.0, 54.0), (35.0, 54.0), (33.0, 57.0),
        (30.0, 60.0), (27.0, 62.0), (25.5, 61.5), (25.0, 63.0),
        (24.5, 66.0), (25.0, 68.0),
        // Indian subcontinent
        (24.0, 69.0), (22.0, 69.0), (20.5, 72.5), (19.0, 73.0),
        (16.0, 73.5), (15.0, 74.0), (11.5, 76.0), (8.0, 77.5),
        (7.5, 78.0), (10.0, 80.0), (13.0, 80.5), (16.0, 82.0),
        (19.0, 85.0), (21.0, 87.0), (22.0, 89.0), (22.5, 91.0),
        // Bangladesh → Myanmar → SE Asia
        (21.0, 92.0), (18.0, 95.0), (16.0, 97.5), (13.0, 99.0),
        (10.0, 99.0), (7.0, 100.0), (3.0, 101.0), (1.5, 103.5),
        (1.0, 104.0),
        // Up through Vietnam → China coast
        (5.0, 108.0), (10.0, 106.0), (12.0, 109.0), (16.0, 108.5),
        (18.5, 106.0), (21.0, 107.0), (23.0, 108.5), (22.0, 111.0),
        (24.0, 118.0), (25.5, 120.0),
        (28.0, 121.0), (30.0, 122.0), (32.0, 122.0), (34.0, 120.0),
        (35.5, 120.0), (37.0, 122.5), (38.0, 121.0),
        // Korea
        (39.0, 125.0), (40.0, 124.5), (42.0, 130.0), (43.0, 131.5),
        // Russian Pacific coast
        (46.0, 135.0), (48.0, 135.0), (50.0, 140.0), (52.0, 141.0),
        (54.0, 143.0), (56.0, 140.0), (58.0, 150.0), (60.0, 157.0),
        (62.0, 163.0), (64.0, 170.0), (66.0, 180.0),
    ],
    // ── Asia: Russian Arctic ──
    &[
        (66.0, 180.0), (69.0, 179.0), (70.0, 170.0), (71.0, 160.0),
        (72.0, 140.0), (73.5, 130.0), (75.0, 110.0), (73.0, 80.0),
        (71.0, 70.0), (68.5, 55.0), (66.0, 50.0), (62.0, 48.0),
        (60.0, 45.0), (58.0, 40.0),
    ],
    // ── Arabian Peninsula ──
    &[
        (30.0, 33.0), (29.5, 35.0), (28.0, 35.0), (25.0, 37.0),
        (20.0, 39.0), (16.0, 42.5), (13.0, 43.5), (12.5, 44.0),
        (13.0, 48.0), (16.0, 52.0), (22.0, 59.0), (24.0, 57.0),
        (25.5, 56.0), (26.0, 51.5), (25.5, 50.5), (27.0, 49.5),
        (29.5, 48.0), (30.0, 48.5), (31.0, 47.5), (33.0, 44.0),
        (35.5, 36.0), (31.5, 32.0),
    ],
    // ── Japan ──
    &[
        (31.0, 131.0), (33.0, 130.0), (34.0, 131.0), (34.0, 133.0),
        (35.5, 134.0), (36.0, 136.5), (37.0, 137.0), (38.0, 139.0),
        (39.5, 140.0), (41.0, 140.0), (43.0, 145.0), (44.0, 145.0),
        (43.5, 143.0), (42.0, 140.5), (40.0, 139.5), (38.5, 137.0),
        (36.0, 136.0), (35.0, 132.0), (33.5, 131.0), (31.0, 131.0),
    ],
    // ── Australia ──
    &[
        (-12.0, 136.0), (-12.0, 132.0), (-14.0, 129.0), (-14.5, 126.0),
        (-18.0, 122.0), (-20.0, 119.0), (-22.0, 114.5), (-25.0, 113.5),
        (-28.0, 114.0), (-31.0, 115.5), (-34.0, 116.0), (-35.0, 117.5),
        (-35.5, 120.0), (-34.5, 123.0), (-33.0, 128.0), (-35.0, 136.0),
        (-36.5, 137.0), (-38.5, 141.0), (-39.0, 146.0), (-38.0, 148.0),
        (-36.0, 150.0), (-34.0, 151.5), (-31.0, 153.0), (-28.0, 153.5),
        (-25.0, 152.5), (-22.0, 150.0), (-19.5, 147.5), (-17.0, 146.0),
        (-16.0, 145.5), (-14.5, 144.0), (-12.0, 142.0), (-14.0, 141.0),
        (-15.0, 140.0), (-14.5, 137.0), (-12.0, 136.0),
    ],
    // ── New Zealand ──
    &[
        (-35.0, 174.0), (-37.0, 176.0), (-39.0, 177.0), (-41.0, 175.0),
        (-42.0, 172.0), (-44.0, 169.0), (-46.5, 168.0), (-46.0, 166.5),
        (-44.0, 168.5), (-42.5, 172.0), (-41.0, 174.0), (-39.0, 174.0),
        (-37.0, 175.0), (-35.0, 174.0),
    ],
    // ── Greenland ──
    &[
        (60.0, -43.0), (62.0, -42.0), (66.0, -37.0), (70.0, -22.0),
        (72.0, -18.0), (76.0, -18.0), (78.0, -20.0), (80.0, -22.0),
        (82.0, -30.0), (83.0, -40.0), (82.0, -50.0), (80.0, -56.0),
        (78.0, -62.0), (76.0, -68.0), (73.0, -57.0), (70.0, -52.0),
        (66.0, -50.0), (62.0, -48.0), (60.0, -43.0),
    ],
    // ── Sri Lanka ──
    &[
        (9.5, 80.0), (8.0, 80.0), (6.5, 80.5), (6.0, 81.0),
        (7.5, 82.0), (9.5, 80.0),
    ],
    // ── Indonesia (Sumatra + Java, simplified) ──
    &[
        (5.5, 95.0), (3.0, 99.0), (1.0, 102.0), (-1.0, 104.0),
        (-3.0, 106.0), (-6.0, 106.0), (-7.0, 107.0), (-8.0, 110.0),
        (-7.5, 112.0), (-8.0, 114.5), (-8.5, 116.0),
    ],
    // ── Borneo ──
    &[
        (7.0, 117.0), (6.0, 116.0), (4.0, 115.0), (2.0, 110.0),
        (1.0, 109.5), (-1.0, 110.0), (-3.0, 111.0), (-3.5, 114.0),
        (-2.0, 117.0), (0.0, 118.0), (2.0, 118.0), (4.0, 118.0),
        (7.0, 117.0),
    ],
    // ── Philippines (simplified) ──
    &[
        (18.5, 121.0), (16.0, 120.0), (14.0, 121.0), (12.0, 124.0),
        (10.0, 126.0), (7.0, 126.0), (6.0, 125.0),
    ],
    // ── Iceland ──
    &[
        (64.0, -22.0), (65.5, -18.0), (66.5, -16.0), (66.0, -14.0),
        (64.5, -14.0), (63.5, -18.0), (63.5, -21.0), (64.0, -22.0),
    ],
    // ── Madagascar ──
    &[
        (-12.5, 49.0), (-15.0, 50.0), (-18.0, 49.5), (-22.0, 48.0),
        (-25.0, 47.0), (-25.5, 44.5), (-22.0, 44.0), (-18.0, 44.0),
        (-15.0, 46.0), (-12.5, 49.0),
    ],
    // ── Korea peninsula (south) ──
    &[
        (38.5, 128.5), (36.0, 129.5), (35.0, 129.0), (34.0, 127.0),
        (34.5, 126.5), (36.0, 126.5), (37.5, 126.0), (38.5, 126.5),
    ],
];

pub fn latlon_to_cell(lat: f32, lon: f32, area: Rect) -> (u16, u16) {
    let x_frac = (lon + 180.0) / 360.0;
    let y_frac = (90.0 - lat) / 180.0;
    let x = area.x + (x_frac * area.width as f32).clamp(0.0, (area.width - 1) as f32) as u16;
    let y = area.y + (y_frac * area.height as f32).clamp(0.0, (area.height - 1) as f32) as u16;
    (x, y)
}

fn threat_color(severity: ThreatLevel) -> Color {
    match severity {
        ThreatLevel::Low => Color::Cyan,
        ThreatLevel::Medium => Color::Yellow,
        ThreatLevel::High => Color::Red,
        ThreatLevel::Critical => Color::LightRed,
    }
}

fn priority_color(p: &CommPriority) -> Color {
    match p {
        CommPriority::Flash => Color::Red,
        CommPriority::Immediate => Color::Yellow,
        CommPriority::Priority => Color::Green,
        CommPriority::Routine => Color::DarkGray,
    }
}

pub struct WorldMap<'a> {
    pub missiles: &'a [MissileTrajectory],
    pub threats: &'a [ThreatMarker],
    pub comms: &'a [CommMessage],
    pub player_country: Option<Country>,
    pub tick: u64,
}

impl Widget for WorldMap<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let blink = (self.tick / 15) % 2 == 0;
        let player_cap = self.player_country.as_ref().map(country_capital);

        Canvas::default()
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
            .paint(|ctx| {
                // ── graticule (faint grid) ──
                for lat in [-60.0, -30.0, 0.0, 30.0, 60.0] {
                    ctx.draw(&CanvasLine {
                        x1: -180.0, y1: lat, x2: 180.0, y2: lat,
                        color: Color::Indexed(236), // very dark gray
                    });
                }
                for lon in [-120.0, -60.0, 0.0, 60.0, 120.0] {
                    ctx.draw(&CanvasLine {
                        x1: lon, y1: -70.0, x2: lon, y2: 80.0,
                        color: Color::Indexed(236),
                    });
                }

                // ── continent outlines ──
                for polyline in MAP_LINES {
                    for pair in polyline.windows(2) {
                        ctx.draw(&CanvasLine {
                            x1: pair[0].1, y1: pair[0].0,
                            x2: pair[1].1, y2: pair[1].0,
                            color: Color::Green,
                        });
                    }
                }

                // ── comm lines with animated beads ──
                if let Some((plat, plon)) = player_cap {
                    let seen = self.recent_comm_origins();
                    for (country, priority) in &seen {
                        let (clat, clon) = country_capital(country);
                        if (clat - plat).abs() < 0.5 && (clon - plon).abs() < 0.5 {
                            continue; // skip self
                        }
                        let color = priority_color(priority);

                        // dashed line
                        let steps = 20;
                        for i in 0..steps {
                            if i % 3 == 0 { continue; } // gap every 3rd segment
                            let t0 = i as f64 / steps as f64;
                            let t1 = (i + 1) as f64 / steps as f64;
                            ctx.draw(&CanvasLine {
                                x1: plon + (clon - plon) * t0,
                                y1: plat + (clat - plat) * t0,
                                x2: plon + (clon - plon) * t1,
                                y2: plat + (clat - plat) * t1,
                                color: Color::Indexed(238), // dim dashes
                            });
                        }

                        // animated bead traveling along the line
                        let speed = match priority {
                            CommPriority::Flash => 40,
                            CommPriority::Immediate => 60,
                            CommPriority::Priority => 80,
                            _ => 120,
                        };
                        let t = ((self.tick % speed) as f64) / speed as f64;
                        let bx = plon + (clon - plon) * t;
                        let by = plat + (clat - plat) * t;
                        ctx.draw(&Points { coords: &[(bx, by)], color });

                        // second bead offset for Flash priority
                        if matches!(priority, CommPriority::Flash) {
                            let t2 = ((self.tick % speed) as f64 + (speed as f64 * 0.5)) / speed as f64;
                            let t2 = t2 % 1.0;
                            ctx.draw(&Points {
                                coords: &[(plon + (clon - plon) * t2, plat + (clat - plat) * t2)],
                                color: Color::LightRed,
                            });
                        }
                    }
                }

                // ── city markers (crosshair style) ──
                for loc in LOCATIONS {
                    let s = 1.5;
                    ctx.draw(&CanvasLine {
                        x1: loc.lon - s, y1: loc.lat, x2: loc.lon + s, y2: loc.lat,
                        color: Color::LightGreen,
                    });
                    ctx.draw(&CanvasLine {
                        x1: loc.lon, y1: loc.lat - s * 0.8, x2: loc.lon, y2: loc.lat + s * 0.8,
                        color: Color::LightGreen,
                    });
                    ctx.print(
                        loc.lon + 3.0, loc.lat,
                        Span::styled(loc.name, Style::default().fg(Color::DarkGray)),
                    );
                }

                // ── missile trajectories ──
                for m in self.missiles {
                    let p = m.progress.clamp(0.0, 1.0) as f64;
                    let (oy, ox) = (m.origin.0 as f64, m.origin.1 as f64);
                    let (ty, tx) = (m.target.0 as f64, m.target.1 as f64);

                    // trail line
                    ctx.draw(&CanvasLine { x1: ox, y1: oy, x2: tx, y2: ty, color: Color::DarkGray });

                    // moving warhead
                    let cx = ox + (tx - ox) * p;
                    let cy = oy + (ty - oy) * p;
                    let dot_color = if p >= 1.0 { Color::LightRed } else { Color::Red };
                    ctx.draw(&Points { coords: &[(cx, cy)], color: dot_color });

                    // exhaust trail
                    for trail in 1..=4 {
                        let tp = (p - trail as f64 * 0.03).max(0.0);
                        let trail_x = ox + (tx - ox) * tp;
                        let trail_y = oy + (ty - oy) * tp;
                        ctx.draw(&Points {
                            coords: &[(trail_x, trail_y)],
                            color: Color::Indexed(240),
                        });
                    }
                }

                // ── threat markers ──
                for t in self.threats {
                    let visible = match t.severity {
                        ThreatLevel::Critical | ThreatLevel::High => blink,
                        _ => true,
                    };
                    if visible {
                        let color = threat_color(t.severity);
                        ctx.draw(&Points {
                            coords: &[(t.location.1 as f64, t.location.0 as f64)],
                            color,
                        });
                    }
                }
            })
            .render(area, buf);
    }
}

fn priority_rank(p: CommPriority) -> u8 {
    match p {
        CommPriority::Routine => 0,
        CommPriority::Priority => 1,
        CommPriority::Immediate => 2,
        CommPriority::Flash => 3,
    }
}

impl<'a> WorldMap<'a> {
    fn recent_comm_origins(&self) -> Vec<(Country, CommPriority)> {
        let mut seen = std::collections::HashMap::new();
        for comm in self.comms.iter().rev().take(20) {
            seen.entry(comm.origin)
                .and_modify(|existing: &mut CommPriority| {
                    if priority_rank(comm.priority) > priority_rank(*existing) {
                        *existing = comm.priority;
                    }
                })
                .or_insert(comm.priority);
        }
        seen.into_iter().collect()
    }
}
