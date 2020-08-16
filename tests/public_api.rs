use asterix_deku::{AsterixMessage, AsterixPacket};
use asterix_deku::data_item::*;
use asterix_deku::types::*;
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
        //TODO add squawk?

        let flight_level_in_binary_repre = message.flight_level_in_binary_repre.as_ref().unwrap();
        assert_eq!(flight_level_in_binary_repre.v, V::CodeValidated);
        assert_eq!(flight_level_in_binary_repre.g, G::Default);
        assert_eq!(flight_level_in_binary_repre.flight_level, 330);

        let aircraft_address = message.aircraft_address.as_ref().unwrap();
        assert_eq!(aircraft_address.address, 0x003c_660c);

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
        // TODO assert BDS1
        // TODO assert BDS2

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
        assert_eq!(track_status.tre, Some(TRE::TrackStillAlive));
        assert_eq!(track_status.gho, Some(GHO::TrueTargetTrack));
        assert_eq!(track_status.sup, Some(SUP::No));
        assert_eq!(track_status.tcc, Some(TCC::RadarPlanePlotTransformation));
        assert_eq!(track_status.fx2, Some(FX::EndOfDataItem));

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
        0x30, 0x00, 0x30, 0xfd, 0xf6, 0x19, 0xc9, 0x35, 0x6d, 0x4d, 0xa0, 0xc5, 0xaf, 0xf1, 0xe0,
        0x02, 0x00, 0x05, 0x28, 0x3c, 0x66, 0x0c, 0x10, 0xc2, 0x36, 0xd4, 0x18, 0x00, 0x01, 0xc0,
        0x78, 0x00, 0x31, 0xbc, 0x00, 0x00, 0x40, 0x0d, 0xeb, 0x07, 0xb9, 0x58, 0x2e, 0x41, 0x00,
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

#[test]
fn third_packet() {
    let bytes = vec![
        0x30, 0x00, 0x37, 0xff, 0xff, 0x02, 0x19, 0x0d, 0x35, 0x6d, 0xee, 0xa0, 0xc2, 0xd3, 0x5b,
        0x90, 0x04, 0xc3, 0x05, 0xa0, 0xe0, 0x56, 0x0b, 0xb8, 0x4b, 0xaa, 0xcd, 0x50, 0x86, 0x79,
        0x51, 0x88, 0x00, 0x01, 0xc6, 0x56, 0x32, 0xb0, 0xa8, 0x00, 0x00, 0x40, 0x01, 0xe2, 0x4b,
        0xf6, 0xc3, 0x04, 0x08, 0x1e, 0xbb, 0x73, 0x40, 0x20, 0xf5,
    ];
    // TODO: Add CAT034
    let (_, packet) = AsterixPacket::from_bytes((&bytes, 0)).unwrap();
    println!("{:#?}", packet);

    assert_eq!(packet.category, 48);
    assert_eq!(packet.length, 55);

    // TODO check NONE values after more are implemented
    if let AsterixMessage::Cat48(ref message) = packet.message {
        assert_eq!(message.fspec, &[0xff, 0x0ff, 0x02]);

        let data_source_identifier = message.data_source_identifier.as_ref().unwrap();
        assert_eq!(data_source_identifier.sac, 25);
        assert_eq!(data_source_identifier.sic, 13);

        let time_of_day = message.time_of_day.as_ref().unwrap();
        assert_eq!(time_of_day.time, 27_355.86);

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
        assert_eq!(measured_position_in_polar_coordinates.rho, 194.824_22);
        assert_eq!(measured_position_in_polar_coordinates.theta, 128.759_77);

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
        //TODO add squawk?

        let flight_level_in_binary_repre = message.flight_level_in_binary_repre.as_ref().unwrap();
        assert_eq!(flight_level_in_binary_repre.v, V::CodeValidated);
        assert_eq!(flight_level_in_binary_repre.g, G::Default);
        assert_eq!(flight_level_in_binary_repre.flight_level, 360);

        let aircraft_address = message.aircraft_address.as_ref().unwrap();
        assert_eq!(aircraft_address.address, 0x004b_aacd);

        let aircraft_identification = message.aircraft_identification.as_ref().unwrap();
        assert_eq!(aircraft_identification.identification, "THY9TX ");

        let mode_smb_data = message.mode_smb_data.as_ref().unwrap();
        assert_eq!(mode_smb_data.count, 1);
        assert_eq!(
            mode_smb_data.mb_data,
            vec![MBData {
                data: [0xc6, 0x56, 0x32, 0xb0, 0xa8, 0x00, 0x00].to_vec()
            }]
        );
        assert_eq!(mode_smb_data.bds1, 4);
        assert_eq!(mode_smb_data.bds2, 0);

        let track_number = message.track_number.as_ref().unwrap();
        assert_eq!(track_number.number, 482);

        let cal_pos_cartesian_coor = message.calculated_position_cartesian_coor.as_ref().unwrap();
        assert_eq!(cal_pos_cartesian_coor.x, 151.921_88);
        assert_eq!(cal_pos_cartesian_coor.y, -121.96875);

        let calculated_track_velocity = message.calculated_track_velocity.as_ref().unwrap();
        assert_eq!(calculated_track_velocity.groundspeed, 0.126_831_05);
        assert_eq!(calculated_track_velocity.heading, 263.600_46);

        let track_status = message.track_status.as_ref().unwrap();
        assert_eq!(track_status.cnf, CNF::ConfirmedTrack);
        assert_eq!(track_status.rad, RAD::SSRModeSTrack);
        assert_eq!(track_status.dou, DOU::NormalConfidence);
        assert_eq!(track_status.mah, MAH::NoHorizontalManSensed);
        assert_eq!(track_status.cdm, CDM::Maintaining);
        assert_eq!(track_status.fx1, FX::EndOfDataItem);
        assert_eq!(track_status.tre, None);
        assert_eq!(track_status.gho, None);
        assert_eq!(track_status.sup, None);
        assert_eq!(track_status.tcc, None);
        assert_eq!(track_status.fx2, None);

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
    assert_eq!(packet.to_bytes(), Ok(bytes));
}
