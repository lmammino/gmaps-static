use std::fmt::{Display, Formatter, Result};

/// Features, or feature types, are geographic characteristics on the map, including roads,
/// parks, bodies of water, businesses, and more.
///
/// The features form a category tree, with all as the root. If you don't specify a feature,
/// all features are selected. Specifying a feature of all has the same effect.
///
/// Some features contain child features you specify using a dot notation. For example,
/// [`Feature::LandscapeNaturalAll`] or [`Feature::RoadLocal`]. If you specify only the parent
/// feature, such as [`Feature::RoadAll`], the styles you specify for the parent apply to all its
/// children, such as [`Feature::RoadLocal`] and [`Feature::RoadHighwayAll`].
///
/// Note that parent features may include some elements that are not included in all of
/// their child features.
#[derive(Clone)]
pub enum Feature {
    /// `all` (default) selects all features.
    All,
    /// `administrative` selects all administrative areas. Styling affects only the labels of administrative areas, not the geographical borders or fill.
    AdminAll,
    /// `administrative.country` selects countries.
    AdminCountry,
    /// `administrative.land_parcel` selects land parcels.
    AdminLandParcel,
    /// `administrative.locality` selects localities.
    AdminLocality,
    /// `administrative.neighborhood` selects neighborhoods.
    AdminNeighborhood,
    /// `administrative.province` selects provinces.
    AdminProvince,
    /// `landscape` selects all landscapes.
    LandscapeAll,
    /// `landscape.man_made` selects man-made features, such as buildings and other structures.
    LandscapeManMade,
    /// `landscape.natural` selects natural features, such as mountains, rivers, deserts, and glaciers.
    LandscapeNaturalAll,
    /// `landscape.natural.landcover` selects land cover features, the physical material that covers the earth's surface, such as forests, grasslands, wetlands, and bare ground.
    LandscapeNaturalLandcover,
    /// `landscape.natural.terrain` selects terrain features of a land surface, such as elevation, slope, and orientation.
    LandscapeNaturalTerrain,
    /// `poi` selects all points of interest.
    PoiAll,
    /// `poi.attraction` selects tourist attractions.
    PoiAttraction,
    /// `poi.business` selects businesses.
    PoiBusiness,
    /// `poi.government` selects government buildings.
    PoiGovernment,
    /// `poi.medical` selects emergency services, including hospitals, pharmacies, police, doctors, and others.
    PoiMedical,
    /// `poi.park` selects parks.
    PoiPark,
    /// `poi.place_of_worship` selects places of worship, including churches, temples, mosques, and others.
    PoiPlaceOfWorship,
    /// `poi.school` selects schools.
    PoiSchool,
    /// `poi.sports_complex` selects sports complexes.
    PoiSportsComplex,
    /// `road` selects all roads.
    RoadAll,
    /// `road.arterial` selects arterial roads.
    RoadArterial,
    /// `road.highway` selects highways.
    RoadHighwayAll,
    /// `road.highway.controlled_access` selects highways with controlled access.
    RoadHighwayControlledAccess,
    /// `road.local` selects local roads.
    RoadLocal,
    /// `transit` selects all transit stations and lines.
    TransitAll,
    /// `transit.line` selects transit lines.
    TransitLine,
    /// `transit.station` selects all transit stations.
    TransitStationAll,
    /// `transit.station.airport` selects airports.
    TransitStationAirport,
    /// `transit.station.bus` selects bus stops.
    TransitStationBus,
    /// `transit.station.rail` selects rail stations.
    TransitStationRail,
    /// `water` selects bodies of water.
    WaterAll,
}

impl Display for Feature {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Feature::*;
        f.write_str(match self {
            All => "all",
            AdminAll => "administrative",
            AdminCountry => "administrative.country",
            AdminLandParcel => "administrative.land_parcel",
            AdminLocality => "administrative.locality",
            AdminNeighborhood => "administrative.neighborhood",
            AdminProvince => "administrative.province",
            LandscapeAll => "landscape",
            LandscapeManMade => "landscape.man_made",
            LandscapeNaturalAll => "landscape.natural",
            LandscapeNaturalLandcover => "landscape.natural.landcover",
            LandscapeNaturalTerrain => "landscape.natural.terrain",
            PoiAll => "poi",
            PoiAttraction => "poi.attraction",
            PoiBusiness => "poi.business",
            PoiGovernment => "poi.government",
            PoiMedical => "poi.medical",
            PoiPark => "poi.park",
            PoiPlaceOfWorship => "poi.place_of_worship",
            PoiSchool => "poi.school",
            PoiSportsComplex => "poi.sports_complex",
            RoadAll => "road",
            RoadArterial => "road.arterial",
            RoadHighwayAll => "road.highway",
            RoadHighwayControlledAccess => "road.highway.controlled_access",
            RoadLocal => "road.local",
            TransitAll => "transit",
            TransitLine => "transit.line",
            TransitStationAll => "transit.station",
            TransitStationAirport => "transit.station.airport",
            TransitStationBus => "transit.station.bus",
            TransitStationRail => "transit.station.rail",
            WaterAll => "water",
        })
    }
}
