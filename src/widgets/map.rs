use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

pub struct MapWidgetMarker {
    pub coordinates: (f32, f32),
    pub icon: String,
    pub text: String,
}

pub struct MapWidget {
    pub background_color: Color,
    pub markers: Vec<MapWidgetMarker>,
    pub center: (f32, f32),
    pub zoom: u8,
}

impl Widget for MapWidget {
    fn html(&self) -> String {
        html!(section #KAPANDJA_MAP_WIDGET {
            div #map{}
        }
        )
        .into_string()
    }

    fn css(&self) -> String {
        format!(
            r#"
#KAPANDJA_MAP_WIDGET #map{{
  height: 50vh;
}}
"#
        )
    }

    fn js(&self) -> String {
        let center = format!("[{}, {}]", self.center.0, self.center.1);
        let zoom = self.zoom;
        let mut markers = "".to_string();
        let mut count = 0;

        for marker in &self.markers {
            let latitude = marker.coordinates.0;
            let longitude = marker.coordinates.1;
            let icon_url = &marker.icon;
            let text = &marker.text;

            let icon = format!(
                r#"
var markerIcon{count} = L.icon({{
  iconUrl: "{icon_url}",
  iconSize: [90, 50]
}});

L.marker([{latitude}, {longitude}], {{icon: markerIcon{count}}}).addTo(map).bindPopup("{text}");
"#
            );

            markers = format!("{markers}{icon}");
            count += 1;
        }

        format!(
            r#"
var map = L.map('map').setView({center}, {zoom});

L.tileLayer('https://tile.openstreetmap.org/{{z}}/{{x}}/{{y}}.png', {{
    maxZoom: 19,
    attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
}}).addTo(map);

{markers}
"#
        )
    }
}
