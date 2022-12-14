# DHL tracking for business customer with Rust.

![workflow](https://github.com/Thomas-Zenkel/dhl_tracking/actions/workflows/rust.yml/badge.svg)

Sandbox request:    
```rust
use dhl_tracking::SendungsverfolgungBuilder;

fn main() {
    let sv = SendungsverfolgungBuilder::new()
        .sandbox(true)
        .passwd_entwicklerportal("your login-password entwicklerportal".to_string())
        .entwickler_id("EntwicklerID from Konto".to_owned())
        .build()
        .unwrap();
    println!("{:?}", sv.get_piece_detail("00340434161094022115").unwrap());
}    
```
Production request:
```rust
use dhl_tracking::SendungsverfolgungBuilder;

let sendungsverfolgung = SendungsverfolgungBuilder::new()
        .zt_kennung("ztxxxxx".to_owned())
        .passwd_zt_kennung("your password".to_owned())
        .app_token("your token".to_owned())
        .app_id("your app id".to_owned())
        .sandbox(false)
        .build()
        .unwrap();

    let delivery_data = sendungsverfolgung
        .get_piece_detail("00300000000000000000")
        .unwrap();
```

Result of a sandbox query (returns always same example data):

```xml
<?xml version="1.0" encoding="UTF-8"?>
<data name="piece-shipment-list" code="0" request-id="018b31f6-647e-4f7b-b1d6-e2b0d27b3026">
    <data name="piece-shipment" error-status="0" piece-id="8e464a3e-219a-459b-823b-07d9d92732e3" shipment-code="" piece-identifier="340434161094022115" identifier-type="2" piece-code="00340434161094022115" event-location="" event-country="DE" status-liste="0" status-timestamp="18.03.2016 10:02" status="Die Sendung wurde erfolgreich zugestellt." short-status="Zustellung erfolgreich" recipient-name="Kraemer" recipient-street="Heinrich-Brüning-Str. 7" recipient-city="53113 Bonn" pan-recipient-name="Deutsche Post DHL" pan-recipient-street="Heinrich-Brüning-Str. 7" pan-recipient-city="53113 Bonn" pan-recipient-address="Heinrich-Brüning-Str. 7 53113 Bonn" pan-recipient-postalcode="53113" shipper-name="Es wurden keine Absender-Daten an DHL übermittelt." shipper-street="" shipper-city="" shipper-address="" product-code="00" product-key="" product-name="DHL PAKET" delivery-event-flag="1" recipient-id="5" recipient-id-text="andere anwesende Person" upu="" shipment-length="0.0" shipment-width="0.0" shipment-height="0.0" shipment-weight="0.0" international-flag="0" division="DPEED" ice="DLVRD" ric="OTHER" standard-event-code="ZU" dest-country="DE" origin-country="DE" searched-piece-code="00340434161094022115" searched-ref-no="" piece-customer-reference="" shipment-customer-reference="" leitcode="" routing-code-ean="" matchcode="" domestic-id="" airway-bill-number="" ruecksendung="false" pslz-nr="5066934803" order-preferred-delivery-day="false">
        <data name="piece-event-list" piece-identifier="340434161094022115" _build-time="2017-01-14 19:56:44.000509" piece-id="8e464a3e-219a-459b-823b-07d9d92732e3" leitcode="5311304400700" routing-code-ean="" ruecksendung="false" pslz-nr="5066934803" order-preferred-delivery-day="false">
            <data name="piece-event" event-timestamp="17.03.2016 11:43" event-status="Die Sendung wurde vom Absender in die PACKSTATION eingeliefert." event-text="Die Sendung wurde vom Absender in die PACKSTATION eingeliefert." event-short-status="Einlieferung in PACKSTATION" ice="SHRCU" ric="PCKST" event-location="Bremen" event-country="Deutschland" standard-event-code="ES" ruecksendung="false" />
            <data name="piece-event" event-timestamp="17.03.2016 13:53" event-status="Die Sendung wurde zum Weitertransport aus der PACKSTATION entnommen." event-text="Die Sendung wurde zum Weitertransport aus der PACKSTATION entnommen." event-short-status="Transport zum Start-Paketzentrum" ice="LDTMV" ric="MVMTV" event-location="Bremen" event-country="Deutschland" standard-event-code="AA" ruecksendung="false" />
            <data name="piece-event" event-timestamp="17.03.2016 13:55" event-status="Die Sendung wurde abgeholt." event-text="Die Sendung wurde abgeholt." event-short-status="Abholung erfolgreich" ice="PCKDU" ric="PUBCR" event-location="" event-country="Deutschland" standard-event-code="AE" ruecksendung="false" />
            <data name="piece-event" event-timestamp="17.03.2016 15:49" event-status="Die Sendung wurde im Start-Paketzentrum bearbeitet." event-text="Die Sendung wurde im Start-Paketzentrum bearbeitet." event-short-status="Start-Paketzentrum" ice="LDTMV" ric="MVMTV" event-location="Bremen" event-country="Deutschland" standard-event-code="AA" ruecksendung="false" />
            <data name="piece-event" event-timestamp="18.03.2016 03:35" event-status="Die Sendung wurde im Ziel-Paketzentrum bearbeitet." event-text="Die Sendung wurde im Ziel-Paketzentrum bearbeitet." event-short-status="Ziel-Paketzentrum" ice="ULFMV" ric="UNLDD" event-location="Neuwied" event-country="Deutschland" standard-event-code="EE" ruecksendung="false" />
            <data name="piece-event" event-timestamp="18.03.2016 09:00" event-status="Die Sendung wurde in das Zustellfahrzeug geladen." event-text="Die Sendung wurde in das Zustellfahrzeug geladen." event-short-status="In Zustellung" ice="SRTED" ric="NRQRD" event-location="" event-country="" standard-event-code="PO" ruecksendung="false" />
            <data name="piece-event" event-timestamp="18.03.2016 10:02" event-status="Die Sendung wurde erfolgreich zugestellt." event-text="Die Sendung wurde erfolgreich zugestellt." event-short-status="Zustellung erfolgreich" ice="DLVRD" ric="OTHER" event-location="Bonn" event-country="Deutschland" standard-event-code="ZU" ruecksendung="false" />
        </data>
    </data>
</data>
```
