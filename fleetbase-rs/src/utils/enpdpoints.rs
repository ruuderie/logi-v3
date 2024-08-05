use std::fmt;

// Parent enum
#[derive(Debug)]
enum Endpoint {
    Places(Places),
    ServiceAreas(ServiceAreas),
    Zones(Zones),
    Contacts(Contacts),
    Vendors(Vendors),
    Vehicles(Vehicles),
    Drivers(Drivers),
    Orders(Orders),
    Fleets(Fleets),
    Payloads(Payloads),
    Entities(Entities),
    ServiceRates(ServiceRates),
    ServiceQuotes(ServiceQuotes),
    PurchaseRates(PurchaseRates),
    TrackingNumbers(TrackingNumbers),
    TrackingStatuses(TrackingStatuses),
}

// Implement to_string for the parent enum
impl Endpoint {
    pub fn to_string(&self) -> String {
        match self {
            Endpoint::Places(e) => e.to_string(),
            Endpoint::ServiceAreas(e) => e.to_string(),
            Endpoint::Zones(e) => e.to_string(),
            Endpoint::Contacts(e) => e.to_string(),
            Endpoint::Vendors(e) => e.to_string(),
            Endpoint::Vehicles(e) => e.to_string(),
            Endpoint::Drivers(e) => e.to_string(),
            Endpoint::Orders(e) => e.to_string(),
            Endpoint::Fleets(e) => e.to_string(),
            Endpoint::Payloads(e) => e.to_string(),
            Endpoint::Entities(e) => e.to_string(),
            Endpoint::ServiceRates(e) => e.to_string(),
            Endpoint::ServiceQuotes(e) => e.to_string(),
            Endpoint::PurchaseRates(e) => e.to_string(),
            Endpoint::TrackingNumbers(e) => e.to_string(),
            Endpoint::TrackingStatuses(e) => e.to_string(),
        }
    }
}

#[derive(Debug)]
enum Places {
    Places,
    PlacesById(String),
}

impl_to_string!(Places);

#[derive(Debug)]
enum ServiceAreas {
    ServiceAreas,
    ServiceAreasById(String),
}

impl_to_string!(ServiceAreas);

#[derive(Debug)]
enum Zones {
    Zones,
    ZonesById(String),
}

impl_to_string!(Zones);

#[derive(Debug)]
enum Contacts {
    Contacts,
    ContactsById(String),
}

impl_to_string!(Contacts);

#[derive(Debug)]
enum Vendors {
    Vendors,
    VendorsById(String),
}

impl_to_string!(Vendors);

#[derive(Debug)]
enum Vehicles {
    Vehicles,
    VehiclesById(String),
}

impl_to_string!(Vehicles);

#[derive(Debug)]
enum Drivers {
    Drivers,
    DriversById(String),
}

impl_to_string!(Drivers);

#[derive(Debug)]
enum Orders {
    Orders,
    OrdersById(String),
    OrdersSchedule(String),
    OrdersDispatch(String),
    OrdersStart(String),
    OrdersUpdateActivity(String),
    OrdersNextActivity(String),
    OrdersSetDestination(String, String),
    OrdersCaptureSignature(String, String),
    OrdersCaptureQr(String, String),
    OrdersOrderV(String),
}

impl_to_string!(Orders);

#[derive(Debug)]
enum Fleets {
    Fleets,
    FleetsById(String),
}

impl_to_string!(Fleets);

#[derive(Debug)]
enum Payloads {
    Payloads,
    PayloadsById(String),
}

impl_to_string!(Payloads);

#[derive(Debug)]
enum Entities {
    Entities,
    EntitiesById(String),
}

impl_to_string!(Entities);

#[derive(Debug)]
enum ServiceRates {
    ServiceRates,
    ServiceRatesById(String),
}

impl_to_string!(ServiceRates);

#[derive(Debug)]
enum ServiceQuotes {
    ServiceQuotes,
}

impl_to_string!(ServiceQuotes);

#[derive(Debug)]
enum PurchaseRates {
    PurchaseRates,
    PurchaseRatesById(String),
}

impl_to_string!(PurchaseRates);

#[derive(Debug)]
enum TrackingNumbers {
    TrackingNumbers,
    TrackingNumbersById(String),
}

impl_to_string!(TrackingNumbers);

#[derive(Debug)]
enum TrackingStatuses {
    TrackingStatuses,
    TrackingStatusesById(String),
}

impl_to_string!(TrackingStatuses);
macro_rules! impl_to_string {
    ($enum_name:ident) => {
        impl $enum_name {
            pub fn to_string(&self) -> String {
                match self {
                    $(Self::$variant $(($($param:ident),*))? => {
                        let base = stringify!($variant).to_lowercase();
                        $(format!("{}/{}", base, $($param),*))
                        $($(.replace("_", "-")))?
                    },)*
                }
            }
        }
    };
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_to_string() {
        let endpoints = vec![
            (Endpoint::Places(Places::Places), "places"),
            (
                Endpoint::Places(Places::PlacesById("123".to_string())),
                "places/123",
            ),
            (
                Endpoint::ServiceAreas(ServiceAreas::ServiceAreas),
                "service-areas",
            ),
            (
                Endpoint::ServiceAreas(ServiceAreas::ServiceAreasById("456".to_string())),
                "service-areas/456",
            ),
            (Endpoint::Orders(Orders::Orders), "orders"),
            (
                Endpoint::Orders(Orders::OrdersUpdateActivity("789".to_string())),
                "orders/update-activity/789",
            ),
            (
                Endpoint::Orders(Orders::OrdersSetDestination(
                    "789".to_string(),
                    "waypoint1".to_string(),
                )),
                "orders/set-destination/789/waypoint1",
            ),
        ];

        for (endpoint, expected) in endpoints {
            assert_eq!(endpoint.to_string(), expected, "Failed for {:?}", endpoint);
        }
    }
}
