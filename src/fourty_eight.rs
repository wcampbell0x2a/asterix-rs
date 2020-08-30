use deku::prelude::*;

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
use crate::fspec::{add_fx, is_fspec, read_fspec, trim_fspec};

// TODO: use const fspecs
#[derive(Debug, Default, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Cat48 {
    #[deku(reader = "read_fspec(rest)")]
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
    #[deku(
        skip,
        cond = "is_fspec(MeasuredPositionInPolarCoordinates::FRN_48, fspec, 0)"
    )]
    pub measured_position_in_polar_coordinates: Option<MeasuredPositionInPolarCoordinates>,
    /// FRN 5
    #[deku(
        skip,
        cond = "is_fspec(Mode3ACodeInOctalRepresentation::FRN_48, fspec, 0)"
    )]
    pub mode_3_a_code_in_octal_representation: Option<Mode3ACodeInOctalRepresentation>,
    /// FRN 6
    #[deku(
        skip,
        cond = "is_fspec(FlightLevelInBinaryRepresentation::FRN_48, fspec, 0)"
    )]
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
    #[deku(
        skip,
        cond = "is_fspec(CalculatedPositionCartesianCorr::FRN_48, fspec, 1)"
    )]
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
    #[deku(
        skip,
        cond = "is_fspec(WarningErrorConditionsTargetClass::FRN_48, fspec, 2)"
    )]
    pub warning_error_con_target_class: Option<WarningErrorConditionsTargetClass>,
    /// FRN 17
    #[deku(
        skip,
        cond = "is_fspec(Mode3ACodeConfidenceIndicator::FRN_48, fspec, 2)"
    )]
    pub mode3a_code_confidence_indicator: Option<Mode3ACodeConfidenceIndicator>,
    /// FRN 18
    #[deku(
        skip,
        cond = "is_fspec(ModeCCodeAndConfidenceIndicator::FRN_48, fspec, 2)"
    )]
    pub modec_code_and_confidence_indicator: Option<ModeCCodeAndConfidenceIndicator>,
    /// FRN 19
    #[deku(skip, cond = "is_fspec(HeightMeasuredBy3dRadar::FRN_48, fspec, 2)")]
    pub height_measured_by_3d_radar: Option<HeightMeasuredBy3dRadar>,
    /// FRN 20
    #[deku(skip, cond = "is_fspec(RadialDopplerSpeed::FRN_48, fspec, 2)")]
    pub radial_doppler_speed: Option<RadialDopplerSpeed>,
    /// FRN 21
    #[deku(
        skip,
        cond = "is_fspec(CommunicationsCapabilityFlightStatus::FRN_48, fspec, 2)"
    )]
    pub communications_capability_flight_status: Option<CommunicationsCapabilityFlightStatus>,
    /// FRN 22
    #[deku(
        skip,
        cond = "is_fspec(ACASResolutionAdvisoryReport::FRN_48, fspec, 3)"
    )]
    pub acas_resolution_advisory_report: Option<ACASResolutionAdvisoryReport>,
    /// FRN 23
    #[deku(
        skip,
        cond = "is_fspec(Mode1CodeOctalRepresentation::FRN_48, fspec, 3)"
    )]
    pub mode_1_code_octal_representation: Option<Mode1CodeOctalRepresentation>,
    /// FRN 24
    #[deku(
        skip,
        cond = "is_fspec(Mode2CodeOctalRepresentation::FRN_48, fspec, 3)"
    )]
    pub mode_2_code_octal_representation: Option<Mode2CodeOctalRepresentation>,
    /// FRN 25
    #[deku(
        skip,
        cond = "is_fspec(Mode1CodeConfidenceIndicator::FRN_48, fspec, 3)"
    )]
    pub mode_1_code_confidence: Option<Mode1CodeConfidenceIndicator>,
    /// FRN 26
    #[deku(
        skip,
        cond = "is_fspec(Mode2CodeConfidenceIndicator::FRN_48, fspec, 3)"
    )]
    pub mode_2_code_confidence: Option<Mode2CodeConfidenceIndicator>,
    // FRN 27
    // FRN 28
}

impl Cat48 {
    pub fn update_fspec(&mut self) {
        // Start with max fpsec
        let mut fspec = vec![0x00, 0x00, 0x00, 0x00];
        // add Data Items fspecs where they are Some
        if self.data_source_identifier.is_some() {
            fspec[0] |= DataSourceIdentifier::FRN_48;
        }
        if self.time_of_day.is_some() {
            fspec[0] |= TimeOfDay::FRN_48;
        }
        if self.target_report_descriptor.is_some() {
            fspec[0] |= TargetReportDescriptor::FRN_48;
        }
        if self.measured_position_in_polar_coordinates.is_some() {
            fspec[0] |= MeasuredPositionInPolarCoordinates::FRN_48;
        }
        if self.mode_3_a_code_in_octal_representation.is_some() {
            fspec[0] |= Mode3ACodeInOctalRepresentation::FRN_48;
        }
        if self.flight_level_in_binary_repre.is_some() {
            fspec[0] |= FlightLevelInBinaryRepresentation::FRN_48;
        }
        if self.radar_plot_characteristics.is_some() {
            fspec[0] |= RadarPlotCharacteristics::FRN_48;
        }
        if self.aircraft_address.is_some() {
            fspec[1] |= AircraftAddress::FRN_48;
        }
        if self.aircraft_identification.is_some() {
            fspec[1] |= AircraftIdentification::FRN_48;
        }
        if self.mode_smb_data.is_some() {
            fspec[1] |= ModeSMBData::FRN_48;
        }
        if self.track_number.is_some() {
            fspec[1] |= TrackNumber::FRN_48;
        }
        if self.calculated_position_cartesian_coor.is_some() {
            fspec[1] |= CalculatedPositionCartesianCorr::FRN_48;
        }
        if self.calculated_track_velocity.is_some() {
            fspec[1] |= CalculatedTrackVelocity::FRN_48;
        }
        if self.track_status.is_some() {
            fspec[1] |= TrackStatus::FRN_48;
        }
        if self.track_quality.is_some() {
            fspec[2] |= TrackQuality::FRN_48;
        }
        if self.warning_error_con_target_class.is_some() {
            fspec[2] |= WarningErrorConditionsTargetClass::FRN_48;
        }
        if self.mode3a_code_confidence_indicator.is_some() {
            fspec[2] |= Mode3ACodeConfidenceIndicator::FRN_48;
        }
        if self.modec_code_and_confidence_indicator.is_some() {
            fspec[2] |= ModeCCodeAndConfidenceIndicator::FRN_48;
        }
        if self.height_measured_by_3d_radar.is_some() {
            fspec[2] |= HeightMeasuredBy3dRadar::FRN_48;
        }
        if self.radial_doppler_speed.is_some() {
            fspec[2] |= RadialDopplerSpeed::FRN_48;
        }
        if self.communications_capability_flight_status.is_some() {
            fspec[2] |= CommunicationsCapabilityFlightStatus::FRN_48;
        }
        if self.acas_resolution_advisory_report.is_some() {
            fspec[3] |= ACASResolutionAdvisoryReport::FRN_48;
        }
        if self.mode_1_code_octal_representation.is_some() {
            fspec[3] |= Mode1CodeOctalRepresentation::FRN_48;
        }
        if self.mode_2_code_octal_representation.is_some() {
            fspec[3] |= Mode2CodeOctalRepresentation::FRN_48;
        }
        trim_fspec(&mut fspec);
        add_fx(&mut fspec);
        self.fspec = fspec;
    }
}
