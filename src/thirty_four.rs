use crate::data_item::{
    AntennaRotationSpeed, CollimationError, DataFilter, DataSourceIdentifier, GenericPolarWindow,
    MessageCountValues, MessageType, SectorNumber, SystemConfigurationAndStatus,
    SystemProcessingMode, ThreeDPositionOfDataSource, TimeOfDay,
};
use crate::fspec::{add_fx, is_fspec, trim_fspec};
use crate::FSPEC_IDENT;
use asterix_derive::UpdateFspec;
use deku::prelude::*;

/// Transmission of Monoradar Service Messages
#[derive(Debug, Default, PartialEq, DekuRead, DekuWrite, UpdateFspec)]
#[deku(endian = "big")]
pub struct Cat34 {
    #[deku(until = "|b: &u8| *b & FSPEC_IDENT == 0")]
    pub fspec: Vec<u8>,
    /// FRN 1
    #[deku(skip, cond = "is_fspec(DataSourceIdentifier::FRN_34, fspec, 0)")]
    pub data_source_identifier: Option<DataSourceIdentifier>,
    /// FRN 2
    #[deku(skip, cond = "is_fspec(MessageType::FRN_34, fspec, 0)")]
    pub message_type: Option<MessageType>,
    /// FRN 3
    #[deku(skip, cond = "is_fspec(TimeOfDay::FRN_34, fspec, 0)")]
    pub time_of_day: Option<TimeOfDay>,
    /// FRN 4
    #[deku(skip, cond = "is_fspec(SectorNumber::FRN_34, fspec, 0)")]
    pub sector_number: Option<SectorNumber>,
    /// FRN 5
    #[deku(skip, cond = "is_fspec(AntennaRotationSpeed::FRN_34, fspec, 0)")]
    pub antenna_rotation_speed: Option<AntennaRotationSpeed>,
    /// FRN 6
    #[deku(skip, cond = "is_fspec(SystemConfigurationAndStatus::FRN_34, fspec, 0)")]
    pub system_configuration_and_status: Option<SystemConfigurationAndStatus>,
    /// FRN 7
    #[deku(skip, cond = "is_fspec(SystemProcessingMode::FRN_34, fspec, 0)")]
    pub system_processing_mode: Option<SystemProcessingMode>,
    /// FRN 8
    #[deku(skip, cond = "is_fspec(MessageCountValues::FRN_34, fspec, 1)")]
    pub message_count_values: Option<MessageCountValues>,
    /// FRN 9
    #[deku(skip, cond = "is_fspec(GenericPolarWindow::FRN_34, fspec, 1)")]
    pub generic_polar_window: Option<GenericPolarWindow>,
    /// FRN 10
    #[deku(skip, cond = "is_fspec(DataFilter::FRN_34, fspec, 1)")]
    pub data_filter: Option<DataFilter>,
    /// FRN 11
    #[deku(skip, cond = "is_fspec(ThreeDPositionOfDataSource::FRN_34, fspec, 1)")]
    pub three_d_position_of_data_source: Option<ThreeDPositionOfDataSource>,
    /// FRN 12
    #[deku(skip, cond = "is_fspec(CollimationError::FRN_34, fspec, 1)")]
    pub collimation_error: Option<CollimationError>,
    // FRN 13: Reserved Expansion Field
    // FRN 14: Special Purpose Field
}
