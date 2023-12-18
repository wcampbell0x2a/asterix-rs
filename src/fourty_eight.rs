use crate::data_item::{
    ACASResolutionAdvisoryReport, AircraftAddress, AircraftIdentification,
    CalculatedPositionCartesianCorr, CalculatedTrackVelocity, CommunicationsCapabilityFlightStatus,
    DataSourceIdentifier, FlightLevelInBinaryRepresentation, HeightMeasuredBy3dRadar,
    MeasuredPositionInPolarCoordinates, Mode1CodeConfidenceIndicator, Mode1CodeOctalRepresentation,
    Mode2CodeConfidenceIndicator, Mode2CodeOctalRepresentation, Mode3ACodeConfidenceIndicator,
    Mode3ACodeInOctalRepresentation, ModeCCodeAndConfidenceIndicator, ModeSMBData,
    RadarPlotCharacteristics, RadialDopplerSpeed, TargetReportDescriptor, TimeOfDay, TrackNumber,
    TrackQuality, TrackStatus, WarningErrorConditionsTargetClass,
};
use crate::fspec::{add_fx, is_fspec, trim_fspec};
use crate::FSPEC_IDENT;
use asterix_derive::UpdateFspec;
use deku::prelude::*;

/// Transmission of Monoradar Target Reports
#[derive(Debug, Default, PartialEq, DekuRead, DekuWrite, UpdateFspec)]
#[deku(endian = "big")]
pub struct Cat48 {
    #[deku(until = "|b: &u8| *b & FSPEC_IDENT == 0")]
    pub fspec: Vec<u8>,
    /// FRN 1
    #[deku(skip, cond = "is_fspec(DataSourceIdentifier::FRN_48, fspec, 0)")]
    pub data_source_identifier: Option<DataSourceIdentifier>,
    /// FRN 2
    #[deku(skip, cond = "is_fspec(TimeOfDay::FRN_48, fspec, 0)")]
    pub time_of_day: Option<TimeOfDay>,
    /// FRN 3
    #[deku(skip, cond = "is_fspec(TargetReportDescriptor::FRN_48, fspec, 0)")]
    pub target_report_descriptor: Option<TargetReportDescriptor>,
    /// FRN 4
    #[deku(skip, cond = "is_fspec(MeasuredPositionInPolarCoordinates::FRN_48, fspec, 0)")]
    pub measured_position_in_polar_coordinates: Option<MeasuredPositionInPolarCoordinates>,
    /// FRN 5
    #[deku(skip, cond = "is_fspec(Mode3ACodeInOctalRepresentation::FRN_48, fspec, 0)")]
    pub mode_3_a_code_in_octal_representation: Option<Mode3ACodeInOctalRepresentation>,
    /// FRN 6
    #[deku(skip, cond = "is_fspec(FlightLevelInBinaryRepresentation::FRN_48, fspec, 0)")]
    pub flight_level_in_binary_repre: Option<FlightLevelInBinaryRepresentation>,
    /// FRN 7
    #[deku(skip, cond = "is_fspec(RadarPlotCharacteristics::FRN_48, fspec, 0)")]
    pub radar_plot_characteristics: Option<RadarPlotCharacteristics>,
    /// FRN 8
    #[deku(skip, cond = "is_fspec(AircraftAddress::FRN_48, fspec, 1)")]
    pub aircraft_address: Option<AircraftAddress>,
    /// FRN 9
    #[deku(skip, cond = "is_fspec(AircraftIdentification::FRN_48, fspec, 1)")]
    pub aircraft_identification: Option<AircraftIdentification>,
    /// FRN 10
    #[deku(skip, cond = "is_fspec(ModeSMBData::FRN_48, fspec, 1)")]
    pub mode_smb_data: Option<ModeSMBData>,
    /// FRN 11
    #[deku(skip, cond = "is_fspec(TrackNumber::FRN_48, fspec, 1)")]
    pub track_number: Option<TrackNumber>,
    /// FRN 12
    #[deku(skip, cond = "is_fspec(CalculatedPositionCartesianCorr::FRN_48, fspec, 1)")]
    pub calculated_position_cartesian_coor: Option<CalculatedPositionCartesianCorr>,
    /// FRN 13
    #[deku(skip, cond = "is_fspec(CalculatedTrackVelocity::FRN_48, fspec, 1)")]
    pub calculated_track_velocity: Option<CalculatedTrackVelocity>,
    /// FRN 14
    #[deku(skip, cond = "is_fspec(TrackStatus::FRN_48, fspec, 1)")]
    pub track_status: Option<TrackStatus>,
    /// FRN 15
    #[deku(skip, cond = "is_fspec(TrackQuality::FRN_48, fspec, 2)")]
    pub track_quality: Option<TrackQuality>,
    /// FRN 16
    #[deku(skip, cond = "is_fspec(WarningErrorConditionsTargetClass::FRN_48, fspec, 2)")]
    pub warning_error_con_target_class: Option<WarningErrorConditionsTargetClass>,
    /// FRN 17
    #[deku(skip, cond = "is_fspec(Mode3ACodeConfidenceIndicator::FRN_48, fspec, 2)")]
    pub mode3a_code_confidence_indicator: Option<Mode3ACodeConfidenceIndicator>,
    /// FRN 18
    #[deku(skip, cond = "is_fspec(ModeCCodeAndConfidenceIndicator::FRN_48, fspec, 2)")]
    pub modec_code_and_confidence_indicator: Option<ModeCCodeAndConfidenceIndicator>,
    /// FRN 19
    #[deku(skip, cond = "is_fspec(HeightMeasuredBy3dRadar::FRN_48, fspec, 2)")]
    pub height_measured_by_3d_radar: Option<HeightMeasuredBy3dRadar>,
    /// FRN 20
    #[deku(skip, cond = "is_fspec(RadialDopplerSpeed::FRN_48, fspec, 2)")]
    pub radial_doppler_speed: Option<RadialDopplerSpeed>,
    /// FRN 21
    #[deku(skip, cond = "is_fspec(CommunicationsCapabilityFlightStatus::FRN_48, fspec, 2)")]
    pub communications_capability_flight_status: Option<CommunicationsCapabilityFlightStatus>,
    /// FRN 22
    #[deku(skip, cond = "is_fspec(ACASResolutionAdvisoryReport::FRN_48, fspec, 3)")]
    pub acas_resolution_advisory_report: Option<ACASResolutionAdvisoryReport>,
    /// FRN 23
    #[deku(skip, cond = "is_fspec(Mode1CodeOctalRepresentation::FRN_48, fspec, 3)")]
    pub mode_1_code_octal_representation: Option<Mode1CodeOctalRepresentation>,
    /// FRN 24
    #[deku(skip, cond = "is_fspec(Mode2CodeOctalRepresentation::FRN_48, fspec, 3)")]
    pub mode_2_code_octal_representation: Option<Mode2CodeOctalRepresentation>,
    /// FRN 25
    #[deku(skip, cond = "is_fspec(Mode1CodeConfidenceIndicator::FRN_48, fspec, 3)")]
    pub mode_1_code_confidence: Option<Mode1CodeConfidenceIndicator>,
    /// FRN 26
    #[deku(skip, cond = "is_fspec(Mode2CodeConfidenceIndicator::FRN_48, fspec, 3)")]
    pub mode_2_code_confidence: Option<Mode2CodeConfidenceIndicator>,
    // FRN 27: Special Purpose Field
    // FRN 28: Reserved Expansion Field
}
