use asterix_deku::{
    AsterixMessage, AsterixPacket, MBData, AIC, ARC, CDM, CNF, COM, DOU, FX, G, GHO, L, MAH,
    MSSC, RAB, RAD, RDP, SI, SIM, SPI, STAT, SUP, TCC, TRE, TYP, V,
};
use deku::DekuContainerRead;
use deku::DekuContainerWrite;

#[test]
fn it_works() {
    let bytes = vec![
        0x30, 0x00, 0x30, 0xfd, 0xf7, 0x02, 0x19, 0xc9, 0x35, 0x6d, 0x4d, 0xa0, 0xc5, 0xaf, 0xf1,
        0xe0, 0x02, 0x00, 0x05, 0x28, 0x3c, 0x66, 0x0c, 0x10, 0xc2, 0x36, 0xd4, 0x18,
        //0x20 in wireshark, but last 6 bits don't matter and will 0x00 by writer
        0x00, 0x01, 0xc0, 0x78, 0x00, 0x31, 0xbc, 0x00, 0x00, 0x40, 0x0d, 0xeb, 0x07, 0xb9, 0x58,
        0x2e, 0x41, 0x00, 0x20, 0xf5,
    ];
    let (_, packet) = AsterixPacket::from_bytes((&bytes, 0)).unwrap();

    assert_eq!(packet.category, 48);
    assert_eq!(packet.length, 48);

    // TODO check NONE values after more are implemented
    if let AsterixMessage::Cat48(ref message) = packet.message {
        assert_eq!(message.fspec, &[0xfd, 0xf7, 0x02]);

        let data_source_identifier = message.data_source_identifier.as_ref().unwrap();
        assert_eq!(data_source_identifier.sac, 25);
        assert_eq!(data_source_identifier.sic, 201);

        let time_of_day = message.time_of_day.as_ref().unwrap();
        assert_eq!(time_of_day.time, 27354.602);

        let target_report_descriptor = message.target_report_descriptor.as_ref().unwrap();
        assert_eq!(target_report_descriptor.typ, TYP::SingleModeSRollCall);
        assert_eq!(target_report_descriptor.sim, SIM::ActualTargetReport);
        assert_eq!(target_report_descriptor.rdp, RDP::ReportFromRDPChain1);
        assert_eq!(target_report_descriptor.spi, SPI::AbsenceOfSPI);
        assert_eq!(
            target_report_descriptor.rab,
            RAB::ReportFromAircraftTransponder
        );
        assert_eq!(target_report_descriptor.fx, FX::EndOfDataItem);

        let measured_position_in_polar_coordinates = message
            .measured_position_in_polar_coordinates
            .as_ref()
            .unwrap();
        assert_eq!(measured_position_in_polar_coordinates.rho, 197.6836);
        assert_eq!(measured_position_in_polar_coordinates.theta, 340.13672);

        let mode_3_a_code_in_octal_representation = message
            .mode_3_a_code_in_octal_representation
            .as_ref()
            .unwrap();
        assert_eq!(mode_3_a_code_in_octal_representation.v, V::CodeValidated);
        assert_eq!(mode_3_a_code_in_octal_representation.g, G::Default);
        assert_eq!(
            mode_3_a_code_in_octal_representation.l,
            L::Mode3CodeDerivedFromTheReplyOfTheTransponder
        );

        let flight_level_in_binary_repre = message.flight_level_in_binary_repre.as_ref().unwrap();
        assert_eq!(flight_level_in_binary_repre.v, V::CodeValidated);
        assert_eq!(flight_level_in_binary_repre.g, G::Default);
        assert_eq!(flight_level_in_binary_repre.flight_level, 10260);

        let aircraft_address = message.aircraft_address.as_ref().unwrap();
        assert_eq!(aircraft_address.address, 812_604);

        let aircraft_identification = message.aircraft_identification.as_ref().unwrap();
        assert_eq!(aircraft_identification.identification, "DLH65A ");

        let mode_smb_data = message.mode_smb_data.as_ref().unwrap();
        assert_eq!(mode_smb_data.count, 1);
        assert_eq!(
            mode_smb_data.mb_data,
            vec![MBData {
                data: [0xc0, 0x78, 0x00, 0x31, 0xbc, 0x00, 0x00].to_vec()
            }]
        );

        let track_number = message.track_number.as_ref().unwrap();
        assert_eq!(track_number.number, 3563);

        let calculated_track_velocity = message.calculated_track_velocity.as_ref().unwrap();
        assert_eq!(calculated_track_velocity.groundspeed, 0.120_666_504);
        assert_eq!(calculated_track_velocity.heading, 124.002_686);

        let track_status = message.track_status.as_ref().unwrap();
        assert_eq!(track_status.cnf, CNF::ConfirmedTrack);
        assert_eq!(track_status.rad, RAD::SSRModeSTrack);
        assert_eq!(track_status.dou, DOU::NormalConfidence);
        assert_eq!(track_status.mah, MAH::NoHorizontalManSensed);
        assert_eq!(track_status.cdm, CDM::Maintaining);
        assert_eq!(track_status.fx1, FX::ExtensionIntoFirstExtent);
        assert_eq!(track_status.tre, TRE::TrackStillAlive);
        assert_eq!(track_status.gho, GHO::TrueTargetTrack);
        assert_eq!(track_status.sup, SUP::No);
        assert_eq!(track_status.tcc, TCC::RadarPlanePlotTransformation);
        assert_eq!(track_status.fx2, FX::EndOfDataItem);

        let communications_capability_flight_status = message
            .communications_capability_flight_status
            .as_ref()
            .unwrap();
        assert_eq!(communications_capability_flight_status.com, COM::CommACommB);
        assert_eq!(
            communications_capability_flight_status.stat,
            STAT::NoAlertNoSPIAircraftAirborne
        );
        assert_eq!(
            communications_capability_flight_status.si,
            SI::SICodeCapable
        );
        assert_eq!(communications_capability_flight_status.mssc, MSSC::Yes);
        assert_eq!(
            communications_capability_flight_status.arc,
            ARC::Resolution25ft
        );
        assert_eq!(communications_capability_flight_status.aic, AIC::Yes);
        assert_eq!(communications_capability_flight_status.b1a, 1);
        assert_eq!(communications_capability_flight_status.b1b, 5);
    } else {
        unreachable!("Message is not CAT48");
    }

    // Confirm packet back to bytes
    assert_eq!(packet.to_bytes(), Ok(bytes));
}

#[test]
/// Remove communications_capability_flight_status and update fspec, making sure to check if the
/// fspec works as well as the communications_capability_flight_status is none
fn it_works_only_two_fspec() {
    let bytes = vec![
        0x30, 0x00, 0x30, 0xfd, 0xf6, 0x19, 0xc9, 0x35, 0x6d, 0x4d, 0xa0, 0xc5, 0xaf, 0xf1,
        0xe0, 0x02, 0x00, 0x05, 0x28, 0x3c, 0x66, 0x0c, 0x10, 0xc2, 0x36, 0xd4, 0x18,
        //0x20 in wireshark, but last 6 bits don't matter and will 0x00 by writer
        0x00, 0x01, 0xc0, 0x78, 0x00, 0x31, 0xbc, 0x00, 0x00, 0x40, 0x0d, 0xeb, 0x07, 0xb9, 0x58,
        0x2e, 0x41, 0x00
    ];
    let (_, packet) = AsterixPacket::from_bytes((&bytes, 0)).unwrap();

    assert_eq!(packet.category, 48);
    assert_eq!(packet.length, 48);

    if let AsterixMessage::Cat48(ref message) = packet.message {
        assert_eq!(message.fspec, &[0xfd, 0xf6]);
        // everything here is checked in the above test
        assert!(message.communications_capability_flight_status.is_none());
    } else {
        unreachable!("Message is not CAT48");
    }

    // Confirm packet back to bytes
    assert_eq!(packet.to_bytes(), Ok(bytes));
}
