use asterix::data_item::{
    CodeFx, DataSourceIdentifier, HeightMeasuredBy3dRadar, MBData, MessageType,
    Mode3ACodeConfidenceIndicator, ModeCCodeAndConfidenceIndicator, SectorNumber, TimeOfDay,
    TrackQuality, WarningErrorConditionsTargetClass,
};
use asterix::types::{
    AIC, ARC, CDM, CNF, CODE, COM, DOU, FX, G, GHO, L, MAH, MSSC, MTYPE, RAB, RAD, RDP, SI, SIM,
    SPI, STAT, SUP, TCC, TRE, TYP, V,
};
use asterix::{AsterixMessage, AsterixPacket, Cat34, Cat48};
use deku::{DekuContainerRead, DekuContainerWrite};

#[test]
fn it_works() {
    let bytes = vec![
        0x30, 0x00, 0x30, 0xfd, 0xf7, 0x02, 0x19, 0xc9, 0x35, 0x6d, 0x4d, 0xa0, 0xc5, 0xaf, 0xf1,
        0xe0, 0x02, 0x00, 0x05, 0x28, 0x3c, 0x66, 0x0c, 0x10, 0xc2, 0x36, 0xd4, 0x18,
        //0x20 in wireshark, but last 6 bits don't matter and will 0x00 by writer
        0x00, 0x01, 0xc0, 0x78, 0x00, 0x31, 0xbc, 0x00, 0x00, 0x40, 0x0d, 0xeb, 0x07, 0xb9, 0x58,
        0x2e, 0x41, 0x00, 0x20, 0xf5,
    ];
    let (_, mut packet) = AsterixPacket::from_bytes((&bytes, 0)).unwrap();

    assert_eq!(packet.category, 48);
    assert_eq!(packet.length, 48);

    // TODO check NONE values after more are implemented
    if let AsterixMessage::Cat48(ref mut message) = packet.messages[0] {
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

    packet.finalize().unwrap();

    // Confirm packet back to bytes
    assert_eq!(packet.to_bytes(), Ok(bytes));
}

#[test]
/// Remove communications_capability_flight_status and update fspec, making sure to check if the
/// fspec works as well as the communications_capability_flight_status is none
fn it_works_only_two_fspec() {
    let bytes = vec![
        0x30, 0x00, 0x2d, 0xfd, 0xf6, 0x19, 0xc9, 0x35, 0x6d, 0x4d, 0xa0, 0xc5, 0xaf, 0xf1, 0xe0,
        0x02, 0x00, 0x05, 0x28, 0x3c, 0x66, 0x0c, 0x10, 0xc2, 0x36, 0xd4, 0x18, 0x00, 0x01, 0xc0,
        0x78, 0x00, 0x31, 0xbc, 0x00, 0x00, 0x40, 0x0d, 0xeb, 0x07, 0xb9, 0x58, 0x2e, 0x41, 0x00,
    ];
    let (_, mut packet) = AsterixPacket::from_bytes((&bytes, 0)).unwrap();

    assert_eq!(packet.category, 48);
    assert_eq!(packet.length, 45);

    if let AsterixMessage::Cat48(ref message) = packet.messages[0] {
        assert_eq!(message.fspec, &[0xfd, 0xf6]);
        // everything here is checked in the above test
        assert!(message.communications_capability_flight_status.is_none());
    } else {
        unreachable!("Message is not CAT48");
    }

    packet.finalize().unwrap();

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
    let (_, mut packet) = AsterixPacket::from_bytes((&bytes, 0)).unwrap();
    assert_eq!(packet.category, 48);
    assert_eq!(packet.length, 55);

    // TODO check NONE values after more are implemented
    if let AsterixMessage::Cat48(ref message) = packet.messages[0] {
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
    packet.finalize().unwrap();
    assert_eq!(packet.to_bytes(), Ok(bytes));
}

#[test]
fn test_34() {
    let bytes = vec![
        0x22, 0x00, 0x0b, 0xf0, 0x19, 0x0d, 0x02, 0x35, 0x6d, 0xfa, 0x60,
    ];
    let (_, mut packet) = AsterixPacket::from_bytes((&bytes, 0)).unwrap();

    assert_eq!(packet.category, 34);
    assert_eq!(packet.length, 11);

    // TODO check NONE values after more are implemented
    if let AsterixMessage::Cat34(ref message) = packet.messages[0] {
        assert_eq!(message.fspec, &[0xf0]);

        let data_source_identifier = message.data_source_identifier.as_ref().unwrap();
        assert_eq!(data_source_identifier.sac, 25);
        assert_eq!(data_source_identifier.sic, 13);

        let message_type = message.message_type.as_ref().unwrap();
        assert_eq!(message_type.t, MTYPE::SectorCrossing);

        let time_of_day = message.time_of_day.as_ref().unwrap();
        assert_eq!(time_of_day.time, 27355.953);

        let sector_number = message.sector_number.as_ref().unwrap();
        assert_eq!(sector_number.num, 135);
    } else {
        unreachable!("Not Cat 34");
    }
    packet.finalize().unwrap();
    assert_eq!(packet.to_bytes(), Ok(bytes));
}

#[test]
fn test_four_messages() {
    // Example of having multiple asterix messages being received in one packet, this requires one to
    // parse the first messages, and parsing until the rest.len() == 0
    let bytes = vec![
        // Cat 048
        0x30, 0x00, 0xb9, 0xe1, 0x93, 0x02, 0x19, 0x0d, 0x35, 0x64, 0x21, 0x00, 0x44, 0xd0, 0x74,
        0x02, 0xda, 0x41, 0x80, 0x20, 0xf5, 0xff, 0xff, 0x02, 0x19, 0x0d, 0x35, 0x6e, 0x06, 0xa0,
        0x2b, 0x4d, 0x65, 0x1e, 0x04, 0x18, 0x05, 0xa0, 0xe0, 0x56, 0x0c, 0xcf, 0x46, 0x92, 0xd1,
        0x04, 0x51, 0x72, 0x09, 0x28, 0x20, 0x02, 0xc6, 0x50, 0x00, 0x30, 0x7c, 0x00, 0x00, 0x40,
        0xf0, 0x09, 0xf7, 0x2f, 0xa0, 0x64, 0x02, 0x60, 0x02, 0xf9, 0x0d, 0x46, 0xee, 0xe5, 0x07,
        0xdc, 0xe1, 0xb5, 0x40, 0x20, 0xfd, 0xff, 0xff, 0x02, 0x19, 0x0d, 0x35, 0x6d, 0xfd, 0xa0,
        0x63, 0xd3, 0x61, 0x68, 0x0e, 0xaa, 0x05, 0xa0, 0xe0, 0x56, 0x0c, 0xc2, 0x3c, 0x64, 0x8a,
        0x10, 0xc2, 0x36, 0xe7, 0x58, 0x20, 0x01, 0xc6, 0x50, 0x00, 0x30, 0xb8, 0x00, 0x00, 0x40,
        0x05, 0x9b, 0x22, 0x10, 0xdb, 0x84, 0x07, 0xfc, 0xe2, 0xff, 0x40, 0x20, 0xf5, 0xff, 0xff,
        0x02, 0x19, 0x0d, 0x35, 0x6e, 0x03, 0xa0, 0x6d, 0xa2, 0x64, 0x22, 0x09, 0xe5, 0x06, 0x40,
        0xe0, 0x64, 0x0c, 0xc0, 0xa0, 0x22, 0xa3, 0x3b, 0x1c, 0x38, 0x0c, 0x58, 0x20, 0x01, 0xf1,
        0x19, 0xe9, 0x32, 0x60, 0x04, 0x00, 0x60, 0x05, 0x90, 0x22, 0xa3, 0xd5, 0x83, 0x07, 0xeb,
        0xe2, 0x78, 0x40, 0x20, 0xf6, // Cat 034
        0x22, 0x00, 0x0b, 0xf0, 0x19, 0x0d, 0x02, 0x35, 0x6e, 0x0e, 0x68,
    ];
    let (rest, mut packet) = AsterixPacket::from_bytes((&bytes, 0)).unwrap();
    packet.finalize().unwrap();
    assert_eq!(packet.category, 48);
    let (_, mut packet) = AsterixPacket::from_bytes(rest).unwrap();
    packet.finalize().unwrap();
    assert_eq!(packet.category, 34);
}

#[test]
fn test_not_from_bytes() {
    let mut thirty_eight = Cat34::default();
    thirty_eight.data_source_identifier = Some(DataSourceIdentifier { sac: 25, sic: 13 });
    thirty_eight.message_type = Some(MessageType {
        t: MTYPE::SectorCrossing,
    });
    thirty_eight.time_of_day = Some(TimeOfDay { time: 27355.953 });
    thirty_eight.sector_number = Some(SectorNumber { num: 135 });

    let mut packet = AsterixPacket::default();
    packet.category = 34;
    packet.messages = vec![asterix::AsterixMessage::Cat34(thirty_eight)];
    packet.finalize().unwrap();
    let exp_bytes = vec![
        0x22, 0x00, 0x0b, 0xf0, 0x19, 0x0d, 0x02, 0x35, 0x6d, 0xfa, 0x60,
    ];
    assert_eq!(packet.to_bytes().unwrap(), exp_bytes)
}

// The following data items don't have pcap captures, and are my own testing

#[test]
fn test_48_track_quality() {
    let mut fourty_eight = Cat48::default();
    fourty_eight.track_quality = Some(TrackQuality {
        horizontal_stddev: 0.0,
        vertical_stddev: 0.0,
        groundspeed_stddev: 0.0,
        heading_stddev: 0.0,
    });
    let mut packet = AsterixPacket::default();
    packet.category = 48;
    packet.messages = vec![asterix::AsterixMessage::Cat48(fourty_eight)];
    packet.finalize().unwrap();
    let exp_bytes = vec![
        0x30,
        0x00,
        0x0a,
        0x01,
        0x01,
        0b1000_0000,
        0x00,
        0x00,
        0x00,
        0x00,
    ];
    assert_eq!(packet.to_bytes().unwrap(), exp_bytes);
    let (_, exp_packet) = AsterixPacket::from_bytes((&exp_bytes, 0)).unwrap();
    assert_eq!(packet, exp_packet);

    let mut fourty_eight = Cat48::default();
    fourty_eight.track_quality = Some(TrackQuality {
        horizontal_stddev: 32000.0,
        vertical_stddev: 32000.0,
        groundspeed_stddev: 0.015_563_965,
        heading_stddev: 22.412_11,
    });
    let mut packet = AsterixPacket::default();
    packet.category = 48;
    packet.messages = vec![asterix::AsterixMessage::Cat48(fourty_eight)];
    packet.finalize().unwrap();
    let exp_bytes = vec![
        0x30,
        0x00,
        0x0a,
        0x01,
        0x01,
        0b1000_0000,
        0xfa,
        0xfa,
        0xff,
        0xff,
    ];
    assert_eq!(packet.to_bytes().unwrap(), exp_bytes);
    let (_, exp_packet) = AsterixPacket::from_bytes((&exp_bytes, 0)).unwrap();
    assert_eq!(packet, exp_packet);
}

#[test]
fn test_48_warning_error_con_target_class() {
    let mut fourty_eight = Cat48::default();

    let warning = WarningErrorConditionsTargetClass {
        codefxs: vec![
            CodeFx {
                code: CODE::Angel,
                fx: FX::ExtensionIntoFirstExtent,
            },
            CodeFx {
                code: CODE::Angel,
                fx: FX::EndOfDataItem,
            },
        ],
    };

    fourty_eight.warning_error_con_target_class = Some(warning);
    let mut packet = AsterixPacket::default();
    packet.category = 48;
    packet.messages = vec![asterix::AsterixMessage::Cat48(fourty_eight)];
    packet.finalize().unwrap();
    let exp_bytes = vec![
        0x30,
        0x00,
        0x08,
        0x01,
        0x01,
        0b100_0000,
        0x5 << 1 | 0x01,
        0x5 << 1,
    ];
    assert_eq!(packet.to_bytes().unwrap(), exp_bytes);
    let (_, exp_packet) = AsterixPacket::from_bytes((&exp_bytes, 0)).unwrap();
    assert_eq!(packet, exp_packet);
}

#[test]
fn test_48_mode_3a_code_confidence_indicator() {
    let mut fourty_eight = Cat48::default();

    let confidence = Mode3ACodeConfidenceIndicator {
        reserved: 0,
        confidence: 0b0000_0001,
    };
    fourty_eight.mode3a_code_confidence_indicator = Some(confidence);
    let mut packet = AsterixPacket::default();
    packet.category = 48;
    packet.messages = vec![asterix::AsterixMessage::Cat48(fourty_eight)];
    packet.finalize().unwrap();
    let exp_bytes = vec![0x30, 0x00, 0x08, 0x01, 0x01, 0b10_0000, 0x00, 0x01];
    assert_eq!(packet.to_bytes().unwrap(), exp_bytes);
    let (_, exp_packet) = AsterixPacket::from_bytes((&exp_bytes, 0)).unwrap();
    assert_eq!(packet, exp_packet);
}

#[test]
fn test_48_mode_c_code_confidence() {
    let mut fourty_eight = Cat48::default();
    let confidence = ModeCCodeAndConfidenceIndicator {
        v: V::CodeValidated,
        g: G::Default,
        reserved0: 0,
        mode_c_gray_notation: 0x01,
        reserved1: 0,
        confidence: 0x01,
    };
    fourty_eight.modec_code_and_confidence_indicator = Some(confidence);
    let mut packet = AsterixPacket::default();
    packet.category = 48;
    packet.messages = vec![asterix::AsterixMessage::Cat48(fourty_eight)];
    packet.finalize().unwrap();
    let exp_bytes = vec![
        0x30, 0x00, 0x0a, 0x01, 0x01, 0b1_0000, 0x00, 0x01, 0x00, 0x01,
    ];
    assert_eq!(packet.to_bytes().unwrap(), exp_bytes);
    let (_, exp_packet) = AsterixPacket::from_bytes((&exp_bytes, 0)).unwrap();
    assert_eq!(packet, exp_packet);
}

#[test]
fn test_48_height_3d() {
    let mut fourty_eight = Cat48::default();
    let height = HeightMeasuredBy3dRadar {
        reserved: 0,
        height: 25,
    };
    fourty_eight.height_measured_by_3d_radar = Some(height);
    let mut packet = AsterixPacket::default();
    packet.category = 48;
    packet.messages = vec![asterix::AsterixMessage::Cat48(fourty_eight)];
    packet.finalize().unwrap();
    let exp_bytes = vec![0x30, 0x00, 0x08, 0x01, 0x01, 0b1000, 0x00, 0x01];
    assert_eq!(packet.to_bytes().unwrap(), exp_bytes);
    let (_, exp_packet) = AsterixPacket::from_bytes((&exp_bytes, 0)).unwrap();
    assert_eq!(packet, exp_packet);

    let mut fourty_eight = Cat48::default();
    let height = HeightMeasuredBy3dRadar {
        reserved: 0,
        height: 37200,
    };
    fourty_eight.height_measured_by_3d_radar = Some(height);
    let mut packet = AsterixPacket::default();
    packet.category = 48;
    packet.messages = vec![asterix::AsterixMessage::Cat48(fourty_eight)];
    packet.finalize().unwrap();
    let exp_bytes = vec![0x30, 0x00, 0x08, 0x01, 0x01, 0b1000, 0x05, 0xd0];
    assert_eq!(packet.to_bytes().unwrap(), exp_bytes);
    let (_, exp_packet) = AsterixPacket::from_bytes((&exp_bytes, 0)).unwrap();
    assert_eq!(packet, exp_packet);
}

#[test]
fn test_48_radial_dopplerspeed() {
    // test the first subfield
    let bytes = vec![
        0x30,
        0x00,
        0x09,
        0x01,
        0x01,
        0b100,
        0b1000_0000,
        0b1000_0000,
        0b0000_0001,
    ];
    let (_, packet) = AsterixPacket::from_bytes((&bytes, 0)).unwrap();
    assert_eq!(packet.to_bytes().unwrap(), bytes);

    // test the second subfield
    let bytes = vec![
        0x30,
        0x00,
        0x0e,
        0x01,
        0x01,
        0b100,
        0b0100_0000,
        0x01,
        0x00,
        0x01,
        0x00,
        0x01,
        0x00,
        0x01,
    ];
    let (_, packet) = AsterixPacket::from_bytes((&bytes, 0)).unwrap();
    assert_eq!(packet.to_bytes().unwrap(), bytes);
}

#[test]
fn test_acas_resolution() {
    let bytes = vec![
        0x30,
        0x00,
        0x0e,
        0x01,
        0x01,
        0x01,
        0b1000_0000,
        0x01,
        0x02,
        0x03,
        0x04,
        0x05,
        0x06,
        0x07,
    ];
    let (_, packet) = AsterixPacket::from_bytes((&bytes, 0)).unwrap();
    assert_eq!(packet.to_bytes().unwrap(), bytes);

    if let AsterixMessage::Cat48(ref message) = packet.messages[0] {
        assert_eq!(message.fspec, &[0x01, 0x01, 0x01, 0b1000_0000]);

        let field = message.acas_resolution_advisory_report.as_ref().unwrap();
        assert_eq!(&field.mb_data, &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
    }
}

#[test]
fn test_mode1code_octal_representation() {
    let bytes = vec![0x30, 0x00, 0x08, 0x01, 0x01, 0x01, 0b0100_0000, 0b0000_0001];
    let (_, packet) = AsterixPacket::from_bytes((&bytes, 0)).unwrap();
    assert_eq!(packet.to_bytes().unwrap(), bytes);

    if let AsterixMessage::Cat48(ref message) = packet.messages[0] {
        assert_eq!(message.fspec, &[0x01, 0x01, 0x01, 0b0100_0000]);

        let field = message.mode_1_code_octal_representation.as_ref().unwrap();
        assert_eq!(field.v, V::CodeValidated);
        assert_eq!(field.g, G::Default);
        assert_eq!(field.l, L::Mode3CodeDerivedFromTheReplyOfTheTransponder);
        assert_eq!(field.data, 0x01);
    }
}

#[test]
fn test_mode2code_octal_representation() {
    let bytes = vec![
        0x30,
        0x00,
        0x09,
        0x01,
        0x01,
        0x01,
        0b0010_0000,
        0b0000_0000,
        0b0000_0001,
    ];
    let (_, packet) = AsterixPacket::from_bytes((&bytes, 0)).unwrap();
    assert_eq!(packet.to_bytes().unwrap(), bytes);

    if let AsterixMessage::Cat48(ref message) = packet.messages[0] {
        assert_eq!(message.fspec, &[0x01, 0x01, 0x01, 0b0010_0000]);

        let field = message.mode_2_code_octal_representation.as_ref().unwrap();
        assert_eq!(field.v, V::CodeValidated);
        assert_eq!(field.g, G::Default);
        assert_eq!(field.l, L::Mode3CodeDerivedFromTheReplyOfTheTransponder);
        assert_eq!(field.data, 0x01);
    }
}
